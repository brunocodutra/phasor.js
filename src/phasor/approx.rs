use super::*;
use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

fn distance(p: &Phasor, q: &Phasor) -> f64 {
    use core::num::FpCategory::*;

    match (p.classify(), q.classify()) {
        (Zero, Zero) | (Infinite, Infinite) => cosatan(tansubatan(p.tan, q.tan)),
        _ => {
            2f64 * cosatan(tansubatan(p.tan, q.tan))
                * cosatan(p.mag / q.mag)
                * cosatan(q.mag / p.mag)
        }
    }
}

impl AbsDiffEq for Phasor {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, e: Self::Epsilon) -> bool {
        distance(self, other).abs_diff_eq(&1f64, e)
    }
}

impl RelativeEq for Phasor {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, e: Self::Epsilon, max: Self::Epsilon) -> bool {
        distance(self, other).relative_eq(&1f64, e, max)
    }
}

impl UlpsEq for Phasor {
    fn default_max_ulps() -> u32 {
        Self::Epsilon::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, e: Self::Epsilon, max: u32) -> bool {
        distance(self, other).ulps_eq(&1f64, e, max)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use proptest::{num::f64::NORMAL, prelude::*};

    proptest! {
        #[test]
        fn abs_diff_eq(a: f64, b: f64) {
            let p = Phasor::rect(a * 1.000001, b * 0.999999);
            let q = Phasor::rect(a * 0.999999, b * 1.000001);

            assert!(p.abs_diff_eq(&q, 1E-10));
        }

        #[test]
        fn abs_diff_neq(a in NORMAL, b in NORMAL) {
            let p = Phasor::rect(a * 1.000001, b * 0.999999);
            let q = Phasor::rect(a * 0.999999, b * 1.000001);

            assert!(!p.abs_diff_eq(&q, 1E-12));
        }

        #[test]
        fn relative_eq(a: f64, b: f64) {
            let p = Phasor::rect(a * 1.000001, b * 0.999999);
            let q = Phasor::rect(a * 0.999999, b * 1.000001);

            assert!(p.relative_eq(&q, 0f64, 1E-10));
        }

        #[test]
        fn relative_neq(a in NORMAL, b in NORMAL) {
            let p = Phasor::rect(a * 1.000001, b * 0.999999);
            let q = Phasor::rect(a * 0.999999, b * 1.000001);

            assert!(!p.relative_eq(&q, 0f64, 1E-12));
        }

        #[test]
        fn ulps_eq(a: f64, b: f64) {
            let p = Phasor::rect(a * 1.000001, b * 0.999999);
            let q = Phasor::rect(a * 0.999999, b * 1.000001);

            assert!(p.ulps_eq(&q, 0f64, 100000));
        }

        #[test]
        fn ulps_neq(a in NORMAL, b in NORMAL) {
            let p = Phasor::rect(a * 1.000001, b * 0.999999);
            let q = Phasor::rect(a * 0.999999, b * 1.000001);

            assert!(!p.ulps_eq(&q, 0f64, 1000));
        }
    }
}
