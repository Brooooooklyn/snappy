const { loadBinding } = require('@node-rs/helper')

const {
  compressSync: _compressSync,
  compress: _compress,
  uncompress: _uncompress,
  uncompressSync: _uncompressSync,
} = loadBinding(__dirname, 'snappy', '@napi-rs/snappy')

module.exports.compress = function compress(input) {
  return _compress(Buffer.isBuffer(input) ? input : Buffer.from(input))
}

module.exports.compressSync = function compressSync(input) {
  return _compressSync(Buffer.isBuffer(input) ? input : Buffer.from(input))
}

module.exports.compress = function compress(input) {
  return _compress(Buffer.isBuffer(input) ? input : Buffer.from(input))
}

module.exports.uncompress = function uncompress(input, opt = { asBuffer: true }) {
  return _uncompress(input, Boolean(opt.asBuffer))
}

module.exports.uncompressSync = function uncompressSync(input, opt = { asBuffer: true }) {
  return _uncompressSync(input, Boolean(opt.asBuffer))
}
