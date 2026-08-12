/**
 * Shared streaming-test helpers for framed Snappy.
 */
import { createRequire } from 'node:module'

const requireFrom = createRequire(import.meta.url)

/** True when the suite runs against the WASI-forced binding (`NAPI_RS_FORCE_WASI`). */
export const IS_WASI = !!process.env.NAPI_RS_FORCE_WASI

export const IS_SLOW_EMULATED_ARCH = ['s390x', 'ppc64', 'ppc64le'].includes(process.arch)

export const MAX_EMULATED_FIXTURE_BYTES = 4 * 1024 * 1024

export const runsFixtureOfSize = (byteLength: number): boolean =>
  !IS_SLOW_EMULATED_ARCH || byteLength <= MAX_EMULATED_FIXTURE_BYTES

export interface CompressorInstance {
  update(chunk: string | Uint8Array): Buffer
  finish(): Promise<Buffer>
}

export interface DecompressorInstance {
  update(chunk: Uint8Array): Buffer
  finish(): Promise<Buffer>
}

export function loadBinding() {
  // Prefer the honest package entry (main.js) so stream factories resolve.
  return requireFrom('..') as typeof import('..')
}

export function chunkBySize(buf: Buffer, size: number): Uint8Array[] {
  if (buf.length === 0) {
    return []
  }
  const chunks: Uint8Array[] = []
  for (let i = 0; i < buf.length; i += size) {
    chunks.push(buf.subarray(i, Math.min(i + size, buf.length)))
  }
  return chunks
}

export function chunkByByte(buf: Buffer): Uint8Array[] {
  return chunkBySize(buf, 1)
}

/** Drive class compressor over chunks; return full framed stream. */
export async function driveClassCompress(chunks: Array<string | Uint8Array>): Promise<Buffer> {
  const { Compressor } = loadBinding()
  const compressor = new Compressor()
  const parts: Buffer[] = []
  for (const chunk of chunks) {
    parts.push(Buffer.from(compressor.update(chunk)))
  }
  parts.push(Buffer.from(await compressor.finish()))
  return Buffer.concat(parts)
}

/** Drive class decompressor over framed chunks; return plaintext. */
export async function driveClassUncompress(chunks: Uint8Array[]): Promise<Buffer> {
  const { Decompressor } = loadBinding()
  const decompressor = new Decompressor()
  const parts: Buffer[] = []
  for (const chunk of chunks) {
    parts.push(Buffer.from(decompressor.update(chunk)))
  }
  parts.push(Buffer.from(await decompressor.finish()))
  return Buffer.concat(parts)
}

/** Collect a Web ReadableStream into one Buffer. */
export async function collectWebStream(stream: ReadableStream<Uint8Array>): Promise<Buffer> {
  const reader = stream.getReader()
  const chunks: Buffer[] = []
  try {
    for (;;) {
      const { done, value } = await reader.read()
      if (done) break
      if (value && value.length) {
        chunks.push(Buffer.from(value))
      }
    }
  } finally {
    try {
      reader.releaseLock()
    } catch {
      // ignore
    }
  }
  return Buffer.concat(chunks)
}

/** Wrap Buffer chunks as a WHATWG ReadableStream. */
export function bufferToStream(chunks: Uint8Array[]): ReadableStream<Uint8Array> {
  let i = 0
  return new ReadableStream({
    pull(controller) {
      if (i >= chunks.length) {
        controller.close()
        return
      }
      controller.enqueue(chunks[i++])
    },
  })
}
