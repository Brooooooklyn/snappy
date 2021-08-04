const { loadBinding } = require('@node-rs/helper')

const {
  compressSync: _compressSync,
  compress: _compress,
  uncompress,
  uncompressSync,
} = loadBinding(__dirname, 'snappy', 'snappy')

module.exports = {
  compressSync: function compressSync(input) {
    return _compressSync(Buffer.from(input))
  },
  compress: function compress(input) {
    return _compress(Buffer.from(input))
  },
  uncompress,
  uncompressSync,
}
