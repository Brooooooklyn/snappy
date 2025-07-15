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

|                   | node12 | node14 | node16 | node18 | node20 | node22 |
| ----------------- | ------ | ------ | ------ | ------ | ------ | ------ |
| Windows x64       | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Windows x32       | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Windows arm64     | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| macOS x64         | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| macOS arm64       | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu     | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl    | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu     | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu   | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl  | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux riscv64     | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux s390x       | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Linux powerpc64le | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| OpenHarmony arm64 | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Android arm64     | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Android armv7     | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Android x86       | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| Android x64       | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64       | ✓      | ✓      | ✓      | ✓      | ✓      | ✓      |
| WebAssembly       |        |        |        | ✓      | ✓      | ✓      |

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
    4 220 ops/s, ±0.66%   | fastest

  snappy-v6:
    2 018 ops/s, ±0.84%   | 52.18% slower

  gzip:
    233 ops/s, ±0.52%     | slowest, 94.48% slower

  deflate:
    235 ops/s, ±0.45%     | 94.43% slower

  brotli:
    7 ops/s, ±0.51%       | slowest, 99.85% slower

Finished 4 cases!
  Fastest: snappy
  Slowest: brotli

Running "Decompress" suite...
Progress: 100%

  snappy:
    8 528 ops/s, ±1.03%   | fastest

  snappy-v6:
    6 357 ops/s, ±1.76%   | 25.46% slower

  gzip:
    1 406 ops/s, ±1.80%   | slowest, 83.51% slower

  deflate:
    1 435 ops/s, ±1.88%   | 83.17% slower

  brotli:
    1 208 ops/s, ±1.50%   | slowest, 86.99% slower

Finished 4 cases!
  Fastest: snappy
  Slowest: brotli
```
