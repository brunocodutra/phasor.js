use super::Phasor;

impl Phasor {
    pub fn recip(self) -> Phasor {
        Phasor {
            mag: self.mag.recip(),
            tan: -self.tan,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn inverts_norm(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().norm(), p.norm().recip());
        }

        #[test]
        fn negates_angle(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().angle(), -p.angle());
        }

        #[test]
        fn is_its_own_inverse_function(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().recip(), p);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.recip().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.recip().is_nan());
        }
    }
}
