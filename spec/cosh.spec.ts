import {add, cosh, div, polar, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a hyperbolic cosine', () => {
    samples.forEach(({mag, ang}) => {
      if (isFinite(mag)) {
        const s = polar(mag, ang);
        const u = rect(Math.log(mag), ang);
        const r = add(div(s, rect(2)), div(rect(0.5), s));
        expect(cosh(u)).toBeCloseTo(r);
      }
    });

    expect(cosh(rect(NaN))).toBeNaN();
    expect(cosh(rect(0, NaN))).toBeNaN();
    expect(cosh(rect(NaN, NaN))).toBeNaN();
  });
});
