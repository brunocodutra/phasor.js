use super::Phasor;
use crate::trig::{cosatan, sinatan};
use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

fn eq(p: &Phasor, q: &Phasor, cmp: impl Fn(f64, f64) -> bool) -> bool {
    let sign = p.mag.signum() * q.mag.signum();
    let (sp, cp) = (sinatan(p.tan), cosatan(p.tan));
    let (sq, cq) = (sinatan(q.tan), cosatan(q.tan));

    cmp(p.mag.abs(), q.mag.abs()) && cmp(sp * cq, cp * sq) && (sp * sq + cp * cq).signum() == sign
}

impl AbsDiffEq for Phasor {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, e: Self::Epsilon) -> bool {
        eq(self, other, move |x, y| x.abs_diff_eq(&y, e))
    }
}

impl RelativeEq for Phasor {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, e: Self::Epsilon, max: Self::Epsilon) -> bool {
        eq(self, other, move |x, y| x.relative_eq(&y, e, max))
    }
}

impl UlpsEq for Phasor {
    fn default_max_ulps() -> u32 {
        Self::Epsilon::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, e: Self::Epsilon, max: u32) -> bool {
        eq(self, other, move |x, y| x.ulps_eq(&y, e, max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::*;
    use crate::trig::tansubatan;
    use approx::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn abs_diff_eq_scaled(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag: mag * 1.0000000001, tan };
            let q = Phasor { mag: mag / 1.0000000001, tan };

            assert_abs_diff_eq!(p, q, epsilon = mag.abs() * 1E-9);
            assert_abs_diff_ne!(p, q, epsilon = mag.abs() * 1E-11);
        }

        #[test]
        fn abs_diff_eq_rotated(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };

            let (s, c) = tansubatan(tan, 1E-14 * tan.signum());
            let q = Phasor { mag, tan: s / c };

            assert_abs_diff_eq!(p, q, epsilon = 1E-13);
            assert_abs_diff_ne!(p, q, epsilon = 1E-15);
        }

        #[test]
        fn abs_diff_eq_same(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_abs_diff_eq!(p, p);
        }

        #[test]
        fn abs_diff_eq_opposite(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: -mag, tan };

            assert_abs_diff_ne!(p, q);
        }

        #[test]
        fn abs_diff_eq_orthogonal(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag, tan: -tan.recip() };

