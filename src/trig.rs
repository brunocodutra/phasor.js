pub(crate) fn cosatan(x: f64) -> f64 {
    if x.is_nan() {
        x
    } else if x.abs() <= 1f64 {
        x.hypot(1f64).recip()
    } else {
        sinatan(x.recip()).abs()
    }
}

pub(crate) fn sinatan(x: f64) -> f64 {
    if x.is_nan() {
        x
    } else if x.abs() <= 1f64 {
        x / x.hypot(1f64)
    } else {
        cosatan(x.recip()).copysign(x)
    }
}

pub(crate) fn tanaddatan(x: f64, y: f64) -> f64 {
    if x.abs() <= 1f64 && y.abs() <= 1f64 {
        (x + y) / x.mul_add(-y, 1f64)
    } else if x.abs() > 1f64 && y.abs() <= 1f64 {
        y.mul_add(x.recip(), 1f64) / (x.recip() - y)
    } else if x.abs() <= 1f64 && y.abs() > 1f64 {
        x.mul_add(y.recip(), 1f64) / (y.recip() - x)
    } else {
        (x.recip() + y.recip()) / x.recip().mul_add(y.recip(), -1f64)
    }
}

pub(crate) fn tansubatan(x: f64, y: f64) -> f64 {
    if x.abs() <= 1f64 && y.abs() <= 1f64 {
        (x - y) / x.mul_add(y, 1f64)
    } else if x.abs() > 1f64 && y.abs() <= 1f64 {
        y.mul_add(-x.recip(), 1f64) / (x.recip() + y)
    } else if x.abs() <= 1f64 && y.abs() > 1f64 {
        x.mul_add(y.recip(), -1f64) / (y.recip() + x)
    } else {
        (y.recip() - x.recip()) / x.recip().mul_add(y.recip(), 1f64)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use core::f64::consts::PI;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn cosatan_equals_cosine_of_arc_tangent(x: f64) {
            assert_close_to!(cosatan(x), x.atan().cos());
        }

        #[test]
        fn sinatan_equals_sine_of_arc_tangent(x: f64) {
            assert_close_to!(sinatan(x), x.atan().sin());
        }

        #[test]
        fn tanaddatan_equals_tangent_of_the_sum_of_two_arc_tangent(mut x: f64, mut y: f64) {
            x %= 2f64 * PI;
            y %= 2f64 * PI;
            assert_close_to!(tanaddatan(x.tan(), y.tan()), (x + y).tan(), tol = 1E-9);
        }

        #[test]
        fn tansubatan_equals_tangent_of_the_difference_of_two_arc_tangent(mut x: f64, mut y: f64) {
            x %= 2f64 * PI;
            y %= 2f64 * PI;
            assert_close_to!(tansubatan(x.tan(), y.tan()), (x - y).tan(), tol = 1E-9);
        }
    }
}
