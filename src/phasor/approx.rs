use super::*;
use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

fn distance(p: &Phasor, q: &Phasor) -> f64 {
    if (p.mag.is_infinite() && q.mag.is_infinite())
        || (p.mag.recip().is_infinite() && q.mag.recip().is_infinite())
    {
        cosatan(tansubatan(p.tan, q.tan))
    } else {
        2f64 * cosatan(tansubatan(p.tan, q.tan)) * cosatan(p.mag / q.mag) * cosatan(q.mag / p.mag)
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
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn close_to(a: f64, b: f64) {
            assert_close_to!(
                Phasor::rect(a * 1.0000000001, b * 0.9999999999),
                Phasor::rect(a * 0.9999999999, b * 1.0000000001)
            );
        }
    }
}
