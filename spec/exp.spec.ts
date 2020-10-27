import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have an exponential', () => {
    samples.forEach(({ mag, ang }) => {
      const r = polar(mag, ang);
      const u = rect(Math.log(mag), ang);
      expect(u.exp()).toBeCloseTo(r, 40);
    });
  });
});
