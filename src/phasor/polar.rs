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
    use approx::assert_ulps_eq;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn preserves_modulus_of_magnitude(mag in not_nan(), ang in finite()) {
            let p = Phasor::polar(mag, ang);
            assert_ulps_eq!(p.norm(), mag.abs());
        }

        #[test]
        fn preserves_angle_if_magnitude_is_positive(mag in not_nan(), ang in finite()) {
            let p = Phasor::polar(mag.abs(), ang);
            assert_ulps_eq!(p.angle(), ang.sin().atan2(ang.cos()));
        }

        #[test]
        fn is_real_if_angle_is_zero(mag in not_nan(), ang in zero()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_real());
        }

        #[test]
        fn is_nan_if_angle_is_infinite(mag in not_nan(), ang in infinite()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_nan());
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), ang in any()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_nan());
        }

        #[test]
        fn is_nan_if_angle_is_nan(mag in any(), ang in nan()) {
            let p = Phasor::polar(mag, ang);
            assert!(p.is_nan());
        }
    }
}
