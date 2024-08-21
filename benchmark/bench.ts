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

import { compress, uncompress, compressSync } from '../index.js'
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

b.add('snappy', () => {
  return compress(FIXTURE)
})

b.add('snappy-v6', () => {
  return compressV6(FIXTURE)
})

b.add('gzip', () => {
  return gzipAsync(FIXTURE)
})

b.add('deflate', () => {
  return deflateAsync(FIXTURE)
})

b.add('brotli', () => {
  return brotliCompressAsync(FIXTURE)
})

await b.run()

console.table(b.table())

b.add('snappy', () => {
  return uncompress(SNAPPY_COMPRESSED_FIXTURE)
})

b.add('snappy-v6', () => {
  // @ts-expect-error
  return uncompressV6(SNAPPY_COMPRESSED_FIXTURE)
})

b.add('gzip', () => {
  return gunzipAsync(GZIP_FIXTURE)
})

b.add('deflate', () => {
  return inflateAsync(DEFLATED_FIXTURE)
})

b.add('brotli', () => {
  return brotliDecompressAsync(BROTLI_COMPRESSED_FIXTURE)
})

await b.run()

console.table(b.table())
