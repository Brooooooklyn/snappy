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

|                  | node12 | node14 | node16 | node17 |
| ---------------- | ------ | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      | ✓      |

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
OS: Windows 11 x86_64
Host: Micro-Star International Co., Ltd. MS-7C35
Kernel: 10.0.22000
Terminal: Windows Terminal
CPU: AMD Ryzen 9 5950X (32) @ 3.400GHz
Memory: 32688MiB
```

### Result

```
Running "Compress" suite...
Progress: 100%

  snappy:
    4 690 ops/s, ±0.66%   | fastest

  gzip:
    259 ops/s, ±0.85%     | 94.48% slower

  deflate:
    262 ops/s, ±0.59%     | 94.41% slower

  brotli:
    7 ops/s, ±0.51%       | slowest, 99.85% slower

Finished 4 cases!
  Fastest: snappy
  Slowest: brotli

Running "Decompress" suite...
Progress: 100%

  snappy:
    9 285 ops/s, ±6.18%   | fastest

  gzip:
    1 511 ops/s, ±1.96%   | 83.73% slower

  deflate:
    1 763 ops/s, ±1.36%   | 81.01% slower

  brotli:
    1 208 ops/s, ±1.50%   | slowest, 86.99% slower

Finished 4 cases!
  Fastest: snappy
  Slowest: brotli
```
