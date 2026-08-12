//! Incremental (streaming) `#[napi]` compressor / decompressor classes for the
//! **Snappy frame format** (`.sz`, MIME `application/x-snappy-framed`).
//!
//! This is a different wire format from the one-shot `compress` / `uncompress`
//! APIs, which use the raw Snappy block format via `snap::raw`. Streaming uses
//! `snap::write::FrameEncoder` / `snap::read::FrameDecoder`. The two formats
//! do **not** interoperate: framed output cannot be fed to `uncompress()`, and
//! raw blocks cannot be fed to [`Decompressor`].
//!
//! ## Class API (matches `@napi-rs/lzma`)
//!
//! * [`Compressor::update`] is **synchronous**. It writes the chunk into the
//!   encoder and drains whatever framed bytes have been produced so far
//!   (possibly none). It MUST NEVER flush the encoder — a flush forces a chunk
//!   boundary and would make the output depend on how the input was split.
//!   Meaningful output is only the concatenation of every `update()` plus the
//!   `finish()` tail.
//! * `finish()` returns a `Promise<Buffer>`. It moves the owned encoder onto
//!   the libuv pool via [`AsyncTask`] (NOT a tokio `async fn`, which would pull
//!   `napi/tokio_rt`; the class API must build for every target — wasm included
//!   — on default napi features), flushes remaining buffered plaintext, and
//!   returns the tail. A double-finish is guarded by `Option::take`.
//!
//! Decompression is pull-based (`impl io::Read`), so [`Decompressor`] runs a
//! worker thread with a channel adapter — same deadlock-free design as lzma.

use std::io::{self, Read, Write};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::thread::JoinHandle;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use snap::read::FrameDecoder;
use snap::write::FrameEncoder;

/// Maps a generic I/O or encode error to a napi error.
fn map_io(err: io::Error) -> napi::Error {
  napi::Error::from_reason(err.to_string())
}

/// A `Send`, heap-only sink the streaming encoder drains incrementally.
///
/// Plain `Vec<u8>` — deliberately NO `Rc`/`RefCell`/`Cell`: `finish()` moves the
/// encoder (and therefore this sink) onto a worker thread, so every field must
/// be `Send`. `update()` drains produced bytes with `std::mem::take`.
#[derive(Default)]
pub struct SharedSink(pub Vec<u8>);

impl Write for SharedSink {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.0.extend_from_slice(buf);
    Ok(buf.len())
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

fn already_finished() -> napi::Error {
  napi::Error::new(
    napi::Status::InvalidArg,
    "compressor already finished".to_owned(),
  )
}

/// Off-thread finish: flush the encoder on the libuv pool.
pub struct CompressorFinish(Option<FrameEncoder<SharedSink>>);

#[napi]
impl Task for CompressorFinish {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let encoder = self.0.take().ok_or_else(already_finished)?;
    // `into_inner` flushes remaining buffered plaintext into frames, then
    // returns the underlying sink. `FrameEncoder`'s Drop also flushes, so take
    // ownership here before drop.
    match encoder.into_inner() {
      Ok(sink) => Ok(sink.0),
      Err(err) => {
        // Forget the encoder so Drop does not re-flush after a failed flush;
        // surface the I/O error instead.
        let io_err = err.error().to_string();
        let enc = err.into_inner();
        std::mem::forget(enc);
        Err(napi::Error::from_reason(io_err))
      }
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

/// Incremental Snappy **frame-format** compressor.
///
/// Feed plaintext with [`update`](Compressor::update); call
/// [`finish`](Compressor::finish) for the trailing framed bytes. The valid
/// stream is the concatenation of every `update()` output plus the `finish()`
/// tail. This is **not** the same format as one-shot [`crate::compress`].
#[napi]
pub struct Compressor {
  /// `None` once `finish()` has consumed the encoder (double-finish guard).
  inner: Option<FrameEncoder<SharedSink>>,
}

impl Default for Compressor {
  fn default() -> Self {
    Self::new()
  }
}

#[napi]
impl Compressor {
  /// Create a streaming framed-Snappy compressor.
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      inner: Some(FrameEncoder::new(SharedSink::default())),
    }
  }

