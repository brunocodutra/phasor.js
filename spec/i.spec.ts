import { i, polar } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have an imaginary unit', () => {
    samples.forEach(({ im }) => {
      expect(i(im)).toBeCloseTo(polar(im, Math.PI / 2));
    });
  });
});
