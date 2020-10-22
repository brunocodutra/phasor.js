use super::Phasor;
use core::f64::{consts::PI, NAN};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn angle(&self) -> f64 {
        if self.is_nan() {
            NAN
        } else if self.mag.is_sign_positive() {
            self.tan.atan()
        } else {
            self.tan.atan() - PI.copysign(self.tan)
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
        fn has_absolute_value_no_greater_than_pi(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(!(p.angle().abs() > PI));
        }

        #[test]
        fn equals_arc_formed_by_imaginary_and_real_parts(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.angle(), p.imag().atan2(p.real()), epsilon = 1E-12);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.angle().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.angle().is_nan());
        }
    }
}
