use super::*;
use std::ops::Add;

impl Add for Phasor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Phasor::rect(self.real() + rhs.real(), self.imag() + rhs.imag())
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn add(a: f64, b: f64, c: f64, d: f64) {
            assert_close_to!(
                Phasor::rect(a, b) + Phasor::rect(c, d),
                Phasor::rect(a + c, b + d)
            );
        }
    }
}
