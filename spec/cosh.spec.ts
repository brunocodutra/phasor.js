import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a hyperbolic cosine', () => {
    samples.forEach(({ mag, ang }) => {
      const s = polar(mag, ang);
      const u = rect(Math.log(mag), ang);
      const r = s.div(rect(2)).add(rect(0.5).div(s));
      expect(u.cosh()).toBeCloseTo(r, 40);
    });
  });
});
