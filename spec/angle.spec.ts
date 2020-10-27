import { polar } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have an angle', () => {
    samples.forEach(({ mag, ang }) => {
      const r = mag && Math.atan2(Math.sin(ang), Math.cos(ang));
      expect(polar(mag).angle()).toBeCloseTo(0);
      expect(polar(mag, ang).angle()).toBeCloseTo(r);
    });
  });
});
