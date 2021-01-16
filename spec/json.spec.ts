import { rect } from '../';
import { samples } from './util';

describe('Phasor', () => {
  it('should be serializable', () => {
    samples.forEach(({ re, im }) => {
      const p = rect(re, im);
      expect(p.toString()).toEqual(JSON.stringify(p));
      expect(JSON.parse(JSON.stringify(p))).toEqual({
        mag: p.mag,
        tan: p.tan,
      });
    });
  });
});
