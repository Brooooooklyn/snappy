import { promises as fs } from 'node:fs'
import { setTimeout } from 'node:timers/promises'

import chalk from 'chalk'
import prettyBytes from 'pretty-bytes'

import { displayMemoryUsageFromNode } from './util.mjs'

import { compress, uncompress } from './index.js'

const YARN_LOCK = await fs.readFile('yarn.lock')
const COMPRESSED_YARN_LOCK = await compress(YARN_LOCK)

let initial = process.memoryUsage()

async function detect(job) {
  for (let i = 0; i <= 100000; i++) {
    await job()
    if (i % 1000 === 0) {
      await setTimeout(100)
      if (typeof global.gc === 'function') {
        global.gc()
      }
      if (i === 1000) {
        initial = process.memoryUsage()
      }
      displayMemoryUsageFromNode(initial)
    }
    const memoryDiff = process.memoryUsage().rss - initial.rss
    if (memoryDiff >= 1024 * 1024 * 200) {
      throw new Error(`Memory limit reached ${prettyBytes(memoryDiff)}`)
    }
  }
}

console.info(chalk.green('Decompress broken buffer...'))

await detect(async () => uncompress(COMPRESSED_YARN_LOCK).catch(() => {}))

await setTimeout(1000)

if (typeof global.gc === 'function') {
  global.gc()
}

console.info(chalk.green('Compress file...'))

await detect(async () => compress(YARN_LOCK))
