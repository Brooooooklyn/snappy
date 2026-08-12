// Node-only CJS helper: Duplex stream factories via `Duplex.fromWeb`.
//
// Pure browser-safe streaming logic lives in `stream-polyfill.mjs`. This file
// holds only the Node `Duplex` bridge (`node:stream`), so it is never referenced
// from any browser condition target.

'use strict'

/**
 * Build `{ createCompressStream, createUncompressStream }` bridging WHATWG
 * web-stream transforms to ready-to-pipe Node `Duplex`es.
 *
 * @param {object} api
 * @param {Function} api.compressStream
 * @param {Function} api.uncompressStream
 */
function createNodeStreamFactories({ compressStream, uncompressStream }) {
  const { Duplex } = require('node:stream')
  const bridge = (transform) => () => {
    const { readable, writable } = new TransformStream()
    return Duplex.fromWeb({ writable, readable: transform(readable) })
  }
  return {
    createCompressStream: bridge(compressStream),
    createUncompressStream: bridge(uncompressStream),
  }
}

module.exports = {
  createNodeStreamFactories,
}
