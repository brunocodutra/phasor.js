import { polar } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a magnitude', () => {
    samples.forEach(({ mag, ang }) => {
      const r = Math.abs(mag);
      expect(polar(mag).norm()).toBeCloseTo(r);
      expect(polar(mag, ang).norm()).toBeCloseTo(r);
    });
  });
});
