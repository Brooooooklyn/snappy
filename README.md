# `@napi-rs/snappy`

![https://github.com/Brooooooklyn/snappy/actions](https://github.com/Brooooooklyn/snappy/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@napi-rs/snappy.svg?sanitize=true)

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

Fastest Snappy compression library in Node.js, powered by [napi-rs](https://napi.rs) and [rust-snappy](https://github.com/BurntSushi/rust-snappy).

> For small size data, [snappyjs](https://github.com/zhipeng-jia/snappyjs) is faster, and it support browser. But it doesn't have async API, which is important for Node.js program.

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
OS: Windows 10 x86_64
Host: Micro-Star International Co., Ltd. MS-7C35
Kernel: 10.0.19043
Terminal: Windows Terminal
CPU: AMD Ryzen 9 5950X (32) @ 3.400GHz
Memory: 15959MiB / 32688MiB
```

### Result

```
Running "Compress data" suite...
Progress: 100%

  @napi-rs/snappy:
    333 ops/s, ±2.10%   | fastest

  snappy node:
    163 ops/s, ±1.44%   | slowest, 51.05% slower

Finished 2 cases!
  Fastest: @napi-rs/snappy
  Slowest: snappy node
Running "Uncompress data" suite...
Progress: 100%

  @napi-rs/snappy:
    980 ops/s, ±1.85%   | fastest

  snappy node:
    256 ops/s, ±0.61%   | slowest, 73.88% slower

Finished 2 cases!
  Fastest: @napi-rs/snappy
  Slowest: snappy node

Running "Small size sync compress" suite...
Progress: 100%

  @napi-rs/snappy:
    505 211 ops/s, ±7.97%   | slowest, 47% slower

  snappy js:
    953 272 ops/s, ±0.37%   | fastest

Finished 2 cases!
  Fastest: snappy js
  Slowest: @napi-rs/snappy
```
