import { execFileSync, spawnSync } from 'node:child_process'
import { copyFileSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
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
 * fails under `--no-experimental-require-module`, ts-node and Jest.
 * Regression guard for https://github.com/Brooooooklyn/snappy/issues/357.
 */
test('CommonJS entry never requires an ES module', (t) => {
  for (const file of CJS_ENTRY_FILES) {
    const source = readFileSync(new URL(file, rootUrl), 'utf8')
    const esmRequires = source.match(/require\(\s*['"][^'"]+\.mjs['"]\s*\)/g)
    t.is(esmRequires, null, `${file} must not require() an .mjs file, found: ${esmRequires?.join(', ')}`)
  }
})

/**
 * `--no-require-module` only exists from Node 24.21; `--no-experimental-require-module`
 * is the alias Node has accepted ever since `require(esm)` landed. Anything older
 * has no `require(esm)` to switch off, so the probe is meaningless there.
 */
const REQUIRE_ESM_OFF = '--no-experimental-require-module'
const supportsFlag = spawnSync(process.execPath, [REQUIRE_ESM_OFF, '-e', ''], { encoding: 'utf8' }).status === 0
const withRequireEsmOff = supportsFlag ? test : test.skip

/**
 * Load the JS wrapper graph with `require(esm)` switched off, against a stub
 * binding. The stub keeps the probe on our own modules: the native and wasm
 * loaders drag in `@napi-rs/wasm-runtime`, which has its own `require(esm)` edge
 * that this test is not about.
 */
withRequireEsmOff('the CommonJS entry graph loads with require(esm) disabled', (t) => {
  const dir = mkdtempSync(join(tmpdir(), 'snappy-cjs-entry-'))
  try {
    for (const file of CJS_ENTRY_FILES) {
      copyFileSync(join(root, file), join(dir, file))
    }
    writeFileSync(
      join(dir, 'index.js'),
      `'use strict'\nconst stub = () => {}\nmodule.exports = {\n` +
        ['compress', 'compressSync', 'uncompress', 'uncompressSync', 'Compressor', 'Decompressor']
          .map((name) => `  ${name}: stub,\n`)
          .join('') +
        `}\n`,
    )
    writeFileSync(join(dir, 'probe.cjs'), `console.log(Object.keys(require('./main.js')).sort().join(','))\n`)

    const stdout = execFileSync(process.execPath, [REQUIRE_ESM_OFF, join(dir, 'probe.cjs')], { encoding: 'utf8' })
    t.is(
      stdout.trim(),
      'Compressor,Decompressor,compress,compressStream,compressSync,createCompressStream,createUncompressStream,uncompress,uncompressStream,uncompressSync',
    )
  } finally {
    rmSync(dir, { recursive: true, force: true })
  }
})

const SHARED_START = '/* --- shared start: keep byte-identical with the twin file, see cjs-entry.spec.ts --- */'
const SHARED_END = '/* --- shared end --- */'

function sharedBody(file: string): string {
  const source = readFileSync(new URL(file, rootUrl), 'utf8')
  const start = source.indexOf(SHARED_START)
  const end = source.indexOf(SHARED_END)
  if (start === -1 || end === -1) {
    throw new Error(`${file} is missing its shared markers`)
  }
  // `.gitattributes` pins `*.js` to LF but leaves `*.mjs` on `text=auto`, so a
  // Windows checkout gives the twins different line endings. Compare content.
  return source
    .slice(start + SHARED_START.length, end)
    .replace(/\r\n/g, '\n')
    .trim()
}

/**
 * `stream-polyfill.js` (CommonJS, for `main.js`) and `stream-polyfill.mjs`
 * (ES module, for `browser-entry.js`) hold the same logic. Plain Rollup cannot
 * read named exports out of CommonJS, so the browser path needs real `export`
 * statements and a re-export shim will not do. Fail loudly when they drift.
 */
test('the CommonJS and ES module polyfills stay in sync', (t) => {
  const esm = sharedBody('stream-polyfill.mjs').replace(/^export /gm, '')
  t.is(esm, sharedBody('stream-polyfill.js'))
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
