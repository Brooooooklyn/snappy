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

<!-- Rendered live by the napi.rs support-matrix badge service — not committed SVGs.
     The <img> src is a PNG on purpose: npm proxies <img src> through camo, which
     mangles remote SVG but passes raster untouched, so the light PNG renders on
     npm / npmx / editors / crates. GitHub keeps <picture>, so dark-OS readers get
     the dark PNG via <source>. Commas in the URL are %2C-encoded so <source srcset>
     does not mis-split them. The full matrix is reproduced as text below for search
     and screen readers. To change the card, edit the query (see /support-matrix on
     napi.rs) — no image to re-commit. -->

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://napi.rs/support-matrix.png?engines=%3E%3D%2010&nodeTested=22%2C24&tested=x86_64-pc-windows-msvc%2Caarch64-pc-windows-msvc%2Cx86_64-apple-darwin%2Caarch64-apple-darwin%2Cx86_64-unknown-linux-gnu%2Cx86_64-unknown-linux-musl%2Caarch64-unknown-linux-gnu%2Caarch64-unknown-linux-musl%2Carmv7-unknown-linux-gnueabihf%2Cs390x-unknown-linux-gnu%2Cwasm32-wasip1-threads&untested=i686-pc-windows-msvc%2Cx86_64-unknown-freebsd%2Criscv64gc-unknown-linux-gnu%2Cpowerpc64le-unknown-linux-gnu%2Caarch64-linux-android%2Carm-linux-androideabi%2Caarch64-unknown-linux-ohos&theme=dark">
  <img alt="snappy support matrix. Node.js &gt;= 10; CI tests Node 22 and 24. 18 prebuilt targets across Windows, macOS, Linux, Android, FreeBSD, OpenHarmony and wasm32-wasi: 11 CI-tested, 7 built but untested." src="https://napi.rs/support-matrix.png?engines=%3E%3D%2010&nodeTested=22%2C24&tested=x86_64-pc-windows-msvc%2Caarch64-pc-windows-msvc%2Cx86_64-apple-darwin%2Caarch64-apple-darwin%2Cx86_64-unknown-linux-gnu%2Cx86_64-unknown-linux-musl%2Caarch64-unknown-linux-gnu%2Caarch64-unknown-linux-musl%2Carmv7-unknown-linux-gnueabihf%2Cs390x-unknown-linux-gnu%2Cwasm32-wasip1-threads&untested=i686-pc-windows-msvc%2Cx86_64-unknown-freebsd%2Criscv64gc-unknown-linux-gnu%2Cpowerpc64le-unknown-linux-gnu%2Caarch64-linux-android%2Carm-linux-androideabi%2Caarch64-unknown-linux-ohos">
</picture>

<details>
<summary>Full matrix as text</summary>

### Node.js

`engines.node` is `>= 10`. CI tests **Node 22** and **Node 24**.

### Targets

| Rust triple                     | Platform             | CI                              |
| ------------------------------- | -------------------- | ------------------------------- |
| `x86_64-pc-windows-msvc`        | Windows x64          | tested — node 22, 24            |
| `aarch64-pc-windows-msvc`       | Windows arm64        | tested — node 22, 24            |
| `i686-pc-windows-msvc`          | Windows x32          | built, not tested               |
| `x86_64-apple-darwin`           | macOS x64            | tested — node 22, 24            |
| `aarch64-apple-darwin`          | macOS arm64          | tested — node 22, 24            |
| `x86_64-unknown-linux-gnu`      | Linux x64 gnu        | tested — node 22, 24            |
| `x86_64-unknown-linux-musl`     | Linux x64 musl       | tested — node 22, 24            |
| `aarch64-unknown-linux-gnu`     | Linux arm64 gnu      | tested — node 22, 24            |
| `aarch64-unknown-linux-musl`    | Linux arm64 musl     | tested — node 22, 24            |
| `armv7-unknown-linux-gnueabihf` | Linux armv7 gnu      | tested — node 22 only           |
| `s390x-unknown-linux-gnu`       | Linux s390x          | tested — node 22, 24            |
| `x86_64-unknown-freebsd`        | FreeBSD x64          | built, not tested               |
| `powerpc64le-unknown-linux-gnu` | Linux ppc64le        | built, not tested               |
| `riscv64gc-unknown-linux-gnu`   | Linux riscv64        | built, not tested               |
| `aarch64-linux-android`         | Android arm64        | built, not tested               |
| `arm-linux-androideabi`         | Android armv7        | built, not tested               |
| `aarch64-unknown-linux-ohos`    | OpenHarmony arm64    | built, not tested               |
| `wasm32-wasip1-threads`         | wasm32-wasi, browser | tested — node 24 (`NAPI_RS_FORCE_WASI`) |

Eighteen targets: eleven CI-tested, seven built but not exercised.

### Browser

Bundlers resolve the wasm package through the `browser` export condition. The wasm
build allocates shared memory and spawns worker threads, so `SharedArrayBuffer` must be
available — the page has to be
[cross-origin isolated](https://developer.mozilla.org/docs/Web/API/Window/crossOriginIsolated),
served with `Cross-Origin-Opener-Policy: same-origin` and
`Cross-Origin-Embedder-Policy: require-corp`.

</details>

## API

### One-shot (raw Snappy block format)

```ts
export function compressSync(input: Buffer | string | ArrayBuffer | Uint8Array): Buffer
export function compress(input: Buffer | string | ArrayBuffer | Uint8Array): Promise<Buffer>
export function uncompressSync(compressed: Buffer): Buffer
export function uncompress(compressed: Buffer): Promise<Buffer>
```

### Streaming (framed Snappy format)

Streaming uses the [Snappy frame format](https://github.com/google/snappy/blob/master/framing_format.txt)
(file extension `.sz`). This is **not** the same wire format as the one-shot APIs
above — framed output cannot be passed to `uncompress()`, and raw blocks cannot
be passed to the stream decompressors.

#### Incremental classes

```js
import { Compressor, Decompressor } from 'snappy'

const compressor = new Compressor()
const parts = [compressor.update('Hello '), compressor.update('snappy 🚀'), await compressor.finish()]
const compressed = Buffer.concat(parts)

const decompressor = new Decompressor()
const restored = Buffer.concat([decompressor.update(compressed), await decompressor.finish()])
console.log(restored.toString('utf8')) // Hello snappy 🚀
```

The valid stream is the concatenation of every `update()` output plus the `finish()` tail.

#### Web Streams

```js
import { compressStream, uncompressStream } from 'snappy'

const restored = uncompressStream(compressStream(source)) // ReadableStream<Uint8Array>
```

`input` must be a WHATWG `ReadableStream`; wrap a Node `Readable` with `Readable.toWeb()`.

On wasm / browser builds the native transforms are unavailable; a buffered class-API
polyfill is used automatically.

#### Node Duplex factories

```js
import { createReadStream, createWriteStream } from 'node:fs'
import { createCompressStream, createUncompressStream } from 'snappy'

createReadStream('input.txt')
  .pipe(createCompressStream())
  .pipe(createWriteStream('input.txt.sz'))
```

Requires a modern Node.js with Web Streams and `Duplex.fromWeb` (effectively Node 18+).

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
