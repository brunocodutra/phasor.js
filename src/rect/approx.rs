use super::*;
use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

fn distance(x: &Rect, y: &Rect) -> f64 {
    (x.real - y.real).hypot(x.imag - y.imag)
}

impl AbsDiffEq for Rect {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, e: Self::Epsilon) -> bool {
        distance(self, other).abs_diff_eq(&0f64, e)
    }
}

impl RelativeEq for Rect {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, e: Self::Epsilon, max: Self::Epsilon) -> bool {
        distance(self, other).relative_eq(&0f64, e, max)
    }
}

impl UlpsEq for Rect {
    fn default_max_ulps() -> u32 {
        Self::Epsilon::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, e: Self::Epsilon, max: u32) -> bool {
        distance(self, other).ulps_eq(&0f64, e, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn close_to(rect: Rect) {
            let other = Rect {
                real: rect.real + f64::EPSILON,
                imag: rect.imag + f64::EPSILON,
            };

            assert_close_to!(rect, other);
        }
    }
}
