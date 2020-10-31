use super::Phasor;
use core::num::FpCategory;

impl Phasor {
    pub fn classify(&self) -> FpCategory {
        if self.tan.is_nan() {
            FpCategory::Nan
        } else {
            self.mag.classify()
        }
    }
}

impl Phasor {
    pub fn is_nan(&self) -> bool {
        self.classify() == FpCategory::Nan
    }

    pub fn is_infinite(&self) -> bool {
        self.classify() == FpCategory::Infinite
    }

    pub fn is_finite(&self) -> bool {
        !matches!(self.classify(), FpCategory::Infinite | FpCategory::Nan)
    }

    pub fn is_zero(&self) -> bool {
        self.classify() == FpCategory::Zero
    }

    pub fn is_subnormal(&self) -> bool {
        self.classify() == FpCategory::Subnormal
    }

    pub fn is_normal(&self) -> bool {
        self.classify() == FpCategory::Normal
    }

    pub fn is_real(&self) -> bool {
        self.tan.classify() == FpCategory::Zero && self.mag.classify() != FpCategory::Nan
    }

    pub fn is_imaginary(&self) -> bool {
        self.tan.classify() == FpCategory::Infinite && self.mag.classify() != FpCategory::Nan
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn has_the_class_of_mag_if_tan_is_not_nan(mag in any(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_eq!(p.classify(), mag.classify());
        }

        #[test]
        fn has_the_class_of_tan_if_tan_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert_eq!(p.classify(), tan.classify());
        }

        #[test]
        fn is_nan_if_tan_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_nan());
        }

        #[test]
        fn is_nan_if_mag_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.is_nan());
        }

        #[test]
        fn is_infinite_if_mag_is_infinite_and_tan_is_not_nan(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_infinite());
        }

        #[test]
        fn is_finite_if_mag_is_finite_and_tan_is_not_nan(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_finite());
        }

        #[test]
        fn is_zero_if_mag_is_zero_and_tan_is_not_nan(mag in zero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_zero());
        }

        #[test]
        fn is_subnormal_if_mag_is_subnormal_and_tan_is_not_nan(mag in subnormal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_subnormal());
        }

        #[test]
        fn is_normal_if_mag_is_normal_and_tan_is_not_nan(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.is_normal());
        }

        #[test]
        fn is_real_if_mag_is_not_nan_and_tan_is_zero(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.is_real());
        }

        #[test]
        fn is_not_real_if_mag_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(!p.is_real());
        }

        #[test]
        fn is_not_real_if_tan_is_nonzero(mag in any(), tan in nonzero()) {
            let p = Phasor { mag, tan };
            assert!(!p.is_real());
        }

        #[test]
        fn is_imaginary_if_mag_is_not_nan_and_tan_is_infinite(mag in not_nan(), tan in infinite()) {
            let p = Phasor { mag, tan };
            assert!(p.is_imaginary());
        }

        #[test]
        fn is_not_imaginary_if_mag_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(!p.is_imaginary());
        }

        #[test]
        fn is_not_imaginary_if_tan_is_finite(mag in any(), tan in finite()) {
            let p = Phasor { mag, tan };
            assert!(!p.is_imaginary());
        }
    }
}
