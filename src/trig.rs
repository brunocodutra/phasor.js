pub(crate) fn cosatan(x: f64) -> f64 {
    if x.recip().is_finite() && !x.recip().is_normal() {
        x.recip()
    } else {
        x.hypot(1f64).recip()
    }
}

pub(crate) fn sinatan(x: f64) -> f64 {
    if x.is_finite() && !x.is_normal() {
        x
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
