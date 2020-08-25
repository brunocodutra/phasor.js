use super::*;
use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

fn distance(p: &Phasor, q: &Phasor) -> (f64, f64) {
    (
        p.mag.hypot(q.mag),
        2f64 * cosatan(tansubatan(p.tan, q.tan)) / p.mag.recip().hypot(q.mag.recip()),
    )
}

impl AbsDiffEq for Phasor {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, e: Self::Epsilon) -> bool {
        let (x, y) = distance(self, other);
        x.abs_diff_eq(&y, e)
    }
}

impl RelativeEq for Phasor {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, e: Self::Epsilon, max: Self::Epsilon) -> bool {
        let (x, y) = distance(self, other);
        x.relative_eq(&y, e, max)
    }
}

impl UlpsEq for Phasor {
    fn default_max_ulps() -> u32 {
        Self::Epsilon::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, e: Self::Epsilon, max: u32) -> bool {
        let (x, y) = distance(self, other);
        x.ulps_eq(&y, e, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn close_to(p: Phasor) {
            let q = Phasor {
                mag: p.mag + f64::EPSILON,
                tan: p.tan + f64::EPSILON,
            };

            assert_close_to!(p, q);
        }
    }
}
