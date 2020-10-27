import { polar } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should divide', () => {
    samples.forEach(({ mag: a, ang: b }) => {
      samples.forEach(({ mag: c, ang: d }) => {
        const u = polar(a, b);
        const v = polar(c, d);

        if (!u.isZero() || !v.isZero()) {
          const r = polar(a / c, b - d);
          expect(u.div(v)).toBeCloseTo(r, 4);
        }
      });
    });
  });
});
