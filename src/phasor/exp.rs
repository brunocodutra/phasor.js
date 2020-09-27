use super::Phasor;

impl Phasor {
    pub fn exp(self) -> Self {
        Phasor::polar(self.real().exp(), self.imag())
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
        fn the_exponential_of_a_finite_phasor_has_norm_equal_to_exponential_of_its_real_part(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.exp().norm(), p.real().exp());
        }

        #[test]
        fn the_exponential_of_a_finite_phasor_has_angle_equal_to_its_imaginary_part(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.exp().angle(), p.imag().sin().atan2(p.imag().cos()));
        }

        #[test]
        fn the_exponential_of_the_opposite_of_a_finite_phasor_equals_inverse_of_exponential(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!((-p).exp(), p.exp().recip());
        }

        #[test]
        fn the_exponential_of_a_zero_phasor_is_one(mag in zero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: 1f64, tan: 0f64 };
            assert_close_to!(p.exp(), r);
        }

        #[test]
        fn the_exponential_of_a_real_phasor_is_purely_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: mag.exp(), tan };
            assert_close_to!(p.exp(), r);
        }

        #[test]
        fn the_exponential_of_a_non_real_infinite_phasor_is_nan(mag in infinite(), tan in nonzero()) {
            let p = Phasor { mag, tan };
            assert!(p.exp().is_nan());
        }

        #[test]
        fn the_exponential_of_a_phasor_that_has_undefined_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.exp().is_nan());
        }

        #[test]
        fn the_exponential_of_a_phasor_that_has_undefined_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.exp().is_nan());
        }
    }
}
