import { readFileSync } from 'fs'
import { join } from 'path'
import { promisify } from 'util'
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
} from 'zlib'

import b from 'benny'

import { compress, uncompress, compressSync } from '../index'

const gzipAsync = promisify(gzip)
const brotliCompressAsync = promisify(brotliCompress)
const deflateAsync = promisify(deflate)
const gunzipAsync = promisify(gunzip)
const inflateAsync = promisify(inflate)
const brotliDecompressAsync = promisify(brotliDecompress)

const FIXTURE = readFileSync(join(__dirname, '..', 'yarn.lock'))
const SNAPPY_COMPRESSED_FIXTURE = Buffer.from(compressSync(FIXTURE))
const GZIP_FIXTURE = gzipSync(FIXTURE)
const DEFLATED_FIXTURE = deflateSync(FIXTURE)
const BROTLI_COMPRESSED_FIXTURE = brotliCompressSync(FIXTURE)

async function run() {
  await b.suite(
    'Compress',

    b.add('snappy', () => {
      return compress(FIXTURE)
    }),

    b.add('gzip', () => {
      return gzipAsync(FIXTURE)
    }),

    b.add('deflate', () => {
      return deflateAsync(FIXTURE)
    }),

    b.add('brotli', () => {
      return brotliCompressAsync(FIXTURE)
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'Decompress',

    b.add('snappy', () => {
      return uncompress(SNAPPY_COMPRESSED_FIXTURE)
    }),

    b.add('gzip', () => {
      return gunzipAsync(GZIP_FIXTURE)
    }),

    b.add('deflate', () => {
      return inflateAsync(DEFLATED_FIXTURE)
    }),

    b.add('brotli', () => {
      return brotliDecompressAsync(BROTLI_COMPRESSED_FIXTURE)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
