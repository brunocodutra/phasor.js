import {equal, polar, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should be comparable for equality', () => {
    samples.forEach(({real, imag, mag, ang}) => {
      expect(equal(polar(mag, ang), rect(real, imag), 1E-15)).toBe(true);
    });

    expect(equal(polar(NaN), rect(NaN))).toEqual(false);
  });
});
