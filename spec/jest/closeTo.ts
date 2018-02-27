import {matcherHint, printExpected, printReceived} from 'jest-matcher-utils';

import {isComplex, Complex, equal} from 'index';

export default {
  toBeCloseTo(x: number | Complex, y: number | Complex, e = 1E-6) {
    const pass = (
        (typeof x === 'number' && typeof y === 'number')
      ? (x === y) || (Math.abs(x - y) < e) || (Math.abs(x - y) / Math.hypot(x, y)) < e
      : (isComplex(x) && isComplex(y))
      ? equal(x, y, e)
      : false
    );

    const message = (
        pass
      ? () =>
        matcherHint('.not.toBeCloseTo', 'received', 'expected, precision') +
        '\n\n' +
        `Expected value not to be close to (with relative precision of ${printExpected(e)}):\n` +
        `  ${printExpected(y)}\n` +
        `Received:\n` +
        `  ${printReceived(x)}`
      : () =>
        matcherHint('.toBeCloseTo', 'received', 'expected, precision') +
        '\n\n' +
        `Expected value to be close to (with relative precision of ${printExpected(e)}):\n` +
        `  ${printExpected(y)}\n` +
        `Received:\n` +
        `  ${printReceived(x)}`
    );

    return {message, pass};
  },
};

declare global {
  namespace jest {
    interface Matchers<R> {
      toBeCloseTo(y: number | Complex, e?: number): R;
    }
  }
}
