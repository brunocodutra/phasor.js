use super::*;
use core::ops::Mul;

impl Mul for Phasor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let tan = tanaddatan(self.tan, rhs.tan);
        let mag = if self.tan.signum() != tan.signum() && rhs.tan.signum() != tan.signum() {
            -self.mag * rhs.mag
        } else {
            self.mag * rhs.mag
        };

        Phasor { mag, tan }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use alloc::format;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn mul(a: f64, b: f64, c: f64, d: f64) {
            let p = Phasor::rect(a, b);
            let q = Phasor::rect(c, d);

            let t = p.mag.abs().max(1f64);
            let u = q.mag.abs().max(1f64);

            let r = Phasor::rect(
                t * ((a / t) * (c / u) - (b / t) * (d / u)) * u,
                t * ((a / t) * (d / u) + (b / t) * (c / u)) * u,
            );

            prop_assume!(r.mag.is_normal());
            assert_close_to!(p * q, r);
        }
    }
}
