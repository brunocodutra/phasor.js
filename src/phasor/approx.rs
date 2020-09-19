use super::Phasor;
use crate::trig::{cosatan2, cossubatan, sinatan2};
use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[allow(clippy::float_cmp)]
fn similarity(p: &Phasor, q: &Phasor) -> f64 {
    let k = if p.mag.abs() == q.mag.abs() {
        p.mag.signum() * q.mag.signum()
    } else {
        sinatan2(p.mag, q.mag) * cosatan2(p.mag, q.mag) * 2f64
    };

    k * cossubatan(p.tan, q.tan)
}

impl AbsDiffEq for Phasor {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, e: Self::Epsilon) -> bool {
        similarity(self, other).abs_diff_eq(&1f64, e)
    }
}

impl RelativeEq for Phasor {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, e: Self::Epsilon, max: Self::Epsilon) -> bool {
        similarity(self, other).relative_eq(&1f64, e, max)
    }
}

impl UlpsEq for Phasor {
    fn default_max_ulps() -> u32 {
        Self::Epsilon::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, e: Self::Epsilon, max: u32) -> bool {
        similarity(self, other).ulps_eq(&1f64, e, max)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::*;
    use core::f64::EPSILON;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn abs_diff_eq_nan(nan in nan(), not_nan in not_nan()) {
            let p = Phasor { mag: nan, tan: nan };
            assert!(!p.abs_diff_eq(&p, 1f64));

            let p = Phasor { mag: nan, tan: not_nan };
            assert!(!p.abs_diff_eq(&p, 1f64));

            let p = Phasor { mag: not_nan, tan: nan };
            assert!(!p.abs_diff_eq(&p, 1f64));
        }

        #[test]
        fn abs_diff_eq_same(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.abs_diff_eq(&p, EPSILON));
        }

        #[test]
        fn abs_diff_eq_real(mag in nonzero(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.abs_diff_eq(&p, EPSILON));

            let q = Phasor { mag, tan: -tan };
            assert!(p.abs_diff_eq(&q, EPSILON));

            let q = Phasor { mag: -mag, tan };
            assert!(!p.abs_diff_eq(&q, 1f64));

            let q = Phasor { mag: -mag, tan: -tan };
            assert!(!p.abs_diff_eq(&q, 1f64));
        }

        #[test]
        fn abs_diff_eq_imaginary(mag in nonzero(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert!(p.abs_diff_eq(&p, EPSILON));

            let q = Phasor { mag, tan: -tan };
            assert!(!p.abs_diff_eq(&q, 1f64));

            let q = Phasor { mag: -mag, tan };
            assert!(!p.abs_diff_eq(&q, 1f64));

            let q = Phasor { mag: -mag, tan: -tan };
            assert!(p.abs_diff_eq(&q, EPSILON));
        }

        #[test]
        fn abs_diff_eq_scaled(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag: mag * 1.000001, tan };
            let q = Phasor { mag: mag / 1.000001, tan };

            assert!(p.abs_diff_eq(&q, 1E-10));
            assert!(!p.abs_diff_eq(&q, 1E-12));
        }

        #[test]
        fn abs_diff_eq_opposite(mag in nonzero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: -mag, tan };

            assert!(!p.abs_diff_eq(&q, 1f64));
        }

        #[test]
        fn relative_eq_nan(nan in nan(), not_nan in not_nan()) {
            let p = Phasor { mag: nan, tan: nan };
            assert!(!p.relative_eq(&p, 0f64, 1f64));

            let p = Phasor { mag: nan, tan: not_nan };
            assert!(!p.relative_eq(&p, 0f64, 1f64));

            let p = Phasor { mag: not_nan, tan: nan };
            assert!(!p.relative_eq(&p, 0f64, 1f64));
        }

        #[test]
        fn relative_eq_same(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.relative_eq(&p, 0f64, EPSILON));
        }

        #[test]
        fn relative_eq_real(mag in nonzero(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.relative_eq(&p, 0f64, EPSILON));

            let q = Phasor { mag, tan: -tan };
            assert!(p.relative_eq(&q, 0f64, EPSILON));

            let q = Phasor { mag: -mag, tan };
            assert!(!p.relative_eq(&q, 0f64, 1f64));

            let q = Phasor { mag: -mag, tan: -tan };
            assert!(!p.relative_eq(&q, 0f64, 1f64));
        }

        #[test]
        fn relative_eq_imaginary(mag in nonzero(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert!(p.relative_eq(&p, 0f64, EPSILON));

            let q = Phasor { mag, tan: -tan };
            assert!(!p.relative_eq(&q, 0f64, 1f64));

            let q = Phasor { mag: -mag, tan };
            assert!(!p.relative_eq(&q, 0f64, 1f64));

            let q = Phasor { mag: -mag, tan: -tan };
            assert!(p.relative_eq(&q, 0f64, EPSILON));
        }

        #[test]
        fn relative_eq_scaled(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag: mag * 1.000001, tan };
            let q = Phasor { mag: mag / 1.000001, tan };

            assert!(p.relative_eq(&q, 0f64, 1E-10));
            assert!(!p.relative_eq(&q, 0f64, 1E-12));
        }

        #[test]
        fn relative_eq_opposite(mag in nonzero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: -mag, tan };

            assert!(!p.relative_eq(&q, 0f64, 1f64));
        }

        #[test]
        fn ulps_eq_nan(nan in nan(), not_nan in not_nan()) {
            let p = Phasor { mag: nan, tan: nan };
            assert!(!p.ulps_eq(&p, 0f64, u32::MAX));

            let p = Phasor { mag: nan, tan: not_nan };
            assert!(!p.ulps_eq(&p, 0f64, u32::MAX));

            let p = Phasor { mag: not_nan, tan: nan };
            assert!(!p.ulps_eq(&p, 0f64, u32::MAX));
        }

        #[test]
        fn ulps_eq_same(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.ulps_eq(&p, 0f64, 2));
        }

        #[test]
        fn ulps_eq_real(mag in nonzero(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.ulps_eq(&p, 0f64, 2));

            let q = Phasor { mag, tan: -tan };
            assert!(p.ulps_eq(&q, 0f64, 2));

            let q = Phasor { mag: -mag, tan };
            assert!(!p.ulps_eq(&q, 0f64, u32::MAX));

            let q = Phasor { mag: -mag, tan: -tan };
            assert!(!p.ulps_eq(&q, 0f64, u32::MAX));
        }

        #[test]
        fn ulps_eq_imaginary(mag in nonzero(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert!(p.ulps_eq(&p, 0f64, 2));

            let q = Phasor { mag, tan: -tan };
            assert!(!p.ulps_eq(&q, 0f64, u32::MAX));

            let q = Phasor { mag: -mag, tan };
            assert!(!p.ulps_eq(&q, 0f64, u32::MAX));

            let q = Phasor { mag: -mag, tan: -tan };
            assert!(p.ulps_eq(&q, 0f64, 2));
        }

        #[test]
        fn ulps_eq_scaled(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag: mag * 1.000001, tan };
            let q = Phasor { mag: mag / 1.000001, tan };

            assert!(p.ulps_eq(&q, 0f64, 100000));
            assert!(!p.ulps_eq(&q, 0f64, 1000));
        }

        #[test]
        fn ulps_eq_opposite(mag in nonzero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = Phasor { mag: -mag, tan };

            assert!(!p.ulps_eq(&q, 0f64, u32::MAX));
        }
    }
}
