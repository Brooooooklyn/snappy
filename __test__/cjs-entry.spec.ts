import { execFileSync, spawnSync } from 'node:child_process'
import { readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'

import test from 'ava'

const requireFrom = createRequire(import.meta.url)
const rootUrl = new URL('../', import.meta.url)
const root = fileURLToPath(rootUrl)

/** Every CommonJS file that `require('snappy')` pulls in on the main entry path. */
const CJS_ENTRY_FILES = ['main.js', 'stream-polyfill.js', 'node-stream.js']

/**
 * `main.js` is CommonJS, so anything it `require()`s must be CommonJS too.
 * Requiring an ES module works only on Node >= 20.19 / >= 22.12, and still
 * fails under `--no-require-module`, ts-node and Jest.
 * Regression guard for https://github.com/Brooooooklyn/snappy/issues/357.
 */
test('CommonJS entry never requires an ES module', (t) => {
  for (const file of CJS_ENTRY_FILES) {
    const source = readFileSync(new URL(file, rootUrl), 'utf8')
    const esmRequires = source.match(/require\(\s*['"][^'"]+\.mjs['"]\s*\)/g)
    t.is(esmRequires, null, `${file} must not require() an .mjs file, found: ${esmRequires?.join(', ')}`)
  }
})

/** Node accepts `--no-require-module` only where `require(esm)` exists at all. */
const supportsNoRequireModule =
  spawnSync(process.execPath, ['--no-require-module', '-e', ''], { encoding: 'utf8' }).status === 0

const withFlagOff = supportsNoRequireModule ? test : test.skip

withFlagOff('the package entry loads with require(esm) disabled', (t) => {
  const stdout = execFileSync(
    process.execPath,
    ['--no-require-module', '-e', 'console.log(typeof require(process.argv[1]).compressSync)', root],
    { encoding: 'utf8' },
  )
  t.is(stdout.trim(), 'function')
})

test('the package entry exposes the documented surface', (t) => {
  const snappy = requireFrom('..') as typeof import('..')
  for (const name of [
    'compress',
    'compressSync',
    'uncompress',
    'uncompressSync',
    'Compressor',
    'Decompressor',
    'compressStream',
    'uncompressStream',
    'createCompressStream',
    'createUncompressStream',
  ]) {
    t.is(typeof (snappy as unknown as Record<string, unknown>)[name], 'function', name)
  }
})
