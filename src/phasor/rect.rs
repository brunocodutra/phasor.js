use super::Phasor;
use core::num::FpCategory;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn rect(re: f64, im: f64) -> Self {
        Phasor {
            mag: re.hypot(im).copysign(re),

            tan: if im.classify() == FpCategory::Zero {
                im / re.signum() // := +-{0, PI}
            } else {
                im / re
            },
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use alloc::format;
    use approx::assert_ulps_eq;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn preserves_finite_real_part(re in finite(), im in finite()) {
            prop_assume!(re.abs() >= f64::MIN_POSITIVE * im.abs());
            prop_assume!(re.abs() <= f64::MAX * im.abs());
            prop_assume!(re.hypot(im).is_finite());

            let p = Phasor::rect(re, im);
            assert_ulps_eq!(p.real(), re);
        }

        #[test]
        fn preserves_finite_imaginary_part(re in finite(), im in finite()) {
            prop_assume!(re.abs() >= f64::MIN_POSITIVE * im.abs());
            prop_assume!(re.abs() <= f64::MAX * im.abs());
            prop_assume!(re.hypot(im).is_finite());

            let p = Phasor::rect(re, im);
            assert_ulps_eq!(p.imag(), im);
        }

        #[test]
        fn is_real_if_imaginary_part_is_zero(re in not_nan(), im in zero()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_real());
        }

        #[test]
        fn is_imaginary_if_real_part_is_zero_and_imaginary_part_is_nonzero(re in zero(), im in nonzero()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_imaginary());
        }

        #[test]
        fn is_nan_if_real_and_imaginary_parts_are_infinite(re in infinite(), im in infinite()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_nan());
        }

        #[test]
        fn is_nan_if_real_part_is_nan(re in nan(), im in any()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_nan());
        }

        #[test]
        fn is_nan_if_imaginary_part_is_nan(re in any(), im in nan()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_nan());
        }
    }
}
