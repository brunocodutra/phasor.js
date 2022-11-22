import { rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should be comparable by absolute difference', () => {
    samples.forEach(({ re: a, im: b }) => {
      samples.forEach(({ re: c, im: d }) => {
        expect(rect(a, b).absDiffEq(rect(c, d))).toBe(a === c && b == d);
      });
    });
  });

  it('should be comparable by relative difference', () => {
    samples.forEach(({ re: a, im: b }) => {
      samples.forEach(({ re: c, im: d }) => {
        expect(rect(a, b).relativeEq(rect(c, d))).toBe(a === c && b == d);
      });
    });
  });

  it('should be comparable by difference in units in the last place', () => {
    samples.forEach(({ re: a, im: b }) => {
      samples.forEach(({ re: c, im: d }) => {
        expect(rect(a, b).ulpsEq(rect(c, d))).toBe(a === c && b == d);
      });
    });
  });
});
