import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { promisify } from 'node:util'
import {
  gzip,
  deflate,
  brotliCompress,
  inflate,
  brotliDecompress,
  gzipSync,
  deflateSync,
  brotliCompressSync,
  gunzip,
} from 'node:zlib'

import { Bench, hrtimeNow } from 'tinybench'
import { compress as legacyCompress, uncompress as legacyUncompress } from 'legacy-snappy'

import { compress, uncompress, compressSync, uncompressSync } from '../index.js'
import { fileURLToPath } from 'node:url'

const gzipAsync = promisify(gzip)
const brotliCompressAsync = promisify(brotliCompress)
const deflateAsync = promisify(deflate)
const gunzipAsync = promisify(gunzip)
const inflateAsync = promisify(inflate)
const brotliDecompressAsync = promisify(brotliDecompress)
const compressV6 = promisify(legacyCompress)
const uncompressV6 = promisify(legacyUncompress)

const FIXTURE = readFileSync(join(fileURLToPath(import.meta.url), '..', '..', 'yarn.lock'))
const SNAPPY_COMPRESSED_FIXTURE = Buffer.from(compressSync(FIXTURE))
const GZIP_FIXTURE = gzipSync(FIXTURE)
const DEFLATED_FIXTURE = deflateSync(FIXTURE)
const BROTLI_COMPRESSED_FIXTURE = brotliCompressSync(FIXTURE)

const b = new Bench({
  now: hrtimeNow,
})

b.add('snappy-compress', () => {
  return compress(FIXTURE)
})

b.add('snappy-compress-sync', () => {
  return compressSync(FIXTURE)
})

b.add('snappy-v6-compress', () => {
  return compressV6(FIXTURE)
})

b.add('gzip-compress', () => {
  return gzipAsync(FIXTURE)
})

b.add('deflate-compress', () => {
  return deflateAsync(FIXTURE)
})

b.add('brotli-compress', () => {
  return brotliCompressAsync(FIXTURE)
})

await b.run()

console.table(b.table())

const bUncompress = new Bench({
  now: hrtimeNow,
})
const output = new Uint8Array(FIXTURE.length)

bUncompress.add('snappy-uncompress', () => {
  return uncompress(SNAPPY_COMPRESSED_FIXTURE)
})
bUncompress.add('snappy-alloc', () => {
  return uncompress(SNAPPY_COMPRESSED_FIXTURE, { output: output })
})
bUncompress.add('snappy-sync', () => {
  return uncompressSync(SNAPPY_COMPRESSED_FIXTURE)
})
const output2 = new Uint8Array(FIXTURE.length)

bUncompress.add('snappy-sync-alloc', () => {
  return uncompressSync(SNAPPY_COMPRESSED_FIXTURE, { output: output2 })
})

bUncompress.add('snappy-v6-uncompress', () => {
  // @ts-expect-error
  return uncompressV6(SNAPPY_COMPRESSED_FIXTURE)
})

bUncompress.add('gzip-uncompress', () => {
  return gunzipAsync(GZIP_FIXTURE)
})

bUncompress.add('deflate-uncompress', () => {
  return inflateAsync(DEFLATED_FIXTURE)
})

bUncompress.add('brotli-uncompress', () => {
  return brotliDecompressAsync(BROTLI_COMPRESSED_FIXTURE)
})

await bUncompress.run()

console.table(bUncompress.table())
