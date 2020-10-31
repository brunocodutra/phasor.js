import { matcherHint, printExpected, printReceived } from 'jest-matcher-utils';

import { Phasor, polar } from '../';

export default {
  toBeCloseTo(x: number | Phasor, y: number | Phasor, ulps?: number) {
    const p: Phasor = typeof x === 'number' ? polar(x) : x;
    const q: Phasor = typeof y === 'number' ? polar(y) : y;
    const pass = p.isCloseTo(q, ulps && ulps * Number.EPSILON, ulps);

    const message = (
      pass
        ? () =>
          matcherHint('.not.toBeCloseTo', 'received', 'expected, precision') +
          '\n\n' +
          `Expected value not to be close to (within ${printExpected(ulps)} ulps):\n` +
          `  ${printExpected(y)}\n` +
          `Received:\n` +
          `  ${printReceived(x)}`
        : () =>
          matcherHint('.toBeCloseTo', 'received', 'expected, precision') +
          '\n\n' +
          `Expected value to be close to (within ${printExpected(ulps)} ulps):\n` +
          `  ${printExpected(y)}\n` +
          `Received:\n` +
          `  ${printReceived(x)}`
    );

    return { message, pass };
  },
};

declare global {
  namespace jest {
    interface Matchers<R> {
      toBeCloseTo(y: number | Phasor, e?: number): R;
    }
  }
}
