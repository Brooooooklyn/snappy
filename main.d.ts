/* package entry types: native binding + JS stream factories */
export * from './index'

import type { Duplex } from 'node:stream'

/**
 * Convenience Node-stream factory: returns a ready-to-pipe `Duplex` that
 * compresses with the framed Snappy format. Requires Node.js with Web Streams
 * and `Duplex.fromWeb` (effectively Node 18+).
 */
export declare function createCompressStream(): Duplex

/**
 * Convenience Node-stream factory: returns a ready-to-pipe `Duplex` that
 * decompresses framed Snappy. Requires Node.js with Web Streams and
 * `Duplex.fromWeb` (effectively Node 18+).
 */
export declare function createUncompressStream(): Duplex
