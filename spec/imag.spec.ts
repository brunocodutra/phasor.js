import { rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have an imaginary part', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re).imag()).toBeCloseTo(0);
      expect(rect(re, im).imag()).toBeCloseTo(im);
    });
  });
});
