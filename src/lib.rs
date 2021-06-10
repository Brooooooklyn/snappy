#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{CallContext, Env, Error, JsBuffer, JsBufferValue, JsObject, Ref, Result, Status, Task};
use snap::raw::{Decoder, Encoder};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("compressSync", compress_sync)?;
  exports.create_named_method("compress", compress)?;
  exports.create_named_method("uncompressSync", uncompress_sync)?;
  exports.create_named_method("uncompress", uncompress)?;
  Ok(())
}

struct Enc {
  inner: Encoder,
  data: Ref<JsBufferValue>,
}

impl Task for Enc {
  type Output = Vec<u8>;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let data_ref: &[u8] = &self.data;
    self
      .inner
      .compress_vec(data_ref)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_buffer_with_data(output).map(|b| b.into_raw())
  }
}

struct Dec {
  inner: Decoder,
  data: Ref<JsBufferValue>,
}

impl Task for Dec {
  type Output = Vec<u8>;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let data_ref: &[u8] = &self.data;
    self
      .inner
      .decompress_vec(data_ref)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_buffer_with_data(output).map(|b| b.into_raw())
  }
}

#[js_function(1)]
fn compress_sync(ctx: CallContext) -> Result<JsBuffer> {
  let data = ctx.get::<JsBuffer>(0)?;
  let mut enc = Encoder::new();
  enc
    .compress_vec(&data.into_value()?)
    .map_err(|e| Error::new(napi::Status::GenericFailure, format!("{}", e)))
    .and_then(|d| ctx.env.create_buffer_with_data(d))
    .map(|b| b.into_raw())
}

#[js_function(1)]
fn compress(ctx: CallContext) -> Result<JsObject> {
  let data = ctx.get::<JsBuffer>(0)?;
  let enc = Encoder::new();
  let encoder = Enc {
    inner: enc,
    data: data.into_ref()?,
  };
  ctx.env.spawn(encoder).map(|v| v.promise_object())
}

#[js_function(1)]
fn uncompress_sync(ctx: CallContext) -> Result<JsBuffer> {
  let data = ctx.get::<JsBuffer>(0)?;
  let mut dec = Decoder::new();
  dec
    .decompress_vec(&data.into_value()?)
    .map_err(|e| Error::new(napi::Status::GenericFailure, format!("{}", e)))
    .and_then(|d| ctx.env.create_buffer_with_data(d))
    .map(|b| b.into_raw())
}

#[js_function(1)]
fn uncompress(ctx: CallContext) -> Result<JsObject> {
  let data = ctx.get::<JsBuffer>(0)?;
  let dec = Decoder::new();
  let decoder = Dec {
    inner: dec,
    data: data.into_ref()?,
  };
  ctx.env.spawn(decoder).map(|v| v.promise_object())
}
