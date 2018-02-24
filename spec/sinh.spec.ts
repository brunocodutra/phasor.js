import {div, polar, rect, sinh, sub} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a hyperbolic sine', () => {
    samples.forEach(({mag, ang}) => {
      if (isFinite(mag)) {
        const s = polar(mag, ang);
        const u = rect(Math.log(mag), ang);
        const r = sub(div(s, rect(2)), div(rect(0.5), s));
        expect(sinh(u)).toBeCloseTo(r);
      }
    });

    expect(sinh(rect(NaN))).toBeNaN();
    expect(sinh(rect(0, NaN))).toBeNaN();
    expect(sinh(rect(NaN, NaN))).toBeNaN();
  });
});
