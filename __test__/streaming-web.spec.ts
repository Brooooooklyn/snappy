import test from 'ava'

import {
  bufferToStream,
  chunkBySize,
  collectWebStream,
  driveClassCompress,
  IS_WASI,
  loadBinding,
} from './helpers'

const INPUT = Buffer.from('Web stream 🚀 snappy framed '.repeat(2048), 'utf8')

// Native web streams are compiled out under WASI; polyfill still works via class API.
const webTest = test

webTest('compressStream → uncompressStream round-trips', async (t) => {
  const { compressStream, uncompressStream } = loadBinding()
  const compressed = await collectWebStream(compressStream(bufferToStream([INPUT])))
  const restored = await collectWebStream(uncompressStream(bufferToStream(chunkBySize(compressed, 4096))))
  t.deepEqual(restored, INPUT)
})

webTest('compressStream multi-chunk input round-trips', async (t) => {
  const { compressStream, uncompressStream } = loadBinding()
  const compressed = await collectWebStream(compressStream(bufferToStream(chunkBySize(INPUT, 1024))))
  const restored = await collectWebStream(uncompressStream(bufferToStream([compressed])))
  t.deepEqual(restored, INPUT)
})

webTest('uncompressStream decodes class-compressed framed stream', async (t) => {
  const { uncompressStream } = loadBinding()
  const compressed = await driveClassCompress([INPUT])
  const restored = await collectWebStream(uncompressStream(bufferToStream(chunkBySize(compressed, 7))))
  t.deepEqual(restored, INPUT)
})

webTest('empty input stream round-trips', async (t) => {
  const { compressStream, uncompressStream } = loadBinding()
  const compressed = await collectWebStream(compressStream(bufferToStream([])))
  const restored = await collectWebStream(uncompressStream(bufferToStream([compressed])))
  t.is(restored.length, 0)
})

if (!IS_WASI) {
  webTest('native compressStream is present on non-WASI builds', async (t) => {
    // Raw binding (index.js) should expose native transforms when not WASI-forced.
    const { createRequire } = await import('node:module')
    const requireFrom = createRequire(import.meta.url)
    const raw = requireFrom('../index.js') as { compressStream?: unknown }
    t.is(typeof raw.compressStream, 'function')
  })
}
