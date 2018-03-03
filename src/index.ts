export type Complex = {
  readonly mag: number,
  readonly tan: number,
};

export type Phasor = Complex;

export const isComplex = (p: any): p is Complex => (
  typeof p === 'object' &&
  'mag' in p && typeof p.mag === 'number' &&
  'tan' in p && typeof p.tan === 'number'
);

export const isPhasor = isComplex;

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

export const i = (im = 1) => rect(0, im);

const cosatan = (x: number) => 1 / Math.hypot(1, x);
const sinatan = (x: number) => Math.sign(x) * cosatan(1 / x);

const tanaddatan = (b: number, d: number) => {
  const absb = Math.abs(b);
  const absd = Math.abs(d);

  return (
      (absb === 0)
    ? d
    : (absd === 0)
    ? b
    : (absb <= 1 && absd <= 1)
    ? (b + d) / (1 - b * d)
    : (absb > 1 && absd <= 1)
    ? (1 + d / b) / (1 / b - d)
    : (absb <= 1 && absd > 1)
    ? (b / d + 1) / (1 / d - b)
    : (1 / d + 1 / b) / ((1 / b) * (1 / d) - 1)
  );
};

const tansubatan = (b: number, d: number) => {
  const absb = Math.abs(b);
  const absd = Math.abs(d);

  return (
      (absb === 0)
    ? -d
    : (absd === 0)
    ? b
    : (absb <= 1 && absd <= 1)
    ? (b - d) / (1 + b * d)
    : (absb > 1 && absd <= 1)
    ? (1 - d / b) / (1 / b + d)
    : (absb <= 1 && absd > 1)
    ? (b / d - 1) / (1 / d + b)
    : (1 / d - 1 / b) / ((1 / b) * (1 / d) + 1)
  );
};

const close = (x: number, y: number, e: number) => (
  (x === y) ||
  (Math.abs(x - y) <= Math.abs(e) ** 1.5) ||
  (Math.abs(x - y) / Math.hypot(x, y)) <= Math.abs(e) / 2 ** 0.5
);

export const equal = (p: Complex, q: Complex, e = 0) => {
  const {mag: a, tan: b} = p;
  const {mag: c, tan: d} = q;

  return (
      (close(b, 0, e) && close(d, 0, e))
    ? close(a, c, e)
    : (close(1 / b, 0, e) && close(1 / d, 0, e))
    ? close(a * Math.sign(b), c * Math.sign(d), e)
    : (Math.abs(a) === Infinity && Math.abs(c) === Infinity)
    ? close(sinatan(b) * sinatan(d) + cosatan(b) * cosatan(d), Math.sign(a * c), e ** 2)
    : close(a ** 2 + c ** 2, 2 * a * c * cosatan(tansubatan(b, d)), e ** 2)
  );
};

export const norm = (p: Complex): number => Math.abs(p.mag);
export const angle = (p: Complex): number => Math.atan(p.tan) + (
  (p.mag < 0) ? (p.tan > 0) ? -Math.PI : Math.PI : 0
);

export const real = (p: Complex): number => (1 / p.tan) && p.mag * cosatan(p.tan);
export const imag = (p: Complex): number => p.tan && p.mag * sinatan(p.tan);

export const neg = (p: Complex): Complex => complex(-p.mag, p.tan);
export const conj = (p: Complex): Complex => complex(p.mag, -p.tan);

export const add = (p: Complex, q: Complex): Complex => {
  return rect(real(p) + real(q), imag(p) + imag(q));
};

export const sub = (p: Complex, q: Complex): Complex => {
  return rect(real(p) - real(q), imag(p) - imag(q));
};

export const mul = (p: Complex, q: Complex): Complex => {
  const {mag: a, tan: b} = p;
  const {mag: c, tan: d} = q;

  const tan = tanaddatan(b, d);

  const mag = a * c * ((
    (b > 0 && d > 0 && tan <= 0) ||
    (b < 0 && d < 0 && tan >= 0)
  ) ? -1 : 1);

  return complex(mag, tan);
};

export const div = (p: Complex, q: Complex): Complex => {
  const {mag: a, tan: b} = p;
  const {mag: c, tan: d} = q;

  const tan = tansubatan(b, d);

  const mag = a / c * ((
    (b > 0 && d < 0 && tan <= 0) ||
    (b < 0 && d > 0 && tan >= 0)
  ) ? -1 : 1);

  return complex(mag, tan);
};

export const exp = (p: Complex): Complex => polar(Math.exp(real(p)), imag(p));
export const log = (p: Complex): Complex => rect(Math.log(norm(p)), angle(p));

export const sinh = (p: Complex): Complex => div(sub(exp(p), exp(neg(p))), rect(2));
export const cosh = (p: Complex): Complex => div(add(exp(p), exp(neg(p))), rect(2));
