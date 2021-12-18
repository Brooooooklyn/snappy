import chalk from 'chalk'
import prettyBytes from 'pretty-bytes'
import { table } from 'table'

export function displayMemoryUsageFromNode(initialMemoryUsage) {
  const finalMemoryUsage = process.memoryUsage()
  const titles = Object.keys(initialMemoryUsage).map((k) => chalk.whiteBright(k))
  const tableData = [titles]
  const diffColumn = []
  for (const [key, value] of Object.entries(initialMemoryUsage)) {
    const diff = finalMemoryUsage[key] - value
    const prettyDiff = prettyBytes(diff, { signed: true })
    if (diff > 0) {
      diffColumn.push(chalk.red(prettyDiff))
    } else if (diff < 0) {
      diffColumn.push(chalk.green(prettyDiff))
    } else {
      diffColumn.push(chalk.grey(prettyDiff))
    }
  }
  tableData.push(diffColumn)
  console.info(table(tableData))
}
