import { createReadStream, createWriteStream } from 'node:fs'
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { Readable } from 'node:stream'
import { pipeline } from 'node:stream/promises'

import test from 'ava'

import { chunkBySize, driveClassCompress, loadBinding } from './helpers'

const INPUT = Buffer.from('Node-stream factory 🚀 snappy bridge '.repeat(4096), 'utf8')

const collect = async (readable: NodeJS.ReadableStream): Promise<Buffer> => {
  const chunks: Buffer[] = []
  for await (const chunk of readable) {
    chunks.push(Buffer.from(chunk as Uint8Array))
  }
  return Buffer.concat(chunks)
}

test('createCompressStream → createUncompressStream round-trips (piped, multi-chunk)', async (t) => {
  const { createCompressStream, createUncompressStream } = loadBinding()
  const compressed = await collect(Readable.from(chunkBySize(INPUT, 64 * 1024)).pipe(createCompressStream()))
  const restored = await collect(Readable.from(chunkBySize(compressed, 4096)).pipe(createUncompressStream()))
  t.deepEqual(restored, INPUT)
})

test('createCompressStream output decodes via class Decompressor', async (t) => {
  const { createCompressStream } = loadBinding()
  const compressed = await collect(Readable.from([INPUT]).pipe(createCompressStream()))
  const { Decompressor } = loadBinding()
  const d = new Decompressor()
  const head = Buffer.from(d.update(compressed))
  const tail = Buffer.from(await d.finish())
  t.deepEqual(Buffer.concat([head, tail]), INPUT)
})

test('createUncompressStream decodes class-compressed framed stream', async (t) => {
  const { createUncompressStream } = loadBinding()
  const compressed = await driveClassCompress([INPUT])
  const restored = await collect(Readable.from(chunkBySize(compressed, 7)).pipe(createUncompressStream()))
  t.deepEqual(restored, INPUT)
})

test('fs.createReadStream → createCompressStream → createUncompressStream → file round-trips', async (t) => {
  const { createCompressStream, createUncompressStream } = loadBinding()
  const dir = await mkdtemp(join(tmpdir(), 'snappy-factory-'))
  t.teardown(() => rm(dir, { recursive: true, force: true }))
  const srcPath = join(dir, 'input.bin')
  const szPath = join(dir, 'output.sz')
  const outPath = join(dir, 'restored.bin')
  await writeFile(srcPath, INPUT)

  await pipeline(createReadStream(srcPath), createCompressStream(), createWriteStream(szPath))
  await pipeline(createReadStream(szPath), createUncompressStream(), createWriteStream(outPath))

  t.deepEqual(await readFile(outPath), INPUT)
})
