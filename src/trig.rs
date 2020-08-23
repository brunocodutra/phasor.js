pub(crate) fn cosatan(x: f64) -> f64 {
    if x.abs() <= 1f64 {
        x.hypot(1f64).recip()
    } else {
        sinatan(x.recip()).abs()
    }
}

pub(crate) fn sinatan(x: f64) -> f64 {
    if x.abs() <= 1f64 {
        x / x.hypot(1f64)
    } else {
        cosatan(x.recip()).copysign(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close_to;
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
    }
}
