import test from 'ava'

import {
  chunkByByte,
  chunkBySize,
  driveClassCompress,
  driveClassUncompress,
  loadBinding,
} from './helpers'

const INPUT = Buffer.from('Hello 🚀'.repeat(500), 'utf8')

const emptyAnd1Byte = (buf: Buffer): Uint8Array[] => {
  const chunks: Uint8Array[] = [Buffer.alloc(0)]
  for (const byte of chunkByByte(buf)) {
    chunks.push(byte)
    chunks.push(Buffer.alloc(0))
  }
  return chunks
}

const CHUNKINGS: ReadonlyArray<{ name: string; split: (buf: Buffer) => Uint8Array[] }> = [
  { name: '1-byte', split: (buf) => chunkByByte(buf) },
  { name: '64-byte', split: (buf) => chunkBySize(buf, 64) },
  { name: 'single-chunk', split: (buf) => [buf] },
  { name: 'awkward empty+1-byte', split: (buf) => emptyAnd1Byte(buf) },
]

for (const { name, split } of CHUNKINGS) {
  test(`class compress round-trips via class uncompress (${name})`, async (t) => {
    const compressed = await driveClassCompress(split(INPUT))
    const restored = await driveClassUncompress(chunkBySize(compressed, 64))
    t.deepEqual(restored, INPUT)
  })
}

test('class compress output is byte-identical across all chunkings', async (t) => {
  const reference = await driveClassCompress([INPUT])
  for (const { name, split } of CHUNKINGS) {
    const got = await driveClassCompress(split(INPUT))
    t.true(got.equals(reference), `${name} output diverged from the single-chunk reference`)
  }
})

test('class compress of empty input decodes back to empty', async (t) => {
  const compressed = await driveClassCompress([])
  const restored = await driveClassUncompress([compressed])
  t.is(restored.length, 0)
})

const STRING_CHUNK = 'Hello 🚀 streaming string chunk — Ünïcöde'

test('class compress accepts a string chunk (UTF-8) and round-trips', async (t) => {
  const compressed = await driveClassCompress([STRING_CHUNK])
  const restored = await driveClassUncompress([compressed])
  t.deepEqual(restored, Buffer.from(STRING_CHUNK, 'utf8'))
})

test('a string chunk compresses byte-identically to the equivalent Uint8Array', async (t) => {
  const fromString = await driveClassCompress([STRING_CHUNK])
  const fromBytes = await driveClassCompress([Buffer.from(STRING_CHUNK, 'utf8')])
  t.true(fromString.equals(fromBytes))
})

test('double finish on Compressor rejects', async (t) => {
  const { Compressor } = loadBinding()
  const c = new Compressor()
  await c.finish()
  // finish() takes the encoder synchronously, so a second call throws (not rejects).
  t.throws(() => c.finish(), { code: 'InvalidArg' })
})

test('double finish on Decompressor rejects', async (t) => {
  const { Decompressor } = loadBinding()
  const d = new Decompressor()
  await d.finish()
  t.throws(() => d.finish(), { code: 'InvalidArg' })
})

test('framed stream output is NOT valid raw uncompress input', async (t) => {
  const { uncompressSync } = loadBinding()
  const framed = await driveClassCompress([INPUT])
  t.throws(() => uncompressSync(framed), { any: true })
})

test('corrupt framed input surfaces InvalidArg', async (t) => {
  const { Decompressor } = loadBinding()
  const d = new Decompressor()
  // Snappy frame magic is sNaPpY; garbage should fail at finish or update.
  const garbage = Buffer.from('this is not framed snappy data at all!!!!!')
  try {
    d.update(garbage)
    await d.finish()
    t.fail('expected decode error')
  } catch (err) {
    t.true(err instanceof Error)
    t.is((err as { code?: string }).code, 'InvalidArg')
  }
})
