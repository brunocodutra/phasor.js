import {ln, polar, rect} from 'index';

import {samples} from './util';

const normalize = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));

describe('Complex', () => {
  it('should have a logarithm', () => {
    samples.forEach(({mag, ang}) => {
      if (isFinite(mag)) {
        const r = rect(Math.log(mag), normalize(ang));
        const u = polar(mag, ang);
        expect(ln(u)).toBeCloseTo(r);
      }
    });

    expect(ln(polar(NaN))).toBeNaN();
    expect(ln(polar(0, NaN))).toBeNaN();
    expect(ln(polar(NaN, NaN))).toBeNaN();
  });
});
