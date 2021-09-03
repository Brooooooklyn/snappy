const { displayMemoryUsageFromNode } = require('./util')

const { compress } = require('./index')

const initial = process.memoryUsage()

async function main() {
  for (let i = 0; i <= 1000000; i++) {
    await compress(Buffer.from('hello'))
    if (i % 10000 === 0) {
      displayMemoryUsageFromNode(initial)
    }

    if (process.memoryUsage().rss - initial.rss >= 1024 * 1024 * 100) {
      throw new Error('Memory limit reached')
    }
  }
}

main()
