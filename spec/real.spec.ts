import { rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a real part', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re).real()).toBeCloseTo(re);
      expect(rect(re, im).real()).toBeCloseTo(re);
    });
  });
});
