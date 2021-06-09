import { Duplex } from 'stream'

import test from 'ava'

import { SnappyStream } from '../index'

test('SnappyStream should be instance of Duplex', (t) => {
  t.true(new SnappyStream() instanceof Duplex)
})
