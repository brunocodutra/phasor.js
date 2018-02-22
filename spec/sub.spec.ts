import {rect, sub} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should subtract', () => {
    samples.forEach(({real: a, imag: b}) => {
      samples.forEach(({real: c, imag: d}) => {
        if ((isFinite(a) || isFinite(c) || a !== c) && (isFinite(b) || isFinite(d) || b !== d)) {
          const u = rect(a, b);
          const v = rect(c, d);
          const r = rect(a - c, b - d);
          expect(sub(u, v)).toBeCloseTo(r);
        }
      });

      expect(sub(rect(a, b), rect(NaN))).toBeNaN();
      expect(sub(rect(a, b), rect(0, NaN))).toBeNaN();
      expect(sub(rect(a, b), rect(NaN, NaN))).toBeNaN();
    });
  });
});
