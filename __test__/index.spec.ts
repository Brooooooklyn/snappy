import test from 'ava'

import { compressSync, compress, uncompressSync, uncompress } from '../index'

test('should be able to compress Buffer', (t) => {
  const fixture = 'hello world'
  t.deepEqual(compressSync(fixture), Buffer.from([11, 40, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]))
})

test('should be able to compress Buffer with copied option', (t) => {
  const fixture = 'hello world'
  t.deepEqual(
    compressSync(fixture, { copyOutputData: true }),
    Buffer.from([11, 40, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]),
  )
})

test('compress should be async version of compressSync', async (t) => {
  const fixture = 'hello world 😂 🎧 🚀'
  t.deepEqual(await compress(fixture), compressSync(fixture))
})

test('should be able to compress with copyOutputData', async (t) => {
  const fixture = 'hello world 😂 🎧 🚀'
  t.deepEqual(await compress(fixture, { copyOutputData: true }), compressSync(fixture))
})

test('should be able to uncompress sync', (t) => {
  const fixture = 'hello world 😂 🎧 🚀'
  t.deepEqual(uncompressSync(compressSync(fixture)), Buffer.from(fixture))
})

test('should be able to uncompress sync into string', (t) => {
  const fixture = 'hello world 😂 🎧 🚀'
  t.deepEqual(uncompressSync(compressSync(fixture), { asBuffer: false }), fixture)
})

test('should be able to uncompress', async (t) => {
  const fixture = 'hello world 😂 🎧 🚀'
  t.deepEqual(await uncompress(await compress(fixture)), Buffer.from(fixture))
})

test('should be able to uncompress into string', async (t) => {
  const fixture = 'hello world 😂 🎧 🚀'
  t.deepEqual(await uncompress(await compress(fixture), { asBuffer: false }), fixture)
})
