use crate::Phasor;
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
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn the_modulus_of_the_angle_of_a_phasor_is_never_greater_than_pi(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(!(p.angle().abs() > PI));
        }

        #[test]
        fn the_angle_of_a_finite_nonzero_phasor_equals_the_arc_whose_tangent_is_imaginary_part_over_real_part(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.angle(), p.imag().atan2(p.real()), tol = 1E-12);
        }

        #[test]
        fn the_angle_of_a_phasor_with_undefined_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.angle().is_nan());
        }

        #[test]
        fn the_angle_of_a_phasor_with_undefined_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.angle().is_nan());
        }
    }
}
