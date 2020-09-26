use super::Phasor;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn polar(mag: f64, angle: f64) -> Self {
        Phasor {
            mag: mag * angle.cos().signum(),
            tan: angle.tan(),
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
        fn polar_form_preserves_modulus_of_magnitude(mag in not_nan(), ang in finite()) {
            let p = Phasor::polar(mag, ang);
            assert_close_to!(p.norm(), mag.abs());
        }

        #[test]
        fn polar_form_preserves_angle_if_magnitude_is_positive(mag in not_nan(), ang in finite()) {
            let p = Phasor::polar(mag.abs(), ang);
            assert_close_to!(p.angle(), ang.sin().atan2(ang.cos()));
        }

        #[test]
        fn polar_form_with_zero_angle_is_purely_real(mag in not_nan(), ang in zero()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_real());
        }

        #[test]
        fn polar_form_with_infinite_angle_is_nan(mag in not_nan(), ang in infinite()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_nan());
        }

        #[test]
        fn polar_form_with_undefined_magnitude_is_nan(mag in nan(), ang in any()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_nan());
        }

        #[test]
        fn polar_form_with_undefined_angle_is_nan(mag in any(), ang in nan()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_nan());
        }
    }
}
