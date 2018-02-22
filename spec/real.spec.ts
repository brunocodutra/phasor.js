import {real, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a real part', () => {
    samples.forEach(({real: re, imag: im}) => {
      expect(real(rect(re))).toBeCloseTo(re);
      expect(real(rect(re, im))).toBeCloseTo(re);
    });

    expect(real(rect(NaN))).toBeNaN();
    expect(real(rect(0, NaN))).toBeNaN();
    expect(real(rect(NaN, NaN))).toBeNaN();
  });
});
