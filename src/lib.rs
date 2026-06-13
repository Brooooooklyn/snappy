#![deny(clippy::all)]

use napi::{bindgen_prelude::*, ScopedTask};
use napi_derive::napi;
use snap::raw::{Decoder, Encoder};

#[cfg(all(
  not(target_family = "wasm"),
  not(target_env = "ohos"),
  not(target_env = "musl")
))]
#[global_allocator]
static GLOBAL: mimalloc_safe::MiMalloc = mimalloc_safe::MiMalloc;

#[napi(object)]
pub struct DecOptions {
  pub as_buffer: Option<bool>,
  /// do not use `create_external_buffer` to create the output buffer \n
  /// set this option to `true` will make the API slower \n
  /// for compatibility with electron >= 21 \n
  /// see https://www.electronjs.org/blog/v8-memory-cage and https://github.com/electron/electron/issues/35801#issuecomment-1261206333
  pub copy_output_data: Option<bool>,
  pub output: Option<Uint8Array>,
}

#[napi(object)]
pub struct EncOptions {
  /// do not use `create_external_buffer` to create the output buffer \n
  /// for compatibility with electron >= 21 \n
  /// set this option to `true` will make the API slower \n
  /// see https://www.electronjs.org/blog/v8-memory-cage and https://github.com/electron/electron/issues/35801#issuecomment-1261206333
  pub copy_output_data: Option<bool>,
}

pub struct Enc {
  inner: Encoder,
  data: Either<String, Uint8Array>,
  options: Option<EncOptions>,
}

#[napi]
impl<'env> ScopedTask<'env> for Enc {
  type Output = Vec<u8>;
  type JsValue = BufferSlice<'env>;

  fn compute(&mut self) -> Result<Self::Output> {
    self
      .inner
      .compress_vec(match self.data {
        Either::A(ref b) => b.as_bytes(),
        Either::B(ref s) => s.as_ref(),
      })
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))
  }

  fn resolve(&mut self, env: &'env Env, output: Self::Output) -> Result<Self::JsValue> {
    if self
      .options
      .as_ref()
      .and_then(|o| o.copy_output_data)
      .unwrap_or(false)
    {
      BufferSlice::copy_from(env, output)
    } else {
      BufferSlice::from_data(env, output)
    }
  }
}

pub struct Dec {
  inner: Decoder,
  data: Either<String, Uint8Array>,
  options: Option<DecOptions>,
}

#[napi]
impl<'env> ScopedTask<'env> for Dec {
  type Output = Either<Vec<u8>, u32>;
  type JsValue = Either3<String, BufferSlice<'env>, u32>;

  fn compute(&mut self) -> Result<Self::Output> {
    let input_data = match &self.data {
      Either::A(ref s) => s.as_bytes(),
      Either::B(b) => b.as_ref(),
    };

    if let Some(ref mut opts) = self.options {
      if let Some(ref mut output_buffer) = opts.output {
        let decompressed_len = self
          .inner
          .decompress(input_data, unsafe { output_buffer.as_mut() })
          .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))?;

        return Ok(Either::B(decompressed_len as u32));
      }
    }
    self
      .inner
      .decompress_vec(input_data)
      .map(Either::A)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))
  }

  fn resolve(&mut self, env: &'env Env, output: Self::Output) -> Result<Self::JsValue> {
    match output {
      Either::B(length) => Ok(Either3::C(length)),
      Either::A(output) => {
        let opt_ref = self.options.as_ref();
        if opt_ref.and_then(|o| o.as_buffer).unwrap_or(true) {
          if opt_ref.and_then(|o| o.copy_output_data).unwrap_or(false) {
            BufferSlice::copy_from(env, output).map(Either3::B)
          } else {
            BufferSlice::from_data(env, output).map(Either3::B)
          }
        } else {
          Ok(Either3::A(String::from_utf8(output).map_err(|e| {
            Error::new(Status::GenericFailure, format!("{e}"))
          })?))
        }
      }
    }
  }
}

#[napi]
pub fn compress_sync<'env>(
  env: Env,
  input: Either<String, &'env [u8]>,
  options: Option<EncOptions>,
) -> Result<BufferSlice<'env>> {
  let mut enc = Encoder::new();
  enc
    .compress_vec(match input {
      Either::A(ref s) => s.as_bytes(),
      Either::B(b) => b,
    })
    .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
    .and_then(|output| {
      if options
        .as_ref()
        .and_then(|o| o.copy_output_data)
        .unwrap_or(false)
      {
        BufferSlice::copy_from(&env, output)
      } else {
        BufferSlice::from_data(&env, output)
      }
    })
}

#[napi]
pub fn compress(
  input: Either<String, Uint8Array>,
  options: Option<EncOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<Enc> {
  let enc = Encoder::new();
  let encoder = Enc {
    inner: enc,
    data: input,
    options,
  };
  AsyncTask::with_optional_signal(encoder, signal)
}

#[napi(ts_return_type = r#"Buffer
export declare function uncompressSync(input: string | Uint8Array, options: { asBuffer: false }): string;
export declare function uncompressSync(input: string | Uint8Array, options: { output: Uint8Array }): number;
export declare function uncompressSync(input: string | Uint8Array, options?: { asBuffer?: true }): Buffer;
export declare function uncompressSync(input: string | Uint8Array, options?: DecOptions): string | Buffer | number;
"#)]
pub fn uncompress_sync<'env>(
  env: &'env Env,
  #[napi(ts_arg_type = "undefined")] input: Either<String, &'env [u8]>,
  options: Option<DecOptions>,
) -> Result<Either3<String, BufferSlice<'env>, u32>> {
  let mut dec = Decoder::new();
  let input_data = match input {
    Either::A(ref s) => s.as_bytes(),
    Either::B(b) => b,
  };

  let as_buffer = options.as_ref().and_then(|o| o.as_buffer).unwrap_or(true);
  let copy_output_data = options
    .as_ref()
    .and_then(|o| o.copy_output_data)
    .unwrap_or(false);

  if let Some(mut opts) = options {
    if let Some(ref mut output_buffer) = opts.output {
      let decompressed_len = dec
        .decompress(input_data, unsafe { output_buffer.as_mut() })
        .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))?;

      return Ok(Either3::C(decompressed_len as u32));
    }
  }
  dec
    .decompress_vec(input_data)
    .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
    .and_then(|output| {
      if as_buffer {
        if copy_output_data {
          BufferSlice::copy_from(env, output).map(Either3::B)
        } else {
          BufferSlice::from_data(env, output).map(Either3::B)
        }
      } else {
        Ok(Either3::A(String::from_utf8(output).map_err(|e| {
          Error::new(Status::GenericFailure, format!("{e}"))
        })?))
      }
    })
}

#[napi(ts_return_type = r#"Promise<Buffer>
export declare function uncompress(input: string | Uint8Array, options: { asBuffer: false }): Promise<string>;
export declare function uncompress(input: string | Uint8Array, options: { output: Uint8Array }): Promise<number>;
export declare function uncompress(input: string | Uint8Array, options?: { asBuffer?: true }): Promise<Buffer>;
export declare function uncompress(input: string | Uint8Array, options?: DecOptions): Promise<string | Buffer | number>;
"#)]
pub fn uncompress(
  input: Either<String, Uint8Array>,
  options: Option<DecOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<Dec> {
  let dec = Decoder::new();
  let decoder = Dec {
    inner: dec,
    data: input,
    options,
  };
  AsyncTask::with_optional_signal(decoder, signal)
}
