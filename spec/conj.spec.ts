import {conj, polar, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a conjugate', () => {
    samples.forEach(({real: re, imag: im, mag, ang}) => {
      expect(conj(rect(re, im))).toBeCloseTo(rect(re, -im));
      expect(conj(polar(mag, ang))).toBeCloseTo(polar(mag, -ang));
    });

    expect(conj(rect(NaN))).toBeNaN();
    expect(conj(rect(0, NaN))).toBeNaN();
    expect(conj(rect(NaN, NaN))).toBeNaN();
  });
});
