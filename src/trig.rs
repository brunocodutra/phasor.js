use core::f64::consts::FRAC_1_SQRT_2;
use core::num::FpCategory::{Infinite, Zero};

#[inline(always)]
pub(crate) fn sinatan(x: f64) -> f64 {
    sinatan2(x, 1f64)
}

#[inline(always)]
pub(crate) fn cosatan(x: f64) -> f64 {
    cosatan2(x, 1f64)
}

pub(crate) fn sinatan2(s: f64, c: f64) -> f64 {
    match (s.classify(), c.classify()) {
        (Zero, Zero) => s,
        (Infinite, Infinite) => FRAC_1_SQRT_2.copysign(s),
        _ => {
            if s.abs() > c.abs() {
                1f64.hypot(c / s).recip().copysign(s)
            } else {
                (s / c).copysign(s) / 1f64.hypot(s / c)
            }
        }
    }
}

pub(crate) fn cosatan2(s: f64, c: f64) -> f64 {
    match (s.classify(), c.classify()) {
        (Zero, Zero) => c.signum(),
        (Infinite, Infinite) => FRAC_1_SQRT_2.copysign(c),
        _ => {
            if s.abs() > c.abs() {
                (c / s).copysign(c) / 1f64.hypot(c / s)
            } else {
                1f64.hypot(s / c).recip().copysign(c)
            }
        }
    }
}

pub(crate) fn tanaddatan(x: f64, y: f64) -> (f64, f64) {
    let xs = x.abs().min(1f64) * x.signum();
    let xu = x.abs().max(1f64);

    let ys = y.abs().min(1f64) * y.signum();
    let yu = y.abs().max(1f64);

    (xs / yu + ys / xu, xu.recip() * yu.recip() - xs * ys)
}

pub(crate) fn tansubatan(x: f64, y: f64) -> (f64, f64) {
    let xs = x.abs().min(1f64) * x.signum();
    let xu = x.abs().max(1f64);

    let ys = y.abs().min(1f64) * y.signum();
    let yu = y.abs().max(1f64);

    (xs / yu - ys / xu, xu.recip() * yu.recip() + xs * ys)
}

pub(crate) fn cossubatan(x: f64, y: f64) -> f64 {
    let (s, c) = tansubatan(x, y);
    cosatan2(s, c)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::assert_close_to;
    use core::f64::consts::FRAC_PI_2;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn sinatan_equals_sine_of_atan(x in not_nan()) {
            assert_close_to!(sinatan(x), x.atan().sin());
        }

        #[test]
        fn sinatan_of_nan_is_nan(x in nan()) {
            assert!(sinatan(x).is_nan());
        }

        #[test]
        fn cosatan_equals_cosine_of_atan(x in not_nan()) {
            assert_close_to!(cosatan(x), x.atan().cos());
        }

        #[test]
        fn cosatan_of_nan_is_nan(x in nan()) {
            assert!(cosatan(x).is_nan());
        }

        #[test]
        fn sinatan2_equals_sine_of_atan2(x in not_nan(), y in not_nan()) {
            assert_close_to!(sinatan2(x, y), x.atan2(y).sin());
        }

        #[test]
        fn sinatan2_of_nan_is_nan(x in nan(), y in any()) {
            assert!(sinatan2(x, y).is_nan());
            assert!(sinatan2(y, x).is_nan());
        }

        #[test]
        fn cosatan2_equals_cosine_of_atan2(x in not_nan(), y in not_nan()) {
            assert_close_to!(cosatan2(x, y), x.atan2(y).cos());
        }

        #[test]
        fn cosatan2_of_nan_is_nan(x in nan(), y in any()) {
            assert!(cosatan2(x, y).is_nan());
            assert!(cosatan2(y, x).is_nan());
        }

        #[test]
        fn tanaddatan_equals_tangent_of_the_sum_of_atan(x in modulo(FRAC_PI_2), y in modulo(FRAC_PI_2)) {
            let (s, c) = tanaddatan(x.tan(), y.tan());
            assert_close_to!(s.atan2(c), x + y);
        }

        #[test]
        fn tanaddatan_of_nan_is_nan(x in nan(), y in any()) {
            let (s, c) = tanaddatan(x.tan(), y.tan());

            assert!(s.is_nan());
            assert!(c.is_nan());

            let (s, c) = tanaddatan(y.tan(), x.tan());

            assert!(s.is_nan());
            assert!(c.is_nan());
        }

        #[test]
        fn tansubatan_equals_tangent_of_the_difference_of_atan(x in modulo(FRAC_PI_2), y in modulo(FRAC_PI_2)) {
            let (s, c) = tansubatan(x.tan(), y.tan());
            assert_close_to!(s.atan2(c), x - y);
        }

        #[test]
        fn tansubatan_of_nan_is_nan(x in nan(), y in any()) {
            let (s, c) = tansubatan(x.tan(), y.tan());

            assert!(s.is_nan());
            assert!(c.is_nan());

            let (s, c) = tansubatan(y.tan(), x.tan());

            assert!(s.is_nan());
            assert!(c.is_nan());
        }

        #[test]
        fn cossubatan_equals_cosine_of_the_difference_of_atan(x in modulo(FRAC_PI_2), y in modulo(FRAC_PI_2)) {
            assert_close_to!(cossubatan(x.tan(), y.tan()), (x - y).cos());
        }

        #[test]
        fn cossubatan_of_nan_is_nan(x in nan(), y in any()) {
            assert!(cossubatan(x, y).is_nan());
            assert!(cossubatan(y, x).is_nan());
        }
    }
}
