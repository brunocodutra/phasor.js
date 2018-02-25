import {mul, polar} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should multiply', () => {
    samples.forEach(({mag: a, ang: b}) => {
      samples.forEach(({mag: c, ang: d}) => {
        if ((isFinite(a) || c !== 0) && (a !== 0 || isFinite(c))) {
          const u = polar(a, b);
          const v = polar(c, d);
          const r = polar(a * c, b + d);
          expect(mul(u, v)).toBeCloseTo(r);
        }
      });

      expect(mul(polar(a, b), polar(NaN))).toBeNaN();
      expect(mul(polar(a, b), polar(0, NaN))).toBeNaN();
      expect(mul(polar(a, b), polar(NaN, NaN))).toBeNaN();
    });
  });
});