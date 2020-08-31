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
        _ => 1f64.hypot(c / s).recip().copysign(s),
    }
}

pub(crate) fn cosatan2(s: f64, c: f64) -> f64 {
    match (s.classify(), c.classify()) {
        (Zero, Zero) => c.signum(),
        (Infinite, Infinite) => FRAC_1_SQRT_2.copysign(c),
        _ => 1f64.hypot(s / c).recip().copysign(c),
    }
}

pub(crate) fn tanaddatan(x: f64, y: f64) -> f64 {
    match (x.abs() <= 1f64, y.abs() <= 1f64) {
        (true, true) => (x + y) / x.mul_add(-y, 1f64),
        (false, true) => y.mul_add(x.recip(), 1f64) / (x.recip() - y),
        (true, false) => x.mul_add(y.recip(), 1f64) / (y.recip() - x),
        (false, false) => (x.recip() + y.recip()) / x.recip().mul_add(y.recip(), -1f64),
    }
}

pub(crate) fn tansubatan(x: f64, y: f64) -> f64 {
    match (x.abs() <= 1f64, y.abs() <= 1f64) {
        (true, true) => (x - y) / x.mul_add(y, 1f64),
        (false, true) => y.mul_add(-x.recip(), 1f64) / (x.recip() + y),
        (true, false) => x.mul_add(y.recip(), -1f64) / (y.recip() + x),
        (false, false) => (y.recip() - x.recip()) / x.recip().mul_add(y.recip(), 1f64),
    }
}

pub(crate) fn cossubatan(x: f64, y: f64) -> f64 {
    let tan = tansubatan(x, y);

    if x.signum() != tan.signum() && y.signum() == tan.signum() {
        -cosatan(tan)
    } else {
        cosatan(tan)
    }
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
            assert_close_to!(tanaddatan(x.tan(), y.tan()), (x + y).tan(), tol = 1E-10);
        }

        #[test]
        fn tanaddatan_of_nan_is_nan(x in nan(), y in any()) {
            assert!(tanaddatan(x, y).is_nan());
            assert!(tanaddatan(y, x).is_nan());
        }

        #[test]
        fn tansubatan_equals_tangent_of_the_difference_of_atan(x in modulo(FRAC_PI_2), y in modulo(FRAC_PI_2)) {
            assert_close_to!(tansubatan(x.tan(), y.tan()), (x - y).tan(), tol = 1E-10);
        }

        #[test]
        fn tansubatan_of_nan_is_nan(x in nan(), y in any()) {
            assert!(tansubatan(x, y).is_nan());
            assert!(tansubatan(y, x).is_nan());
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
