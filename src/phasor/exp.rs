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
    use alloc::format;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn has_norm_equal_to_exponential_of_real_part(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.exp().norm(), p.real().exp());
        }

        #[test]
        fn has_angle_equal_to_imaginary_part(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.exp().angle(), p.imag().sin().atan2(p.imag().cos()));
        }

        #[test]
        fn equals_inverse_of_exponential_of_opposite(mag in finite(), tan in not_nan()) {
            prop_assume!(mag.exp().classify() == (-mag).exp().recip().classify());

            let p = Phasor { mag, tan };
            assert_close_to!(p.exp(), (-p).exp().recip());
        }

        #[test]
        fn equals_one_if_phasor_is_zero(mag in zero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: 1f64, tan: 0f64 };
            assert_close_to!(p.exp(), r);
        }

        #[test]
        fn is_real_if_phasor_is_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: mag.exp(), tan };
            assert_close_to!(p.exp(), r);
        }

        #[test]
        fn is_nan_if_phasor_is_infinite_and_not_real(mag in infinite(), tan in nonzero()) {
            let p = Phasor { mag, tan };
            assert!(p.exp().is_nan());
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.exp().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.exp().is_nan());
        }
    }
}
