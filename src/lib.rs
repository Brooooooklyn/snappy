#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::ffi::CString;

use napi::{
  CallContext, Env, Error, JsBoolean, JsBuffer, JsObject, JsUnknown, Result, Status, Task,
};
use snap::raw::{Decoder, Encoder};

#[cfg(all(
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
  data: Vec<u8>,
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
  data: Vec<u8>,
  as_buffer: bool,
}

impl Task for Dec {
  type Output = Vec<u8>;
  type JsValue = JsUnknown;

  fn compute(&mut self) -> Result<Self::Output> {
    let data_ref: &[u8] = &self.data;
    self
      .inner
      .decompress_vec(data_ref)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    if self.as_buffer {
      env
        .create_buffer_with_data(output)
        .map(|b| b.into_raw().into_unknown())
    } else {
      let len = output.len();
      let c_string = CString::new(output)?;
      unsafe { env.create_string_from_c_char(c_string.as_ptr(), len) }.map(|v| v.into_unknown())
    }
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
    data: data.into_value()?.to_vec(),
  };
  ctx.env.spawn(encoder).map(|v| v.promise_object())
}

#[js_function(2)]
fn uncompress_sync(ctx: CallContext) -> Result<JsUnknown> {
  let data = ctx.get::<JsBuffer>(0)?;
  let as_buffer = ctx.get::<JsBoolean>(1)?.get_value()?;
  let mut dec = Decoder::new();
  dec
    .decompress_vec(&data.into_value()?)
    .map_err(|e| Error::new(napi::Status::GenericFailure, format!("{}", e)))
    .and_then(|d| {
      if as_buffer {
        ctx
          .env
          .create_buffer_with_data(d)
          .map(|b| b.into_raw().into_unknown())
      } else {
        let len = d.len();
        let c_string = CString::new(d)?;
        unsafe { ctx.env.create_string_from_c_char(c_string.as_ptr(), len) }
          .map(|v| v.into_unknown())
      }
    })
}

#[js_function(2)]
fn uncompress(ctx: CallContext) -> Result<JsObject> {
  let data = ctx.get::<JsBuffer>(0)?;
  let as_buffer = ctx.get::<JsBoolean>(1)?.get_value()?;
  let dec = Decoder::new();
  let decoder = Dec {
    inner: dec,
    data: data.into_value()?.to_vec(),
    as_buffer,
  };
  ctx.env.spawn(decoder).map(|v| v.promise_object())
}
