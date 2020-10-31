import { polar } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a reciprocate', () => {
    samples.forEach(({ mag, ang }) => {
      expect(polar(mag, ang).recip()).toBeCloseTo(polar(1 / mag, -ang));
    });
  });
});
