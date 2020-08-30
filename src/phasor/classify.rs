use crate::Phasor;
use core::num::FpCategory;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

impl Phasor {
    pub fn classify(&self) -> FpCategory {
        if self.tan.is_nan() {
            FpCategory::Nan
        } else {
            self.mag.classify()
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn is_nan(&self) -> bool {
        self.classify() == FpCategory::Nan
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn is_infinite(&self) -> bool {
        self.classify() == FpCategory::Infinite
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn is_finite(&self) -> bool {
        !matches!(self.classify(), FpCategory::Infinite | FpCategory::Nan)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn is_zero(&self) -> bool {
        self.classify() == FpCategory::Zero
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn is_subnormal(&self) -> bool {
        self.classify() == FpCategory::Subnormal
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn is_normal(&self) -> bool {
        self.classify() == FpCategory::Normal
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn phasor_has_the_class_of_mag_if_tan_is_not_nan(mag in any(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_eq!(p.classify(), mag.classify());
        }

        #[test]
        fn phasor_has_the_class_of_tan_if_tan_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert_eq!(p.classify(), tan.classify());
        }

        #[test]
        fn phasor_is_nan_if_tan_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_nan());
        }

        #[test]
        fn phasor_is_nan_if_mag_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.is_nan());
        }

        #[test]
        fn phasor_is_infinite_if_mag_is_infinite_and_tan_is_not_nan(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_infinite());
        }

        #[test]
        fn phasor_is_finite_if_mag_is_finite_and_tan_is_not_nan(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_finite());
        }

        #[test]
        fn phasor_is_zero_if_mag_is_zero_and_tan_is_not_nan(mag in zero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_zero());
        }

        #[test]
        fn phasor_is_subnormal_if_mag_is_subnormal_and_tan_is_not_nan(mag in subnormal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_subnormal());
        }

        #[test]
        fn phasor_is_normal_if_mag_is_normal_and_tan_is_not_nan(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_normal());
        }
    }
}
