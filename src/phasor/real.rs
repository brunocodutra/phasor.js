use crate::{trig::cosatan, Phasor};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn real(&self) -> f64 {
        let c = cosatan(self.tan);
        if c.abs() > 0f64 {
            c * self.mag
        } else {
            c * self.mag.signum()
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
        fn equals_product_of_magnitude_and_cosine_of_angle(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.real(), mag * cosatan(tan));
        }

        #[test]
        fn equals_magnitude_if_phasor_is_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.real(), mag);
        }

        #[test]
        fn is_zero_if_phasor_is_imaginary(mag in not_nan(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.real(), 0f64);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.real().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.real().is_nan());
        }
    }
}
