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
const MEMORY_POOL = 50_000

const initiateMemoryPool = () => {
  const pool = new Array(MEMORY_POOL) as Uint8Array[]
  for (let i = 0; i < MEMORY_POOL; i++) {
    pool[i] = new Uint8Array(FIXTURE.length)
  }

  const response = {
    currentBufferIndex: 0,
    getAvailableBuffer() {
      const buffer = pool[response.currentBufferIndex]
      response.currentBufferIndex++
      return buffer!
    },
  }

  return response
}

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
const outputPool = initiateMemoryPool() // new Uint8Array(FIXTURE.length)

bUncompress.add('snappy-uncompress', () => {
  return uncompress(SNAPPY_COMPRESSED_FIXTURE)
})
bUncompress.add('snappy-alloc-uncompress', () => {
  return uncompress(SNAPPY_COMPRESSED_FIXTURE, { output: outputPool.getAvailableBuffer() })
})
bUncompress.add('snappy-sync-uncompress', () => {
  return uncompressSync(SNAPPY_COMPRESSED_FIXTURE)
})
const outputPool2 = initiateMemoryPool()

bUncompress.add('snappy-sync-alloc-uncompress', () => {
  return uncompressSync(SNAPPY_COMPRESSED_FIXTURE, { output: outputPool2.getAvailableBuffer() })
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
