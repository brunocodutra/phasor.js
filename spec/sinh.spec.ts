import {angle, div, norm, rect, sinh, sub} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a hyperbolic sine', () => {
    samples.forEach(({real, imag, mag}) => {
      if (isFinite(mag)) {
        const s = rect(real, imag);
        const u = rect(Math.log(norm(s)), angle(s));
        const r = sub(div(s, rect(2)), div(rect(0.5), s));
        expect(sinh(u)).toBeCloseTo(r);
      }
    });

    expect(sinh(rect(NaN))).toBeNaN();
    expect(sinh(rect(0, NaN))).toBeNaN();
    expect(sinh(rect(NaN, NaN))).toBeNaN();
  });
});
