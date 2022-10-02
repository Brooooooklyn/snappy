#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::*, JsBuffer, JsBufferValue, Ref};
use snap::raw::{Decoder, Encoder};

#[cfg(all(
  not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

pub enum Data {
  Buffer(Ref<JsBufferValue>),
  String(String),
}

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

impl TryFrom<Either<String, JsBuffer>> for Data {
  type Error = Error;

  fn try_from(value: Either<String, JsBuffer>) -> Result<Self> {
    match value {
      Either::A(s) => Ok(Data::String(s)),
      Either::B(b) => Ok(Data::Buffer(b.into_ref()?)),
    }
  }
}

pub struct Enc {
  inner: Encoder,
  data: Data,
  options: Option<EncOptions>,
}

#[napi]
impl Task for Enc {
  type Output = Vec<u8>;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    self
      .inner
      .compress_vec(match self.data {
        Data::Buffer(ref b) => b.as_ref(),
        Data::String(ref s) => s.as_bytes(),
      })
      .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    if self
      .options
      .as_ref()
      .and_then(|o| o.copy_output_data)
      .unwrap_or(false)
    {
      env.create_buffer_copy(output)
    } else {
      env.create_buffer_with_data(output)
    }
    .map(|b| b.into_raw())
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    if let Data::Buffer(b) = &mut self.data {
      b.unref(env)?;
    }
    Ok(())
  }
}

pub struct Dec {
  inner: Decoder,
  data: Data,
  options: Option<DecOptions>,
}

#[napi]
impl Task for Dec {
  type Output = Vec<u8>;
  type JsValue = Either<String, JsBuffer>;

  fn compute(&mut self) -> Result<Self::Output> {
    self
      .inner
      .decompress_vec(match self.data {
        Data::Buffer(ref b) => b.as_ref(),
        Data::String(ref s) => s.as_bytes(),
      })
      .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    let opt_ref = self.options.as_ref();
    if opt_ref.and_then(|o| o.as_buffer).unwrap_or(true) {
      if opt_ref.and_then(|o| o.copy_output_data).unwrap_or(false) {
        Ok(Either::B(
          env.create_buffer_copy(output).map(|o| o.into_raw())?,
        ))
      } else {
        Ok(Either::B(
          env.create_buffer_with_data(output).map(|o| o.into_raw())?,
        ))
      }
    } else {
      Ok(Either::A(String::from_utf8(output).map_err(|e| {
        Error::new(Status::GenericFailure, format!("{}", e))
      })?))
    }
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    if let Data::Buffer(b) = &mut self.data {
      b.unref(env)?;
    }
    Ok(())
  }
}

#[napi]
pub fn compress_sync(
  env: Env,
  input: Either<String, JsBuffer>,
  options: Option<EncOptions>,
) -> Result<JsBuffer> {
  let enc = Encoder::new();
  let mut encoder = Enc {
    inner: enc,
    data: Data::try_from(input)?,
    options,
  };
  let output = encoder.compute()?;
  encoder.resolve(env, output)
}

#[napi]
pub fn compress(
  input: Either<String, JsBuffer>,
  options: Option<EncOptions>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<Enc>> {
  let enc = Encoder::new();
  let encoder = Enc {
    inner: enc,
    data: Data::try_from(input)?,
    options,
  };
  match signal {
    Some(s) => Ok(AsyncTask::with_signal(encoder, s)),
    None => Ok(AsyncTask::new(encoder)),
  }
}

#[napi]
pub fn uncompress_sync(
  env: Env,
  input: Either<String, JsBuffer>,
  options: Option<DecOptions>,
) -> Result<Either<String, JsBuffer>> {
  let dec = Decoder::new();
  let mut decoder = Dec {
    inner: dec,
    data: Data::try_from(input)?,
    options,
  };
  let output = decoder.compute()?;
  decoder.resolve(env, output)
}

#[napi]
pub fn uncompress(
  input: Either<String, JsBuffer>,
  options: Option<DecOptions>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<Dec>> {
  let dec = Decoder::new();
  let decoder = Dec {
    inner: dec,
    data: Data::try_from(input)?,
    options,
  };
  Ok(AsyncTask::with_optional_signal(decoder, signal))
}
