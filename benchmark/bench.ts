import { readFileSync } from 'fs'
import { cpus } from 'os'
import { join } from 'path'

import b from 'benny'
import { compress as compressCpp, uncompress as uncompressCpp } from 'snappy'

import { compress, uncompress, compressSync } from '../index'

const FIXTURE = readFileSync(join(__dirname, '..', 'yarn.lock'))
const COMPRESSED_FIXTURE = Buffer.from(compressSync(FIXTURE))

const THREADS = cpus().length

async function run() {
  await b.suite(
    'Compress data',

    b.add('@napi-rs/snappy', async () => {
      await Promise.all(Array.from({ length: THREADS }).map(() => compress(FIXTURE)))
    }),

    b.add('snappy node', async () => {
      await Promise.all(
        Array.from({ length: THREADS }).map(
          () =>
            new Promise((resolve, reject) => {
              compressCpp(FIXTURE, (err, buffer) => {
                if (err) {
                  reject(err)
                } else {
                  resolve(buffer)
                }
              })
            }),
        ),
      )
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'Uncompress data',

    b.add('@napi-rs/snappy', async () => {
      await Promise.all(Array.from({ length: THREADS }).map(() => uncompress(COMPRESSED_FIXTURE)))
    }),

    b.add('snappy node', async () => {
      await Promise.all(
        Array.from({ length: THREADS }).map(
          () =>
            new Promise((resolve, reject) => {
              uncompressCpp(COMPRESSED_FIXTURE, { asBuffer: true }, (err, buffer) => {
                if (err) {
                  reject(err)
                } else {
                  resolve(buffer)
                }
              })
            }),
        ),
      )
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
