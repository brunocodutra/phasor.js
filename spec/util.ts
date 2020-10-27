type Sample = { mag: number, ang: number, re: number, im: number };

export const samples: Sample[] = [
  { mag: 0, ang: 0, re: 0, im: 0 },
  [1E-12, 1E-9, 1E-6, 1E-3, 1, 1E3, 1E6, 1E9, 1E12].map((mag) =>
    Array(33).fill(Math.PI / 16).map((p, k) => {
      const ang = p * k - Math.PI;
      const re = Math.cos(ang) * mag;
      const im = Math.sin(ang) * mag;

      return { mag, ang, re, im };
    }),
  ),
].flat(2);
