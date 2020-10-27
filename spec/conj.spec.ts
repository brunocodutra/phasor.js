import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a conjugate', () => {
    samples.forEach(({ re, im, mag, ang }) => {
      expect(rect(re, im).conj()).toBeCloseTo(rect(re, -im));
      expect(polar(mag, ang).conj()).toBeCloseTo(polar(mag, -ang));
    });
  });
});
