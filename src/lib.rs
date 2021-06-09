#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{CallContext, Error, JsBuffer, JsObject, Result};
use snap::raw::Encoder;

#[cfg(all(
  unix,
  not(target_env = "musl"),
  not(target_arch = "aarch64"),
  not(target_arch = "arm"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64", not(debug_assertions)))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("encode", encode)?;
  Ok(())
}

#[js_function(1)]
fn encode(ctx: CallContext) -> Result<JsBuffer> {
  let data = ctx.get::<JsBuffer>(0)?;
  let this = ctx.this_unchecked::<JsObject>();
  let enc = ctx.env.unwrap::<Encoder>(&this)?;
  enc
    .compress_vec(&data.into_value()?)
    .map_err(|e| Error::new(napi::Status::GenericFailure, format!("{}", e)))
    .and_then(|d| ctx.env.create_buffer_with_data(d))
    .map(|b| b.into_raw())
}
