import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a complement', () => {
    samples.forEach(({ re, im, mag, ang }) => {
      expect(rect(re, im).neg()).toBeCloseTo(rect(-re, -im));
      expect(polar(mag, ang).neg()).toBeCloseTo(polar(-mag, ang));
    });
  });
});
