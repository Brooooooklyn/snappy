/* browser / wasm entry types for snappy streaming */
export declare function compress(
  input: string | Uint8Array,
  options?: EncOptions | undefined | null,
  signal?: AbortSignal | undefined | null,
): Promise<Buffer>

export declare function compressSync(
  input: string | Uint8Array,
  options?: EncOptions | undefined | null,
): Buffer

export declare function uncompress(
  input: string | Uint8Array,
  options?: DecOptions | undefined | null,
  signal?: AbortSignal | undefined | null,
): Promise<string | Buffer>

export declare function uncompressSync(
  input: string | Uint8Array,
  options?: DecOptions | undefined | null,
): string | Buffer

export interface DecOptions {
  asBuffer?: boolean
  copyOutputData?: boolean
}

export interface EncOptions {
  copyOutputData?: boolean
}

/** Incremental framed-Snappy compressor (not the raw one-shot format). */
export declare class Compressor {
  constructor()
  update(chunk: string | Uint8Array): Buffer
  finish(): Promise<Buffer>
}

/** Incremental framed-Snappy decompressor (not the raw one-shot format). */
export declare class Decompressor {
  constructor()
  update(chunk: Uint8Array): Buffer
  finish(): Promise<Buffer>
}

export declare function compressStream(input: ReadableStream<Uint8Array>): ReadableStream<Uint8Array>
export declare function uncompressStream(input: ReadableStream<Uint8Array>): ReadableStream<Uint8Array>
