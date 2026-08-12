//! Web Streams API (native, tokio-backed) transforms for framed Snappy.
//!
//! `compress_stream` / `uncompress_stream` accept a JS `ReadableStream<Uint8Array>`
//! and return a `ReadableStream<Buffer>`. Target-gated to non-wasm (`lib.rs`);
//! the wasm build falls back to a buffered class-API polyfill in JS.
//!
//! Same three-task pipeline as `@napi-rs/lzma` `stream_web.rs`.

use std::future::{poll_fn, Future};
use std::io::{self, Read, Write};
use std::pin::pin;
use std::task::Poll;

use napi::bindgen_prelude::*;
use napi::tokio::sync::mpsc::{channel, Receiver, Sender};
use napi::tokio_stream::wrappers::ReceiverStream;
use napi::tokio_stream::StreamExt;
use napi_derive::napi;
use snap::read::FrameDecoder;
use snap::write::FrameEncoder;

/// Bound (in messages) of BOTH channels.
const CHANNEL_CAP: usize = 16;

type Chunk = Result<Vec<u8>>;

fn map_io(err: io::Error) -> napi::Error {
  napi::Error::from_reason(err.to_string())
}

fn map_invalid(err: io::Error) -> napi::Error {
  napi::Error::new(napi::Status::InvalidArg, err.to_string())
}

struct ChannelReader {
  rx: Receiver<Chunk>,
  cur: Vec<u8>,
  pos: usize,
}

impl ChannelReader {
  fn new(rx: Receiver<Chunk>) -> Self {
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
        match self.rx.blocking_recv() {
          Some(Ok(chunk)) => {
            self.cur = chunk;
            self.pos = 0;
            continue;
          }
          Some(Err(err)) => return Err(io::Error::other(err.reason)),
          None => break,
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

const WRITE_BUFFER: usize = 64 * 1024;

struct ChannelWriter {
  tx: Sender<Chunk>,
  buf: Vec<u8>,
}

impl ChannelWriter {
  fn new(tx: Sender<Chunk>) -> Self {
    Self {
      tx,
      buf: Vec::with_capacity(WRITE_BUFFER),
    }
  }

  fn send_buffer(&mut self) -> io::Result<()> {
    if self.buf.is_empty() {
      return Ok(());
    }
    let chunk = std::mem::take(&mut self.buf);
    match self.tx.blocking_send(Ok(chunk)) {
      Ok(()) => Ok(()),
      Err(_) => Err(io::Error::other("output stream closed")),
    }
  }
}

impl Write for ChannelWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.buf.extend_from_slice(buf);
    if self.buf.len() >= WRITE_BUFFER {
      self.send_buffer()?;
    }
    Ok(buf.len())
  }

  fn flush(&mut self) -> io::Result<()> {
    self.send_buffer()
  }
}

fn run_compress(mut in_rx: Receiver<Chunk>, out_tx: Sender<Chunk>) {
  let err_tx = out_tx.clone();
  let outcome: Result<()> = (|| {
    let mut writer = FrameEncoder::new(ChannelWriter::new(out_tx));
    loop {
      match in_rx.blocking_recv() {
        Some(Ok(chunk)) => writer.write_all(&chunk).map_err(map_io)?,
        Some(Err(err)) => return Err(err),
        None => break,
      }
    }
    // Flush remaining frames into the channel sink, then flush the coalescer.
    match writer.into_inner() {
      Ok(mut sink) => sink.flush().map_err(map_io),
      Err(err) => {
        let reason = err.error().to_string();
        std::mem::forget(err.into_inner());
        Err(napi::Error::from_reason(reason))
      }
    }
  })();
  if let Err(err) = outcome {
    let _ = err_tx.blocking_send(Err(err));
  }
}

fn run_decompress(in_rx: Receiver<Chunk>, out_tx: Sender<Chunk>) {
  let err_tx = out_tx.clone();
  let outcome: Result<()> = (|| {
    let mut reader = FrameDecoder::new(ChannelReader::new(in_rx));
    let mut buf = [0u8; 64 * 1024];
    loop {
      let n = reader.read(&mut buf).map_err(map_invalid)?;
      if n == 0 {
        break;
      }
      if out_tx.blocking_send(Ok(buf[..n].to_vec())).is_err() {
        return Ok(());
      }
    }
    Ok(())
  })();
  if let Err(err) = outcome {
    let _ = err_tx.blocking_send(Err(err));
  }
}

fn spawn_pipeline<'env, F>(
  env: &'env Env,
  input: ReadableStream<Uint8Array>,
  worker: F,
) -> Result<ReadableStream<'env, BufferSlice<'env>>>
where
  F: FnOnce(Receiver<Chunk>, Sender<Chunk>) + Send + 'static,
{
  let mut reader = input.read()?;
  let (in_tx, in_rx) = channel::<Chunk>(CHANNEL_CAP);
  let (out_tx, out_rx) = channel::<Chunk>(CHANNEL_CAP);

  let pump_out = out_tx.clone();
  spawn(async move {
    loop {
      let mut next = pin!(reader.next());
      let mut closed = pin!(pump_out.closed());
      let event = poll_fn(|cx| {
        if closed.as_mut().poll(cx).is_ready() {
          return Poll::Ready(None);
        }
        match next.as_mut().poll(cx) {
          Poll::Ready(item) => Poll::Ready(Some(item)),
          Poll::Pending => Poll::Pending,
        }
      })
      .await;
      match event {
        None | Some(None) => break,
        Some(Some(item)) => {
          if in_tx.send(item.map(|bytes| bytes.to_vec())).await.is_err() {
            break;
          }
        }
      }
    }
  });

  spawn_blocking(move || worker(in_rx, out_tx));

  ReadableStream::create_with_stream_bytes(env, ReceiverStream::new(out_rx))
}

/// Compress a `ReadableStream<Uint8Array>` into a framed Snappy byte stream.
///
/// `input` must be a WHATWG `ReadableStream`; wrap a Node `Readable` with
/// `Readable.toWeb()`, or use `createCompressStream()` for a ready-to-pipe
/// Node `Duplex`. Output is **framed** Snappy (not the raw one-shot format).
#[napi]
pub fn compress_stream<'env>(
  env: &'env Env,
  input: ReadableStream<Uint8Array>,
) -> Result<ReadableStream<'env, BufferSlice<'env>>> {
  spawn_pipeline(env, input, run_compress)
}

/// Decompress a framed Snappy `ReadableStream<Uint8Array>` into plaintext.
///
/// `input` must be a WHATWG `ReadableStream`; wrap a Node `Readable` with
/// `Readable.toWeb()`, or use `createUncompressStream()` for a ready-to-pipe
/// Node `Duplex`. Input must be **framed** Snappy (not one-shot raw blocks).
#[napi]
pub fn uncompress_stream<'env>(
  env: &'env Env,
  input: ReadableStream<Uint8Array>,
) -> Result<ReadableStream<'env, BufferSlice<'env>>> {
  spawn_pipeline(env, input, run_decompress)
}
