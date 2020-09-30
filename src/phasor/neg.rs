use super::Phasor;
use core::ops::Neg;

impl Neg for Phasor {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Phasor {
            mag: -self.mag,
            tan: self.tan,
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
        fn the_opposite_of_a_phasor_has_opposite_real_part(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.neg().real(), -p.real());
        }

        #[test]
        fn the_opposite_of_a_phasor_has_opposite_imaginary_part(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.neg().imag(), -p.imag());
        }

        #[test]
        fn double_negation_has_no_effect(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.neg().neg(), p);
        }

        #[test]
        fn the_opposite_of_a_phasor_that_has_undefined_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.neg().is_nan());
        }

        #[test]
        fn the_opposite_of_a_phasor_that_has_undefined_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.neg().is_nan());
        }
    }
}
