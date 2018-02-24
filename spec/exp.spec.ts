import {exp, polar, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have an exponential', () => {
    samples.forEach(({mag, ang}) => {
      if (isFinite(mag)) {
        const r = polar(mag, ang);
        const u = rect(Math.log(mag), ang);
        expect(exp(u)).toBeCloseTo(r);
      }
    });

    expect(exp(rect(NaN))).toBeNaN();
    expect(exp(rect(0, NaN))).toBeNaN();
    expect(exp(rect(NaN, NaN))).toBeNaN();
  });
});
