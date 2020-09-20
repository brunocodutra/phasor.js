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
        fn the_reciprocal_of_a_phasor_has_inverse_norm(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().norm(), p.norm().recip());
        }

        #[test]
        fn the_reciprocal_of_a_phasor_has_opposite_angle(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().angle(), -p.angle());
        }

        #[test]
        fn double_inversion_has_no_effect(mag in zero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().recip(), p);
        }

        #[test]
        fn the_reciprocal_of_a_phasor_that_has_undefined_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.recip().is_nan());
        }

        #[test]
        fn the_reciprocal_of_a_phasor_that_has_undefined_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.recip().is_nan());
        }
    }
}
