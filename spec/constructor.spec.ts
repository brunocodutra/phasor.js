import { i, Phasor, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should be constructible given optional magnitude and tangent', () => {
    samples.forEach(({ re, im }) => {
      let p = rect(re, im);
      expect(new Phasor(p.mag, p.tan)).toBeCloseTo(p);
      expect(new Phasor(re)).toBeCloseTo(rect(re));
      expect(new Phasor(im, Infinity)).toBeCloseTo(i(im));
    });
  });
});
