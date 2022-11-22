import { polar, rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should have predicate to check for NaN', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re, im).isNaN()).toBe(isNaN(re) || isNaN(im));
    });
  });

  it('should have predicate to check whether infinite', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re, im).isInfinite()).toBe(Math.abs(re) === Infinity || Math.abs(im) === Infinity);
    });
  });

  it('should have predicate to check whether finite', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re, im).isFinite()).toBe(
        !isNaN(re) && !isNaN(im) &&
        Math.abs(re) !== Infinity && Math.abs(im) !== Infinity
      );
    });
  });

  it('should have predicate to check for zero', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re, im).isZero()).toBe(re === 0 && im === 0);
    });
  });

  it('should have predicate to check whether subnormal', () => {
    samples.forEach(({ mag, ang }) => {
      expect(polar(mag, ang).isSubnormal()).toBe(polar(mag).isSubnormal());
    });
  });

  it('should have predicate to check whether normal', () => {
    samples.forEach(({ mag, ang }) => {
      expect(polar(mag, ang).isNormal()).toBe(polar(mag).isNormal());
    });
  });

  it('should have predicate to check whether real', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re, im).isReal()).toBe(im === 0);
    });
  });

  it('should have predicate to check whether imaginary', () => {
    samples.forEach(({ re, im }) => {
      expect(rect(re, im).isImaginary()).toBe(re === 0 && im !== 0);
    });
  });
});
