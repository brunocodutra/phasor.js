import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a hyperbolic sine', () => {
    samples.forEach(({ mag, ang }) => {
      const u = rect(Math.log(mag), ang);

      if (!u.isZero()) {
        const s = polar(mag, ang);
        const r = s.div(rect(2)).sub(rect(0.5).div(s));
        expect(u.sinh()).toBeCloseTo(r, 40);
      }
    });
  });
});
