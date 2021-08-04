const { writeFileSync } = require('fs')
const { join } = require('path')

const pkgJson = require('./package.json')

pkgJson.optionalDependencies = Object.entries(pkgJson.optionalDependencies).reduce((acc, [name, value]) => {
  acc[`@napi-rs/${name}`] = value
  return acc
}, {})

writeFileSync(join(__dirname, 'package.json'), JSON.stringify(pkgJson, null, 2))
