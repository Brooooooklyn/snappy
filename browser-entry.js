// Honest browser / wasm entry for `snappy`.
//
// The napi-generated `browser.js` is `export * from '@napi-rs/snappy-wasm32-wasi'`
// and is REWRITTEN on every `napi build`. The wasm build also compiles out the
// tokio-backed stream transforms, so this wrapper imports the wasm binding,
// fills stream fns via the class-API polyfill, and re-exports classes + one-shot
// APIs. Node Duplex factories are intentionally omitted (browser has no node:stream).

import * as binding from '@napi-rs/snappy-wasm32-wasi'
import { honestStreams } from './stream-polyfill.mjs'

const { compressStream, uncompressStream } = honestStreams(binding)

export const compress = binding.compress
export const compressSync = binding.compressSync
export const uncompress = binding.uncompress
export const uncompressSync = binding.uncompressSync
export const Compressor = binding.Compressor
export const Decompressor = binding.Decompressor
export { compressStream, uncompressStream }
