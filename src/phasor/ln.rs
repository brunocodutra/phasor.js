use super::Phasor;

impl Phasor {
    pub fn ln(self) -> Self {
        Phasor::rect(self.norm().ln(), self.angle())
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
        fn the_logarithm_of_a_phasor_has_real_path_equal_to_logarithm_of_its_norm(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.ln().real(), p.norm().ln());
        }

        #[test]
        fn the_logarithm_of_a_finite_nonzero_phasor_has_imaginary_part_equal_to_its_angle(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.ln().imag(), p.angle());
        }

        #[test]
        fn the_logarithm_of_the_inverse_of_a_normal_finite_phasor_equals_opposite_of_logarithm(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().ln(), -p.ln());
        }

        #[test]
        fn the_logarithm_of_a_not_real_unit_phasor_is_purely_imaginary(mag in zero(), tan in nonzero()) {
            let p = Phasor { mag: mag.signum(), tan };
            let r = Phasor { mag: p.angle(), tan: f64::INFINITY };
            assert_close_to!(p.ln(), r);
        }

        #[test]
        fn the_logarithm_of_a_real_positive_phasor_is_purely_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag: mag.abs(), tan };
            let r = Phasor { mag: mag.abs().ln(), tan };
            assert_close_to!(p.ln(), r);
        }

        #[test]
        fn the_logarithm_of_a_phasor_that_has_undefined_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.ln().is_nan());
        }

        #[test]
        fn the_logarithm_of_a_phasor_that_has_undefined_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.ln().is_nan());
        }
    }
}
