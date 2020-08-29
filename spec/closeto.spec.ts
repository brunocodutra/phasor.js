import {closeTo, polar} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should be comparable for approximate equality', () => {
    samples.forEach(({ mag, ang}) => {
      expect(closeTo(polar(0, ang), polar(0))).toBe(true);

      expect(closeTo(polar(mag, ang - Math.PI), polar(-mag, ang), 1E-6)).toBe(true);
      expect(closeTo(polar(mag, ang - Math.PI / 2), polar(-mag, ang), 1E-6)).toBe(!mag);
      expect(closeTo(polar(mag, ang + Math.PI / 2), polar(-mag, ang), 1E-6)).toBe(!mag);
      expect(closeTo(polar(mag, ang + Math.PI), polar(-mag, ang), 1E-6)).toBe(true);
    });

    expect(closeTo(polar(NaN), polar(NaN))).toEqual(false);
    expect(closeTo(polar(0, NaN), polar(0, NaN))).toEqual(false);
    expect(closeTo(polar(NaN, NaN), polar(NaN, NaN))).toEqual(false);
  });
});
