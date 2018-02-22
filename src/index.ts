export type Complex = {
  readonly mag: number,
  readonly tan: number,
};

export const isComplex = (p: any): p is Complex => (
  typeof p === 'object' &&
  'mag' in p && typeof p.mag === 'number' &&
  'tan' in p && typeof p.tan === 'number'
);

const complex = (mag: number, tan: number): Complex => ({
  mag: isNaN(tan) ? NaN : mag,
  tan: isNaN(mag) ? NaN : tan,
});

export const polar = (mag: number, ang = 0): Complex => {
  ang %= 2 * Math.PI;
  mag *= (
    (ang > Math.PI / 2 && ang <= 3 * Math.PI / 2) ||
    (ang < -Math.PI / 2 && ang >= -3 * Math.PI / 2)
  ) ? -1 : 1;

  return complex(mag, Math.tan(ang));
};

export const rect = (re: number, im = 0): Complex => complex(
  (re < 0) ? -Math.hypot(re, im) : Math.hypot(re, im),
  (Math.abs(re) === Math.abs(im)) ? Math.sign(im * re) : (re === 0) ? im / 0 : im / re,
);

export const norm = (p: Complex): number => Math.abs(p.mag);
export const angle = (p: Complex): number => Math.atan(p.tan) + (
  (p.mag < 0) ? (p.tan > 0) ? -Math.PI : Math.PI : 0
);

export const real = (p: Complex): number => (1 / p.tan) && (p.mag / Math.hypot(1, p.tan));
export const imag = (p: Complex): number => Math.sign(p.tan) * real({mag: p.mag, tan: 1 / p.tan});

export const neg = (p: Complex): Complex => complex(-p.mag, p.tan);
export const conj = (p: Complex): Complex => complex(p.mag, -p.tan);

export const add = (p: Complex, q: Complex): Complex => {
  return rect(real(p) + real(q), imag(p) + imag(q));
};

export const sub = (p: Complex, q: Complex): Complex => {
  return rect(real(p) - real(q), imag(p) - imag(q));
};
