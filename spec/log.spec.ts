import {log, polar, rect} from 'index';

import {samples} from './util';

const normalize = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));

describe('Complex', () => {
  it('should have a logarithm', () => {
    samples.forEach(({mag, ang}) => {
      if (isFinite(mag)) {
        const r = rect(Math.log(mag), normalize(ang));
        const u = polar(mag, ang);
        expect(log(u)).toBeCloseTo(r);
      }
    });

    expect(log(polar(NaN))).toBeNaN();
    expect(log(polar(0, NaN))).toBeNaN();
    expect(log(polar(NaN, NaN))).toBeNaN();
  });
});
