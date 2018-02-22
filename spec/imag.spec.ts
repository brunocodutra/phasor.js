import {imag, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have an imaginary part', () => {
    samples.forEach(({real: re, imag: im}) => {
      expect(imag(rect(re))).toBeCloseTo(0);
      expect(imag(rect(re, im))).toBeCloseTo(im);
    });

    expect(imag(rect(NaN))).toBeNaN();
    expect(imag(rect(0, NaN))).toBeNaN();
    expect(imag(rect(NaN, NaN))).toBeNaN();
  });
});
