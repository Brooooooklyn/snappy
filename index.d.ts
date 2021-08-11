export function compressSync(input: Buffer | string | ArrayBuffer | Uint8Array): Buffer
export function compress(input: Buffer | string | ArrayBuffer | Uint8Array): Promise<Buffer>
export function uncompressSync(compressed: Buffer): Buffer
export function uncompressSync(compressed: Buffer, opt: { asBuffer: true }): Buffer
export function uncompressSync(compressed: Buffer, opt: { asBuffer: false }): string
export function uncompressSync(compressed: Buffer, opt?: { asBuffer: boolean }): string | Buffer
export function uncompress(compressed: Buffer): Promise<Buffer>
export function uncompress(compressed: Buffer, opt: { asBuffer: true }): Promise<Buffer>
export function uncompress(compressed: Buffer, opt: { asBuffer: false }): Promise<string>
export function uncompress(compressed: Buffer, opt?: { asBuffer: boolean }): Promise<string | Buffer>
