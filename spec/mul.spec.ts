import { polar } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should multiply', () => {
    samples.forEach(({ mag: a, ang: b }) => {
      samples.forEach(({ mag: c, ang: d }) => {
        const u = polar(a, b);
        const v = polar(c, d);
        const r = polar(a * c, b + d);
        expect(u.mul(v)).toBeCloseTo(r, 4);
      });
    });
  });
});