  /// Feed one chunk. A `string` is UTF-8 encoded (matching the one-shot
  /// `compress` convention); a `Uint8Array` is fed verbatim. Returns the
  /// framed bytes produced so far (possibly empty). Never flushes the encoder
  /// (byte-identity invariant across chunkings).
  #[napi]
  pub fn update<'env>(
    &mut self,
    env: &'env Env,
    chunk: Either<String, Uint8Array>,
  ) -> Result<BufferSlice<'env>> {
    let writer = self.inner.as_mut().ok_or_else(already_finished)?;
    let bytes: &[u8] = match &chunk {
      Either::A(text) => text.as_bytes(),
      Either::B(buf) => buf.as_ref(),
    };
    writer.write_all(bytes).map_err(map_io)?;
    let produced = std::mem::take(&mut writer.get_mut().0);
    BufferSlice::from_data(env, produced)
  }

  /// Flush remaining buffered plaintext into frames off the JS thread.
  /// Resolves to the tail bytes. Idempotency-guarded: a second call rejects.
  #[napi]
  pub fn finish(&mut self) -> Result<AsyncTask<CompressorFinish>> {
    let writer = self.inner.take().ok_or_else(already_finished)?;
    Ok(AsyncTask::new(CompressorFinish(Some(writer))))
  }
}

// ===========================================================================
// Streaming decompressor
// ===========================================================================
//
// Channel-direction asymmetry (the crux of deadlock-freedom), copied from lzma:
//
// * The OUT channel (worker -> JS) is BOUNDED. The worker blocks on `out_tx.send`
//   once full, so a decompression bomb cannot run unbounded ahead of JS.
// * The IN channel (JS -> worker) is UNBOUNDED, and `update()` hands off with a
//   NON-BLOCKING send. `update()` is synchronous on the main JS thread and must
//   NEVER block.

/// Bound (in messages) of the decoded-OUT channel.
const OUT_CHANNEL_BOUND: usize = 8;

type WorkerMsg = std::result::Result<Vec<u8>, String>;

/// Blocking, buffer-FILLING [`io::Read`] adapter that pulls compressed chunks
/// off the input channel. Returns a short read ONLY at true EOF (sender dropped).
struct ChannelReader {
  rx: Receiver<Vec<u8>>,
  cur: Vec<u8>,
  pos: usize,
}

impl ChannelReader {
  fn new(rx: Receiver<Vec<u8>>) -> Self {
    Self {
      rx,
      cur: Vec::new(),
      pos: 0,
    }
  }
}

impl Read for ChannelReader {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    let mut written = 0;
    while written < buf.len() {
      if self.pos >= self.cur.len() {
        match self.rx.recv() {
          Ok(chunk) => {
            self.cur = chunk;
            self.pos = 0;
            continue;
          }
          Err(_) => break,
        }
      }
      let n = std::cmp::min(buf.len() - written, self.cur.len() - self.pos);
      buf[written..written + n].copy_from_slice(&self.cur[self.pos..self.pos + n]);
      self.pos += n;
      written += n;
    }
    Ok(written)
  }
}

fn pump_reader<R: Read>(mut reader: R, out_tx: &SyncSender<WorkerMsg>) {
  let mut buf = [0u8; 64 * 1024];
  loop {
    match reader.read(&mut buf) {
      Ok(0) => break,
      Ok(n) => {
        if out_tx.send(Ok(buf[..n].to_vec())).is_err() {
          break;
        }
      }
      Err(e) => {
        let _ = out_tx.send(Err(e.to_string()));
        break;
      }
    }
  }
}

fn decode_error(reason: String) -> napi::Error {
  napi::Error::new(napi::Status::InvalidArg, reason)
}

fn already_finished_decompressor() -> napi::Error {
  napi::Error::new(
    napi::Status::InvalidArg,
    "decompressor already finished".to_owned(),
  )
}

struct DecompressorState {
  in_tx: Sender<Vec<u8>>,
  out_rx: Receiver<WorkerMsg>,
  failed: Option<String>,
  worker: Option<JoinHandle<()>>,
}

