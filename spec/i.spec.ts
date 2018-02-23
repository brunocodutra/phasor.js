import {i, rect} from 'index';

import {samples} from './util';

describe('Complex', () => {
  it('should have an imaginary unit', () => {
    samples.forEach(({mag}) => {
      expect(i(mag)).toEqual(rect(0, mag));
    });

    expect(i()).toEqual(i(1));
    expect(i(NaN)).toBeNaN();
  });
});
