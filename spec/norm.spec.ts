import {norm, polar} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have a magnitude', () => {
    samples.forEach(({mag, ang}) => {
      const r = Math.abs(mag);
      expect(norm(polar(mag))).toBeCloseTo(r);
      expect(norm(polar(mag, ang))).toBeCloseTo(r);
    });

    expect(norm(polar(NaN))).toBeNaN();
    expect(norm(polar(0, NaN))).toBeNaN();
    expect(norm(polar(NaN, NaN))).toBeNaN();
  });
});