impl DecompressorState {
  fn spawn() -> Result<Self> {
    let (in_tx, in_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (out_tx, out_rx) = std::sync::mpsc::sync_channel::<WorkerMsg>(OUT_CHANNEL_BOUND);
    let worker = std::thread::Builder::new()
      .name("snappy-decompressor".to_owned())
      .spawn(move || {
        let reader = FrameDecoder::new(ChannelReader::new(in_rx));
        pump_reader(reader, &out_tx);
      })
      .map_err(|e| {
        napi::Error::from_reason(format!(
          "failed to spawn snappy-decompressor worker thread: {e}"
        ))
      })?;
    Ok(Self {
      in_tx,
      out_rx,
      failed: None,
      worker: Some(worker),
    })
  }

  fn drain_available(&mut self, out: &mut Vec<u8>) -> Result<()> {
    while let Ok(msg) = self.out_rx.try_recv() {
      match msg {
        Ok(bytes) => out.extend(bytes),
        Err(reason) => {
          self.failed = Some(reason.clone());
          return Err(decode_error(reason));
        }
      }
    }
    Ok(())
  }

  fn update_bytes(&mut self, chunk: &[u8]) -> Result<Vec<u8>> {
    if let Some(reason) = &self.failed {
      return Err(decode_error(reason.clone()));
    }
    let mut out = Vec::new();
    self.drain_available(&mut out)?;
    let _ = self.in_tx.send(chunk.to_vec());
    self.drain_available(&mut out)?;
    Ok(out)
  }

  fn into_finish(mut self) -> Result<Vec<u8>> {
    if let Some(reason) = self.failed.take() {
      drop(self.in_tx);
      if let Some(handle) = self.worker.take() {
        let _ = handle.join();
      }
      return Err(decode_error(reason));
    }
    drop(self.in_tx);
    let mut out = Vec::new();
    let mut decode_err = None;
    while let Ok(msg) = self.out_rx.recv() {
      match msg {
        Ok(bytes) => out.extend(bytes),
        Err(reason) => {
          decode_err = Some(decode_error(reason));
          break;
        }
      }
    }
    if let Some(handle) = self.worker.take() {
      if handle.join().is_err() {
        return Err(napi::Error::from_reason(
          "decompressor worker thread panicked".to_owned(),
        ));
      }
    }
    match decode_err {
      Some(e) => Err(e),
      None => Ok(out),
    }
  }
}

pub struct DecompressorFinish(Option<DecompressorState>);

#[napi]
impl Task for DecompressorFinish {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let state = self.0.take().ok_or_else(already_finished_decompressor)?;
    state.into_finish()
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

/// Incremental Snappy **frame-format** decompressor.
///
/// Feed framed compressed chunks with [`update`](Decompressor::update); call
/// [`finish`](Decompressor::finish) for the decoded tail. Not compatible with
/// one-shot raw [`crate::compress`] output.
#[napi]
pub struct Decompressor {
  inner: Option<DecompressorState>,
}

#[napi]
impl Decompressor {
  /// Create a streaming framed-Snappy decompressor and start its worker thread.
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Self {
      inner: Some(DecompressorState::spawn()?),
    })
  }

  /// Feed one compressed chunk; returns the bytes decoded so far (possibly
  /// empty). Deadlock-free under backpressure.
  #[napi]
  pub fn update<'env>(&mut self, env: &'env Env, chunk: Uint8Array) -> Result<BufferSlice<'env>> {
    let state = self
      .inner
      .as_mut()
      .ok_or_else(already_finished_decompressor)?;
    let produced = state.update_bytes(chunk.as_ref())?;
    BufferSlice::from_data(env, produced)
  }

  /// Signal EOF and resolve to the decoded tail off the JS thread.
  /// Idempotency-guarded: a second call rejects cleanly.
  #[napi]
  pub fn finish(&mut self) -> Result<AsyncTask<DecompressorFinish>> {
    let state = self
      .inner
      .take()
      .ok_or_else(already_finished_decompressor)?;
    Ok(AsyncTask::new(DecompressorFinish(Some(state))))
  }
}
