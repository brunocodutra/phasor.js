import {angle, polar} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have an angle', () => {
    samples.forEach(({mag, ang}) => {
      const r = mag && Math.atan2(Math.sin(ang), Math.cos(ang));
      expect(angle(polar(mag))).toBeCloseTo(0);
      expect(angle(polar(mag, ang))).toBeCloseTo(r);
    });

    expect(angle(polar(NaN))).toBeNaN();
    expect(angle(polar(0, NaN))).toBeNaN();
    expect(angle(polar(NaN, NaN))).toBeNaN();
  });
});
