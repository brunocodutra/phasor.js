import {equal, polar} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should be comparable for equality', () => {
    samples.forEach(({ mag, ang}) => {
      expect(equal(polar(0, ang), polar(0))).toBe(true);

      expect(equal(polar(mag, ang - Math.PI), polar(-mag, ang), 1E-6)).toBe(true);
      expect(equal(polar(mag, ang - Math.PI / 2), polar(-mag, ang), 1E-6)).toBe(!mag);
      expect(equal(polar(mag, ang + Math.PI / 2), polar(-mag, ang), 1E-6)).toBe(!mag);
      expect(equal(polar(mag, ang + Math.PI), polar(-mag, ang), 1E-6)).toBe(true);
    });

    expect(equal(polar(NaN), polar(NaN))).toEqual(false);
    expect(equal(polar(0, NaN), polar(0, NaN))).toEqual(false);
    expect(equal(polar(NaN, NaN), polar(NaN, NaN))).toEqual(false);
  });
});
