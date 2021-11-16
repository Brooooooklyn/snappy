#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::*, JsBuffer, JsBufferValue, Ref};
use snap::raw::{Decoder, Encoder};

#[cfg(all(
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub enum Data {
  Buffer(Ref<JsBufferValue>),
  String(String),
}

#[napi(object)]
pub struct Options {
  pub as_buffer: Option<bool>,
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

struct Enc {
  inner: Encoder,
  data: Data,
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
    env.create_buffer_with_data(output).map(|b| b.into_raw())
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    if let Data::Buffer(b) = &mut self.data {
      b.unref(env)?;
    }
    Ok(())
  }
}

struct Dec {
  inner: Decoder,
  data: Data,
  as_buffer: bool,
}

#[napi]
impl Task for Dec {
  type Output = Vec<u8>;
  type JsValue = Either<String, Buffer>;

  fn compute(&mut self) -> Result<Self::Output> {
    self
      .inner
      .decompress_vec(match self.data {
        Data::Buffer(ref b) => b.as_ref(),
        Data::String(ref s) => s.as_bytes(),
      })
      .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    if self.as_buffer {
      Ok(Either::B(output.into()))
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
pub fn compress_sync(input: Either<Buffer, String>) -> Result<Buffer> {
  let mut enc = Encoder::new();
  enc
    .compress_vec(match input {
      Either::A(ref b) => b.as_ref(),
      Either::B(ref s) => s.as_bytes(),
    })
    .map_err(|e| Error::new(napi::Status::GenericFailure, format!("{}", e)))
    .map(|v| v.into())
}

#[napi]
fn compress(
  input: Either<String, JsBuffer>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<Enc>> {
  let enc = Encoder::new();
  let encoder = Enc {
    inner: enc,
    data: Data::try_from(input)?,
  };
  match signal {
    Some(s) => Ok(AsyncTask::with_signal(encoder, s)),
    None => Ok(AsyncTask::new(encoder)),
  }
}

#[napi]
fn uncompress_sync(
  input: Either<String, Buffer>,
  as_buffer: Option<Options>,
) -> Result<Either<String, Buffer>> {
  let as_buffer = as_buffer.and_then(|o| o.as_buffer).unwrap_or(true);
  let mut dec = Decoder::new();
  dec
    .decompress_vec(match input {
      Either::A(ref s) => s.as_bytes(),
      Either::B(ref b) => b.as_ref(),
    })
    .map_err(|e| Error::new(napi::Status::GenericFailure, format!("{}", e)))
    .and_then(|d| {
      if as_buffer {
        Ok(Either::B(d.into()))
      } else {
        Ok(Either::A(String::from_utf8(d).map_err(|e| {
          Error::new(Status::GenericFailure, format!("{}", e))
        })?))
      }
    })
}

#[napi]
fn uncompress(
  input: Either<String, JsBuffer>,
  options: Option<Options>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<Dec>> {
  let as_buffer = options.and_then(|o| o.as_buffer).unwrap_or(true);
  let dec = Decoder::new();
  let decoder = Dec {
    inner: dec,
    data: Data::try_from(input)?,
    as_buffer,
  };
  match signal {
    Some(s) => Ok(AsyncTask::with_signal(decoder, s)),
    None => Ok(AsyncTask::new(decoder)),
  }
}
