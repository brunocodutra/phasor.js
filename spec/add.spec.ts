import { rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should add', () => {
    samples.forEach(({ re: a, im: b }) => {
      samples.forEach(({ re: c, im: d }) => {
        const u = rect(a, b);
        const v = rect(c, d);

        if (!u.isCloseTo(v.neg(), 1E-15)) {
          const r = rect(a + c, b + d);
          expect(u.add(v)).toBeCloseTo(r, 8);
        }
      });
    });
  });
});
