import {add, angle, cosh, div, norm, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a hyperbolic cosine', () => {
    samples.forEach(({real, imag, mag}) => {
      if (isFinite(mag)) {
        const s = rect(real, imag);
        const u = rect(Math.log(norm(s)), angle(s));
        const r = add(div(s, rect(2)), div(rect(0.5), s));
        expect(cosh(u)).toBeCloseTo(r);
      }
    });

    expect(cosh(rect(NaN))).toBeNaN();
    expect(cosh(rect(0, NaN))).toBeNaN();
    expect(cosh(rect(NaN, NaN))).toBeNaN();
  });
});
