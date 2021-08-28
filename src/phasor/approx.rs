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
    use crate::{arbitrary::*, trig::tansubatan};
    use approx::*;
    use test_strategy::proptest;

    #[proptest]
    fn abs_diff_eq_scaled(#[strategy(normal())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor {
            mag: mag * 1.0000000001,
            tan,
        };
        let q = Phasor {
            mag: mag / 1.0000000001,
            tan,
        };

        assert_abs_diff_eq!(p, q, epsilon = mag.abs() * 1E-9);
        assert_abs_diff_ne!(p, q, epsilon = mag.abs() * 1E-11);
    }

    #[proptest]
    fn abs_diff_eq_rotated(#[strategy(regular())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };

        let (s, c) = tansubatan(tan, 1E-14 * tan.signum());
        let q = Phasor { mag, tan: s / c };

        assert_abs_diff_eq!(p, q, epsilon = 1E-13);
        assert_abs_diff_ne!(p, q, epsilon = 1E-15);
    }

    #[proptest]
    fn abs_diff_eq_same(#[strategy(finite())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_abs_diff_eq!(p, p);
    }

    #[proptest]
    fn abs_diff_eq_opposite(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor { mag: -mag, tan };

        assert_abs_diff_ne!(p, q);
    }

    #[proptest]
    fn abs_diff_eq_orthogonal(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor {
            mag,
            tan: -tan.recip(),
        };

        assert_abs_diff_ne!(p, q);
    }

    #[proptest]
    fn abs_diff_eq_small(#[strategy(regular())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor {
            mag: 0f64.copysign(mag),
            tan,
        };
        assert_abs_diff_eq!(p, q, epsilon = mag.abs());
    }

    #[proptest]
    fn abs_diff_eq_real(#[strategy(finite())] mag: f64, #[strategy(zero())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_abs_diff_eq!(p, p);

        let q = Phasor { mag, tan: -tan };
        assert_abs_diff_eq!(p, q);

        let q = Phasor { mag: -mag, tan };
        assert_abs_diff_ne!(p, q);

        let q = Phasor {
            mag: -mag,
            tan: -tan,
        };
        assert_abs_diff_ne!(p, q);
    }

    #[proptest]
    fn abs_diff_eq_imaginary(#[strategy(finite())] mag: f64, #[strategy(infinite())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_abs_diff_eq!(p, p);

        let q = Phasor { mag, tan: -tan };
        assert_abs_diff_ne!(p, q);

        let q = Phasor { mag: -mag, tan };
        assert_abs_diff_ne!(p, q);

        let q = Phasor {
            mag: -mag,
            tan: -tan,
        };
        assert_abs_diff_eq!(p, q);
    }

    #[proptest]
    fn abs_diff_eq_nan(#[strategy(nan())] nan: f64, #[strategy(not_nan())] not_nan: f64) {
        let p = Phasor { mag: nan, tan: nan };
        assert_abs_diff_ne!(p, p, epsilon = f64::INFINITY);

        let p = Phasor {
            mag: nan,
            tan: not_nan,
        };
        assert_abs_diff_ne!(p, p, epsilon = f64::INFINITY);

        let p = Phasor {
            mag: not_nan,
            tan: nan,
        };
        assert_abs_diff_ne!(p, p, epsilon = f64::INFINITY);
    }

    #[proptest]
    fn relative_eq_scaled(#[strategy(normal())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor {
            mag: mag * 1.0000000001,
            tan,
        };
        let q = Phasor {
            mag: mag / 1.0000000001,
            tan,
        };

        assert_relative_eq!(p, q, epsilon = 0f64, max_relative = 1E-9);
        assert_relative_ne!(p, q, epsilon = 0f64, max_relative = 1E-11);
    }

    #[proptest]
    fn relative_eq_rotated(#[strategy(not_nan())] mag: f64, #[strategy(regular())] tan: f64) {
        let p = Phasor {
            mag,
            tan: tan * 1.0000000001,
        };
        let q = Phasor {
            mag,
            tan: tan / 1.0000000001,
        };

        assert_relative_eq!(p, q, epsilon = 0f64, max_relative = 1E-9);
        assert_relative_ne!(p, q, epsilon = 0f64, max_relative = 1E-11);
    }

    #[proptest]
    fn relative_eq_same(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_relative_eq!(p, p);
    }

    #[proptest]
    fn relative_eq_opposite(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor { mag: -mag, tan };

        assert_relative_ne!(p, q);
    }

    #[proptest]
    fn relative_eq_orthogonal(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor {
            mag,
            tan: -tan.recip(),
        };

        assert_relative_ne!(p, q);
    }

    #[proptest]
    fn relative_eq_small(#[strategy(regular())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor {
            mag: 0f64.copysign(mag),
            tan,
        };
        assert_relative_eq!(p, q, epsilon = mag.abs());
    }

    #[proptest]
    fn relative_eq_real(#[strategy(not_nan())] mag: f64, #[strategy(zero())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_relative_eq!(p, p);

        let q = Phasor { mag, tan: -tan };
        assert_relative_eq!(p, q);

        let q = Phasor { mag: -mag, tan };
        assert_relative_ne!(p, q);

        let q = Phasor {
            mag: -mag,
            tan: -tan,
        };
        assert_relative_ne!(p, q);
    }

    #[proptest]
    fn relative_eq_imaginary(#[strategy(not_nan())] mag: f64, #[strategy(infinite())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_relative_eq!(p, p);

        let q = Phasor { mag, tan: -tan };
        assert_relative_ne!(p, q);

        let q = Phasor { mag: -mag, tan };
        assert_relative_ne!(p, q);

        let q = Phasor {
            mag: -mag,
            tan: -tan,
        };
        assert_relative_eq!(p, q);
    }

    #[proptest]
    fn relative_eq_nan(#[strategy(nan())] nan: f64, #[strategy(not_nan())] not_nan: f64) {
        let p = Phasor { mag: nan, tan: nan };
        assert_relative_ne!(p, p, epsilon = f64::INFINITY, max_relative = f64::INFINITY);

        let p = Phasor {
            mag: nan,
            tan: not_nan,
        };
        assert_relative_ne!(p, p, epsilon = f64::INFINITY, max_relative = f64::INFINITY);

        let p = Phasor {
            mag: not_nan,
            tan: nan,
        };
        assert_relative_ne!(p, p, epsilon = f64::INFINITY, max_relative = f64::INFINITY);
    }

    #[proptest]
    fn ulps_eq_scaled(#[strategy(normal())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor {
            mag: mag * 1.0000000001,
            tan,
        };
        let q = Phasor {
            mag: mag / 1.0000000001,
            tan,
        };

        assert_ulps_eq!(p, q, epsilon = 0f64, max_ulps = 10_000_000);
        assert_ulps_ne!(p, q, epsilon = 0f64, max_ulps = 100_000);
    }

    #[proptest]
    fn ulps_eq_rotated(#[strategy(not_nan())] mag: f64, #[strategy(normal())] tan: f64) {
        let p = Phasor {
            mag,
            tan: tan * 1.0000000001,
        };
        let q = Phasor {
            mag,
            tan: tan / 1.0000000001,
        };

        assert_ulps_eq!(p, q, epsilon = 0f64, max_ulps = 10_000_000);
        assert_ulps_ne!(p, q, epsilon = 0f64, max_ulps = 100_000);
    }

    #[proptest]
    fn ulps_eq_same(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p, p);
    }

    #[proptest]
    fn ulps_eq_opposite(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor { mag: -mag, tan };

        assert_ulps_ne!(p, q);
    }

    #[proptest]
    fn ulps_eq_orthogonal(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor {
            mag,
            tan: -tan.recip(),
        };

        assert_ulps_ne!(p, q);
    }

    #[proptest]
    fn ulps_eq_small(#[strategy(regular())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let q = Phasor {
            mag: 0f64.copysign(mag),
            tan,
        };
        assert_ulps_eq!(p, q, epsilon = mag.abs());
    }

    #[proptest]
    fn ulps_eq_real(#[strategy(not_nan())] mag: f64, #[strategy(zero())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p, p);

        let q = Phasor { mag, tan: -tan };
        assert_ulps_eq!(p, q);

        let q = Phasor { mag: -mag, tan };
        assert_ulps_ne!(p, q);

        let q = Phasor {
            mag: -mag,
            tan: -tan,
        };
        assert_ulps_ne!(p, q);
    }

    #[proptest]
    fn ulps_eq_imaginary(#[strategy(not_nan())] mag: f64, #[strategy(infinite())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p, p);

        let q = Phasor { mag, tan: -tan };
        assert_ulps_ne!(p, q);

        let q = Phasor { mag: -mag, tan };
        assert_ulps_ne!(p, q);

        let q = Phasor {
            mag: -mag,
            tan: -tan,
        };
        assert_ulps_eq!(p, q);
    }

    #[proptest]
    fn ulps_eq_nan(#[strategy(nan())] nan: f64, #[strategy(not_nan())] not_nan: f64) {
        let p = Phasor { mag: nan, tan: nan };
        assert_ulps_ne!(p, p, epsilon = f64::INFINITY, max_ulps = u32::MAX);

        let p = Phasor {
            mag: nan,
            tan: not_nan,
        };
        assert_ulps_ne!(p, p, epsilon = f64::INFINITY, max_ulps = u32::MAX);

        let p = Phasor {
            mag: not_nan,
            tan: nan,
        };
        assert_ulps_ne!(p, p, epsilon = f64::INFINITY, max_ulps = u32::MAX);
    }
}
