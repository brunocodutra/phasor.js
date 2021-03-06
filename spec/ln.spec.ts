import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a natural logarithm', () => {
    samples.forEach(({ mag, ang }) => {
      const r = rect(Math.log(mag), Math.atan2(Math.sin(ang), Math.cos(ang)));
      const u = polar(mag, ang);
      expect(u.ln()).toBeCloseTo(r);
    });
  });
});
