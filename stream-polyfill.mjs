// Shared, browser-safe Web Streams helpers + native-or-polyfill wiring.
//
// On a native build the Rust `compressStream` / `uncompressStream` transforms
// exist on the binding and are used directly. On the wasm build those tokio-
// backed fns are compiled out, so we fall back to a buffered polyfill over the
// (tokio-free) streaming class API.

/**
 * Concatenate an array of `Uint8Array` chunks into a single `Uint8Array`.
 * Buffer-free so it runs in a real browser.
 */
function concatChunks(chunks) {
  if (chunks.length === 0) {
    return new Uint8Array(0)
  }
  if (chunks.length === 1) {
    return chunks[0]
  }
  let total = 0
  for (const chunk of chunks) {
    total += chunk.length
  }
  const out = new Uint8Array(total)
  let offset = 0
  for (const chunk of chunks) {
    out.set(chunk, offset)
    offset += chunk.length
  }
  return out
}

/**
 * Drain a Web `ReadableStream<Uint8Array>` fully into a single `Uint8Array`.
 */
export async function bufferAll(input) {
  const reader = input.getReader()
  const chunks = []
  try {
    for (;;) {
      const { done, value } = await reader.read()
      if (done) break
      if (value && value.length) {
        chunks.push(new Uint8Array(value))
      }
    }
  } finally {
    try {
      reader.releaseLock()
    } catch {
      // Best-effort: an already-released/closed reader is fine.
    }
  }
  return concatChunks(chunks)
}

/**
 * Wrap an async `() => Promise<Uint8Array>` producer as a single-chunk
 * `ReadableStream`.
 */
export function singleChunkStream(produce) {
  let emitted = false
  return new ReadableStream({
    async pull(controller) {
      if (emitted) {
        return
      }
      emitted = true
      try {
        const out = await produce()
        if (out && out.length) {
          controller.enqueue(out)
        }
        controller.close()
      } catch (err) {
        controller.error(err)
      }
    },
  })
}

/**
 * Build `{ compressStream, uncompressStream }`: native transforms when present,
 * otherwise a buffered class-API polyfill.
 *
 * @param {object} spec
 * @param {Function|undefined} spec.nativeCompressStream
 * @param {Function|undefined} spec.nativeUncompressStream
 * @param {Function} spec.Compressor
 * @param {Function} spec.Decompressor
 */
export function createStreamApi({ nativeCompressStream, nativeUncompressStream, Compressor, Decompressor }) {
  const compressStream =
    typeof nativeCompressStream === 'function'
      ? nativeCompressStream
      : (input) =>
          singleChunkStream(async () => {
            const compressor = new Compressor()
            const head = new Uint8Array(await compressor.update(await bufferAll(input)))
            const tail = new Uint8Array(await compressor.finish())
            return head.length ? (tail.length ? concatChunks([head, tail]) : head) : tail
          })

  const uncompressStream =
    typeof nativeUncompressStream === 'function'
      ? nativeUncompressStream
      : (input) =>
          singleChunkStream(async () => {
            const decompressor = new Decompressor()
            const head = new Uint8Array(await decompressor.update(await bufferAll(input)))
            const tail = new Uint8Array(await decompressor.finish())
            return head.length ? (tail.length ? concatChunks([head, tail]) : head) : tail
          })

  return { compressStream, uncompressStream }
}

/**
 * Return honest stream fns for a loaded binding: native when present, else
 * class-API polyfill. Never mutates the raw binding object.
 *
 * @param {Record<string, unknown>} binding
 */
export function honestStreams(binding) {
  if (typeof binding.compressStream === 'function' && typeof binding.uncompressStream === 'function') {
    return {
      compressStream: binding.compressStream,
      uncompressStream: binding.uncompressStream,
    }
  }
  return createStreamApi({
    nativeCompressStream: binding.compressStream,
    nativeUncompressStream: binding.uncompressStream,
    Compressor: binding.Compressor,
    Decompressor: binding.Decompressor,
  })
}
