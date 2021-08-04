# `snappy`

![https://github.com/Brooooooklyn/snappy/actions](https://github.com/Brooooooklyn/snappy/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/snappy.svg?sanitize=true)
[![Install size](https://packagephobia.com/badge?p=snappy)](https://packagephobia.com/result?p=snappy)

**!!! For `snappy@6.x` and below, please go to [`node-snappy`](https://github.com/kesla/node-snappy).**

More background about the **6-7** changes, please read [this](https://github.com/Brooooooklyn/snappy/issues/16), Thanks [@kesla](https://github.com/kesla) .

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

Fastest Snappy compression library in Node.js, powered by [napi-rs](https://napi.rs) and [rust-snappy](https://github.com/BurntSushi/rust-snappy).

> For small size data, [snappyjs](https://github.com/zhipeng-jia/snappyjs) is faster, and it support browser. But it doesn't have async API, which is important for Node.js program.

## Install this package

```
yarn add snappy
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
Processor Name: 6-Core Intel Core i7
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
Running "Compress" suite...
Progress: 25%

  snappy:
    1 426 ops/s, ±2.26%

  gzip:
    152 ops/s, ±1.54%

  deflate:
    155 ops/s, ±2.14%

  brotli:
    3 ops/s, ±3.43%       | slowest, 99.79% slower

Finished 4 cases!
  Fastest: snappy
  Slowest: brotli

Running "Decompress" suite...
Progress: 25%

  snappy:
    2 771 ops/s, ±1.13%

  gzip:
    854 ops/s, ±6.99%

  deflate:
    877 ops/s, ±3.19%

  brotli:
    638 ops/s, ±2.31%     | slowest, 76.98% slower

Finished 4 cases!
  Fastest: snappy
  Slowest: brotli
```
