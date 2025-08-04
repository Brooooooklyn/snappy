#![deny(clippy::all)]

use napi::{bindgen_prelude::*, ScopedTask};
use napi_derive::napi;
use snap::raw::{Decoder, Encoder};

#[cfg(not(target_family = "wasm"))]
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
  type Output = Vec<u8>;
  type JsValue = Either<String, BufferSlice<'env>>;

  fn compute(&mut self) -> Result<Self::Output> {
    self
      .inner
      .decompress_vec(match self.data {
        Either::A(ref s) => s.as_bytes(),
        Either::B(ref b) => b.as_ref(),
      })
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))
  }

  fn resolve(&mut self, env: &'env Env, output: Self::Output) -> Result<Self::JsValue> {
    let opt_ref = self.options.as_ref();
    if opt_ref.and_then(|o| o.as_buffer).unwrap_or(true) {
      if opt_ref.and_then(|o| o.copy_output_data).unwrap_or(false) {
        BufferSlice::copy_from(env, output).map(Either::B)
      } else {
        BufferSlice::from_data(env, output).map(Either::B)
      }
    } else {
      Ok(Either::A(String::from_utf8(output).map_err(|e| {
        Error::new(Status::GenericFailure, format!("{e}"))
      })?))
    }
  }
}

#[napi]
pub fn compress_sync(
  env: Env,
  input: Either<String, &[u8]>,
  options: Option<EncOptions>,
) -> Result<BufferSlice> {
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

#[napi]
pub fn uncompress_sync(
  env: Env,
  input: Either<String, &[u8]>,
  options: Option<DecOptions>,
) -> Result<Either<String, BufferSlice>> {
  let mut dec = Decoder::new();
  dec
    .decompress_vec(match input {
      Either::A(ref s) => s.as_bytes(),
      Either::B(b) => b,
    })
    .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
    .and_then(|output| {
      if options.as_ref().and_then(|o| o.as_buffer).unwrap_or(true) {
        if options
          .as_ref()
          .and_then(|o| o.copy_output_data)
          .unwrap_or(false)
        {
          BufferSlice::copy_from(&env, output).map(Either::B)
        } else {
          BufferSlice::from_data(&env, output).map(Either::B)
        }
      } else {
        Ok(Either::A(String::from_utf8(output).map_err(|e| {
          Error::new(Status::GenericFailure, format!("{e}"))
        })?))
      }
    })
}

#[napi]
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
