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
    use crate::assert_close_to;
    use alloc::format;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn rectangular_form_preserves_finite_real_part(re in finite(), im in finite()) {
            prop_assume!(re.abs() >= f64::MIN_POSITIVE * im.abs() && re.abs() <= f64::MAX * im.abs());

            let p = Phasor::rect(re, im);
            assert_close_to!(p.real(), re);
        }

        #[test]
        fn rectangular_form_preserves_finite_imaginary_part(re in finite(), im in finite()) {
            prop_assume!(re.abs() >= f64::MIN_POSITIVE * im.abs() && re.abs() <= f64::MAX * im.abs());

            let p = Phasor::rect(re, im);
            assert_close_to!(p.imag(), im);
        }

        #[test]
        fn rectangular_form_with_zero_imaginary_part_is_purely_real(re in not_nan(), im in zero()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_real());
        }

        #[test]
        fn rectangular_form_with_zero_real_and_nonzero_imaginary_parts_is_purely_imaginary(re in zero(), im in nonzero()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_imaginary());
        }

        #[test]
        fn rectangular_form_with_infinite_real_and_imaginary_parts_is_nan(re in infinite(), im in infinite()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_nan());
        }

        #[test]
        fn rectangular_form_with_undefined_real_part_is_nan(re in nan(), im in any()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_nan());
        }

        #[test]
        fn rectangular_form_with_undefined_imaginary_part_is_nan(re in any(), im in nan()) {
            let p = Phasor::rect(re, im);
            assert!(p.is_nan());
        }
    }
}
