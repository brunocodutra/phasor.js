import {neg, polar, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a complement', () => {
    samples.forEach(({real: re, imag: im, mag, ang}) => {
      expect(neg(rect(re, im))).toBeCloseTo(rect(-re, -im));
      expect(neg(polar(mag, ang))).toBeCloseTo(polar(-mag, ang));
    });

    expect(neg(rect(NaN))).toBeNaN();
    expect(neg(rect(0, NaN))).toBeNaN();
    expect(neg(rect(NaN, NaN))).toBeNaN();
  });
});
