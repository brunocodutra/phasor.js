import { rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should subtract', () => {
    samples.forEach(({ re: a, im: b }) => {
      samples.forEach(({ re: c, im: d }) => {
        const u = rect(a, b);
        const v = rect(c, d);

        if (!u.isCloseTo(v, 1E-15)) {
          const r = rect(a - c, b - d);
          expect(u.sub(v)).toBeCloseTo(r, 8);
        }
      });
    });
  });
});
