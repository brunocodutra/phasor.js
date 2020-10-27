import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have a logarithm', () => {
    samples.forEach(({ mag, ang }) => {
      const r = rect(Math.log10(mag), Math.atan2(Math.sin(ang), Math.cos(ang)) / Math.LN10);
      const u = polar(mag, ang);
      expect(u.log(10)).toBeCloseTo(r);
    });
  });
});
