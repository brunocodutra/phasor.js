use crate::Phasor;
use core::f64::NAN;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn norm(&self) -> f64 {
        if self.is_nan() {
            NAN
        } else {
            self.mag.abs()
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
        fn is_never_negative(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(!p.norm().is_sign_negative());
        }

        #[test]
        fn equals_modulus_of_magnitude(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.norm(), mag.abs());
        }

        #[test]
        fn equals_modulus_of_real_part_if_phasor_is_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.norm(), p.real().abs());
        }

        #[test]
        fn equals_modulus_of_imaginary_part_if_imaginary(mag in not_nan(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.norm(), p.imag().abs());
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.norm().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.norm().is_nan());
        }
    }
}
