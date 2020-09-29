use super::Phasor;

impl Phasor {
    pub fn log(self, base: f64) -> Self {
        self.ln() / Phasor::polar(base.ln(), 0f64)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::assert_close_to;
    use alloc::format;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn the_logarithm_of_a_phasor_to_a_base_is_proportional_to_its_natural_logarithm(mag in not_nan(), tan in not_nan(), b in positive()) {
            prop_assume!(b != 1f64);

            let p = Phasor { mag, tan };

            assert_close_to!(p.log(b).real(), p.ln().real() / b.ln());
            assert_close_to!(p.log(b).imag(), p.ln().imag() / b.ln());
        }

        #[test]
        fn the_logarithm_of_the_inverse_of_a_normal_finite_phasor_equals_opposite_of_logarithm(mag in normal(), tan in not_nan(), b in positive()) {
            prop_assume!(b != 1f64);

            let p = Phasor { mag, tan };
            assert_close_to!(p.recip().log(b), -p.log(b));
        }

        #[test]
        fn the_logarithm_of_a_finite_nonzero_phasor_to_zero_is_zero(mag in regular(), tan in not_nan(), b in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_zero());
        }

        #[test]
        fn the_logarithm_of_infinite_phasor_to_zero_is_nan(mag in infinite(), tan in not_nan(), b in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn the_logarithm_of_zero_phasor_to_zero_is_nan(mag in zero(), tan in not_nan(), b in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn the_logarithm_of_a_finite_nonzero_phasor_to_infinity_is_zero(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(f64::INFINITY).is_zero());
        }

        #[test]
        fn the_logarithm_of_infinite_phasor_to_infinity_is_nan(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(f64::INFINITY).is_nan());
        }

        #[test]
        fn the_logarithm_of_zero_phasor_to_infinity_is_nan(mag in zero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(f64::INFINITY).is_nan());
        }

        #[test]
        fn the_logarithm_of_a_phasor_to_one_is_infinite(mag in not_nan(), tan in not_nan()) {
            prop_assume!(mag != 1f64 || tan != 0f64);

            let p = Phasor { mag, tan };
            assert!(p.log(1f64).is_infinite());
        }

        #[test]
        fn the_logarithm_of_one_to_one_is_nan(tan in zero()) {
            let p = Phasor { mag: 1f64, tan };
            assert!(p.log(1f64).is_nan());
        }

        #[test]
        fn the_logarithm_of_a_phasor_to_a_negative_base_is_nan(mag in any(), tan in any(), b in negative()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn the_logarithm_of_a_phasor_to_an_undefined_base_is_nan(mag in any(), tan in any(), b in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn the_logarithm_of_a_phasor_that_has_undefined_magnitude_is_nan(mag in nan(), tan in any(), b in any()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn the_logarithm_of_a_phasor_that_has_undefined_tangent_is_nan(mag in any(), tan in nan(), b in any()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }
    }
}
