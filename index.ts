import { Transform, TransformCallback } from 'stream'

const { loadBinding } = require('@node-rs/helper')

const bindings = loadBinding(__dirname, 'snappy', '@napi-rs/snappy')

export class SnappyStream extends Transform {
  _transform(chunk: any, _encoding: BufferEncoding, callback: TransformCallback) {
    try {
      callback(null, bindings.encode(Buffer.from(chunk)))
    } catch (e) {
      callback(e)
    }
  }
}