            assert_abs_diff_ne!(p, q);
        }

        #[test]
        fn abs_diff_eq_small(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: 0f64.copysign(mag), tan };
            assert_abs_diff_eq!(p, q, epsilon = mag.abs());
        }

        #[test]
        fn abs_diff_eq_real(mag in finite(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert_abs_diff_eq!(p, p);

            let q = Phasor { mag, tan: -tan };
            assert_abs_diff_eq!(p, q);

            let q = Phasor { mag: -mag, tan };
            assert_abs_diff_ne!(p, q);

            let q = Phasor { mag: -mag, tan: -tan };
            assert_abs_diff_ne!(p, q);
        }

        #[test]
        fn abs_diff_eq_imaginary(mag in finite(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert_abs_diff_eq!(p, p);

            let q = Phasor { mag, tan: -tan };
            assert_abs_diff_ne!(p, q);

            let q = Phasor { mag: -mag, tan };
            assert_abs_diff_ne!(p, q);

            let q = Phasor { mag: -mag, tan: -tan };
            assert_abs_diff_eq!(p, q);
        }

        #[test]
        fn abs_diff_eq_nan(nan in nan(), not_nan in not_nan()) {
            let p = Phasor { mag: nan, tan: nan };
            assert_abs_diff_ne!(p, p, epsilon = f64::INFINITY);

            let p = Phasor { mag: nan, tan: not_nan };
            assert_abs_diff_ne!(p, p, epsilon = f64::INFINITY);

            let p = Phasor { mag: not_nan, tan: nan };
            assert_abs_diff_ne!(p, p, epsilon = f64::INFINITY);
        }

        #[test]
        fn relative_eq_scaled(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag: mag * 1.0000000001, tan };
            let q = Phasor { mag: mag / 1.0000000001, tan };

            assert_relative_eq!(p, q, epsilon = 0f64, max_relative = 1E-9);
            assert_relative_ne!(p, q, epsilon = 0f64, max_relative = 1E-11);
        }

        #[test]
        fn relative_eq_rotated(mag in not_nan(), tan in regular()) {
            let p = Phasor { mag, tan: tan * 1.0000000001 };
            let q = Phasor { mag, tan: tan / 1.0000000001 };

            assert_relative_eq!(p, q, epsilon = 0f64, max_relative = 1E-9);
            assert_relative_ne!(p, q, epsilon = 0f64, max_relative = 1E-11);
        }

        #[test]
        fn relative_eq_same(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_relative_eq!(p, p);
        }

        #[test]
        fn relative_eq_opposite(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: -mag, tan };

            assert_relative_ne!(p, q);
        }

        #[test]
        fn relative_eq_orthogonal(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag, tan: -tan.recip() };

            assert_relative_ne!(p, q);
        }

        #[test]
        fn relative_eq_small(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: 0f64.copysign(mag), tan };
            assert_relative_eq!(p, q, epsilon = mag.abs());
        }

        #[test]
        fn relative_eq_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert_relative_eq!(p, p);

            let q = Phasor { mag, tan: -tan };
            assert_relative_eq!(p, q);

            let q = Phasor { mag: -mag, tan };
            assert_relative_ne!(p, q);

            let q = Phasor { mag: -mag, tan: -tan };
            assert_relative_ne!(p, q);
        }

        #[test]
        fn relative_eq_imaginary(mag in not_nan(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert_relative_eq!(p, p);

            let q = Phasor { mag, tan: -tan };
            assert_relative_ne!(p, q);

            let q = Phasor { mag: -mag, tan };
            assert_relative_ne!(p, q);

            let q = Phasor { mag: -mag, tan: -tan };
            assert_relative_eq!(p, q);
        }

        #[test]
        fn relative_eq_nan(nan in nan(), not_nan in not_nan()) {
            let p = Phasor { mag: nan, tan: nan };
            assert_relative_ne!(p, p, epsilon = f64::INFINITY, max_relative = f64::INFINITY);

            let p = Phasor { mag: nan, tan: not_nan };
            assert_relative_ne!(p, p, epsilon = f64::INFINITY, max_relative = f64::INFINITY);

            let p = Phasor { mag: not_nan, tan: nan };
            assert_relative_ne!(p, p, epsilon = f64::INFINITY, max_relative = f64::INFINITY);
        }

        #[test]
        fn ulps_eq_scaled(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag: mag * 1.0000000001, tan };
            let q = Phasor { mag: mag / 1.0000000001, tan };

            assert_ulps_eq!(p, q, epsilon = 0f64, max_ulps = 10_000_000);
            assert_ulps_ne!(p, q, epsilon = 0f64, max_ulps = 100_000);
        }

        #[test]
        fn ulps_eq_rotated(mag in not_nan(), tan in normal()) {
            let p = Phasor { mag, tan: tan * 1.0000000001 };
            let q = Phasor { mag, tan: tan / 1.0000000001 };

            assert_ulps_eq!(p, q, epsilon = 0f64, max_ulps = 10_000_000);
            assert_ulps_ne!(p, q, epsilon = 0f64, max_ulps = 100_000);
        }

        #[test]
        fn ulps_eq_same(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p, p);
        }

        #[test]
        fn ulps_eq_opposite(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: -mag, tan };

            assert_ulps_ne!(p, q);
        }

        #[test]
        fn ulps_eq_orthogonal(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag, tan: -tan.recip() };

            assert_ulps_ne!(p, q);
        }

        #[test]
        fn ulps_eq_small(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: 0f64.copysign(mag), tan };
            assert_ulps_eq!(p, q, epsilon = mag.abs());
        }

        #[test]
        fn ulps_eq_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p, p);

            let q = Phasor { mag, tan: -tan };
            assert_ulps_eq!(p, q);

            let q = Phasor { mag: -mag, tan };
            assert_ulps_ne!(p, q);

            let q = Phasor { mag: -mag, tan: -tan };
            assert_ulps_ne!(p, q);
        }

        #[test]
        fn ulps_eq_imaginary(mag in not_nan(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p, p);

            let q = Phasor { mag, tan: -tan };
            assert_ulps_ne!(p, q);

            let q = Phasor { mag: -mag, tan };
            assert_ulps_ne!(p, q);

            let q = Phasor { mag: -mag, tan: -tan };
            assert_ulps_eq!(p, q);
        }

        #[test]
        fn ulps_eq_nan(nan in nan(), not_nan in not_nan()) {
            let p = Phasor { mag: nan, tan: nan };
            assert_ulps_ne!(p, p, epsilon = f64::INFINITY, max_ulps = u32::MAX);

            let p = Phasor { mag: nan, tan: not_nan };
            assert_ulps_ne!(p, p, epsilon = f64::INFINITY, max_ulps = u32::MAX);

            let p = Phasor { mag: not_nan, tan: nan };
            assert_ulps_ne!(p, p, epsilon = f64::INFINITY, max_ulps = u32::MAX);
        }
    }
}
