use crate::{trig::sinatan, Phasor};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn imag(&self) -> f64 {
        let s = sinatan(self.tan);
        if s.abs() > 0f64 {
            s * self.mag
        } else {
            s * self.mag.signum()
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
        fn the_imaginary_part_of_a_finite_phasor_equals_magnitude_times_sine_of_angle(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.imag(), mag * sinatan(tan));
        }

        #[test]
        fn the_imaginary_part_of_a_real_phasor_is_zero(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.imag(), 0f64);
        }

        #[test]
        fn the_imaginary_part_of_an_imaginary_phasor_equals_magnitude(mag in not_nan(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.imag(), mag * tan.signum());
        }

        #[test]
        fn the_imaginary_part_of_a_phasor_with_undefined_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.imag().is_nan());
        }

        #[test]
        fn the_imaginary_part_of_a_phasor_with_undefined_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.imag().is_nan());
        }
    }
}
