import { promises as fs } from 'fs'

import chalk from 'chalk'

import { displayMemoryUsageFromNode } from './util.mjs'

import { compress, uncompress } from './index.js'

const initial = process.memoryUsage()

async function detect(job) {
  for (let i = 0; i <= 100000; i++) {
    await job()
    if (i % 1000 === 0) {
      displayMemoryUsageFromNode(initial)
    }

    if (process.memoryUsage().rss - initial.rss >= 1024 * 1024 * 200) {
      throw new Error('Memory limit reached')
    }
  }
}

console.info(chalk.green('Decompress broken buffer...'))

await detect(async () => uncompress(await fs.readFile('yarn.lock')).catch(() => {}))

console.info(chalk.green('Compress file...'))

await detect(async () => compress(await fs.readFile('yarn.lock')))
