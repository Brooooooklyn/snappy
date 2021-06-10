# `@napi-rs/snappy`

![https://github.com/Brooooooklyn/snappy/actions](https://github.com/Brooooooklyn/snappy/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@napi-rs/snappy.svg?sanitize=true)

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

Fastest Snappy compression library in Node.js, powered by [napi-rs](https://napi.rs) and [rust-snappy](https://github.com/BurntSushi/rust-snappy).

## Install this package

```
yarn add @napi-rs/snappy
```

## Support matrix

|                  | node12 | node14 | node16 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## API

```ts
export function compressSync(input: Buffer | string | ArrayBuffer | Uint8Array): Buffer
export function compress(input: Buffer | string | ArrayBuffer | Uint8Array): Promise<Buffer>
export function uncompressSync(compressed: Buffer): Buffer
export function uncompress(compressed: Buffer): Promise<Buffer>
```

## Performance

### Hardware

```
Model Name: MacBook Pro
Model Identifier: MacBookPro15,1
Processor Name: Intel Core i7
Processor Speed: 2.6 GHz
Number of Processors: 1
Total Number of Cores: 6
L2 Cache (per Core): 256 KB
L3 Cache: 12 MB
Hyper-Threading Technology: Enabled
Memory: 16 GB
```

### Result

```
Running "Compress data" suite...
Progress: 100%

  @napi-rs/snappy:
    279 ops/s, ±7.86%   | fastest

  snappy node:
    235 ops/s, ±1.85%   | slowest, 15.77% slower

Finished 2 cases!
  Fastest: @napi-rs/snappy
  Slowest: snappy node
Running "Uncompress data" suite...
Progress: 100%

  @napi-rs/snappy:
    379 ops/s, ±2.09%   | fastest

  snappy node:
    347 ops/s, ±1.93%   | slowest, 8.44% slower

Finished 2 cases!
  Fastest: @napi-rs/snappy
  Slowest: snappy node
```
