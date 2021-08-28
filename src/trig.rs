use std::f64::consts::SQRT_2;

pub(crate) fn sinatan(x: f64) -> f64 {
    sinatan2(x, 1f64)
}

pub(crate) fn cosatan(x: f64) -> f64 {
    cosatan2(x, 1f64)
}

pub(crate) fn sinatan2(s: f64, c: f64) -> f64 {
    cosatan2(c, s).copysign(s)
}

pub(crate) fn cosatan2(s: f64, c: f64) -> f64 {
    if s.abs() > c.abs() {
        (c / s).copysign(c) / 1f64.hypot(c / s)
    } else if s.abs() < c.abs() {
        1f64.hypot(s / c).recip().copysign(c)
    } else if s.is_nan() {
        s.copysign(c)
    } else if c.is_nan() {
        c
    } else {
        SQRT_2.recip().copysign(c)
    }
}

pub(crate) fn tanaddatan(x: f64, y: f64) -> (f64, f64) {
    tansubatan(x, -y)
}

pub(crate) fn tansubatan(x: f64, y: f64) -> (f64, f64) {
    let xs = x.abs().min(1f64) * x.signum();
    let xc = x.abs().max(1f64).recip();

    let ys = y.abs().min(1f64) * y.signum();
    let yc = y.abs().max(1f64).recip();

    (xs * yc - xc * ys, xs * ys + xc * yc)
}

#[cfg(test)]
pub(crate) fn cossubatan(x: f64, y: f64) -> f64 {
    let (s, c) = tansubatan(x, y);
    cosatan2(s, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use approx::assert_ulps_eq;
    use std::f64::consts::FRAC_1_SQRT_2;
    use std::num::FpCategory::Zero;
    use test_strategy::proptest;

    #[proptest]
    fn sinatan_equals_sine_of_atan(#[strategy(not_nan())] x: f64) {
        assert_ulps_eq!(sinatan(x), x.atan().sin());
    }

    #[proptest]
    fn sinatan_of_nan_is_nan(#[strategy(nan())] x: f64) {
        assert!(sinatan(x).is_nan());
    }

    #[proptest]
    fn cosatan_equals_cosine_of_atan(#[strategy(not_nan())] x: f64) {
        assert_ulps_eq!(cosatan(x), x.atan().cos());
    }

    #[proptest]
    fn cosatan_of_nan_is_nan(#[strategy(nan())] x: f64) {
        assert!(cosatan(x).is_nan());
    }

    #[proptest]
    fn sinatan2_equals_sine_of_atan2_except_at_the_origin(
        #[strategy(not_nan())] x: f64,
        #[strategy(not_nan())] y: f64,
    ) {
        if x.classify() != Zero || y.classify() != Zero {
            assert_ulps_eq!(
                sinatan2(x, y),
                x.atan2(y).sin(),
                epsilon = 2f64 * f64::EPSILON
            );
        } else {
            assert_ulps_eq!(sinatan2(x, y), FRAC_1_SQRT_2.copysign(x));
        }
    }

    #[proptest]
    fn sinatan2_preserves_sign_of_first_argument(
        #[strategy(any())] x: f64,
        #[strategy(any())] y: f64,
    ) {
        assert_eq!(1f64.copysign(sinatan2(x, y)), 1f64.copysign(x));
    }

    #[proptest]
    fn sinatan2_of_nan_is_nan(#[strategy(nan())] x: f64, #[strategy(any())] y: f64) {
        assert!(sinatan2(x, y).is_nan());
        assert!(sinatan2(y, x).is_nan());
    }

    #[proptest]
    fn cosatan2_equals_cosine_of_atan2_except_at_the_origin(
        #[strategy(not_nan())] x: f64,
        #[strategy(not_nan())] y: f64,
    ) {
        if x.classify() != Zero || y.classify() != Zero {
            assert_ulps_eq!(
                cosatan2(x, y),
                x.atan2(y).cos(),
                epsilon = 2f64 * f64::EPSILON
            );
        } else {
            assert_ulps_eq!(cosatan2(x, y), FRAC_1_SQRT_2.copysign(y));
        }
    }

    #[proptest]
    fn cosatan2_preserves_sign_of_second_argument(
        #[strategy(any())] x: f64,
        #[strategy(any())] y: f64,
    ) {
        assert_eq!(1f64.copysign(cosatan2(x, y)), 1f64.copysign(y));
    }

    #[proptest]
    fn sinatan2_hypot_cosatan2_equals_one(
        #[strategy(not_nan())] x: f64,
        #[strategy(not_nan())] y: f64,
    ) {
        assert_ulps_eq!(sinatan2(x, y).hypot(cosatan2(x, y)), 1f64);
    }

    #[proptest]
    fn cosatan2_of_nan_is_nan(#[strategy(nan())] x: f64, #[strategy(any())] y: f64) {
        assert!(cosatan2(x, y).is_nan());
        assert!(cosatan2(y, x).is_nan());
    }

    #[proptest]
    fn tanaddatan_equals_tangent_of_the_sum_of_atan(
        #[strategy(not_nan())] x: f64,
        #[strategy(not_nan())] y: f64,
    ) {
        let (s, c) = tanaddatan(x, y);
        assert_ulps_eq!(s.atan2(c), x.atan() + y.atan());
    }

    #[proptest]
    fn tanaddatan_of_nan_is_nan(#[strategy(nan())] x: f64, #[strategy(any())] y: f64) {
        let (s, c) = tanaddatan(x, y);

        assert!(s.is_nan());
        assert!(c.is_nan());

        let (s, c) = tanaddatan(y, x);

        assert!(s.is_nan());
        assert!(c.is_nan());
    }

    #[proptest]
    fn tansubatan_equals_tangent_of_the_difference_of_atan(
        #[strategy(not_nan())] x: f64,
        #[strategy(not_nan())] y: f64,
    ) {
        let (s, c) = tansubatan(x, y);
        assert_ulps_eq!(s.atan2(c), x.atan() - y.atan());
    }

    #[proptest]
    fn tansubatan_of_nan_is_nan(#[strategy(nan())] x: f64, #[strategy(any())] y: f64) {
        let (s, c) = tansubatan(x, y);

        assert!(s.is_nan());
        assert!(c.is_nan());

        let (s, c) = tansubatan(y, x);

        assert!(s.is_nan());
        assert!(c.is_nan());
    }

    #[proptest]
    fn cossubatan_equals_cosine_of_the_difference_of_atan(
        #[strategy(not_nan())] x: f64,
        #[strategy(not_nan())] y: f64,
    ) {
        assert_ulps_eq!(
            cossubatan(x, y),
            (x.atan() - y.atan()).cos(),
            epsilon = 2f64 * f64::EPSILON
        );
    }

    #[proptest]
    fn cossubatan_of_nan_is_nan(#[strategy(nan())] x: f64, #[strategy(any())] y: f64) {
        assert!(cossubatan(x, y).is_nan());
        assert!(cossubatan(y, x).is_nan());
    }
}
