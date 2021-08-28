use super::Phasor;
use std::num::FpCategory;

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
    use test_strategy::proptest;

    #[proptest]
    fn has_the_class_of_mag_if_tan_is_not_nan(
        #[strategy(any())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_eq!(p.classify(), mag.classify());
    }

    #[proptest]
    fn has_the_class_of_tan_if_tan_is_nan(
        #[strategy(any())] mag: f64,
        #[strategy(nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_eq!(p.classify(), tan.classify());
    }

    #[proptest]
    fn is_nan_if_tan_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.is_nan());
    }

    #[proptest]
    fn is_nan_if_mag_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.is_nan());
    }

    #[proptest]
    fn is_infinite_if_mag_is_infinite_and_tan_is_not_nan(
        #[strategy(infinite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.is_infinite());
    }

    #[proptest]
    fn is_finite_if_mag_is_finite_and_tan_is_not_nan(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.is_finite());
    }

    #[proptest]
    fn is_zero_if_mag_is_zero_and_tan_is_not_nan(
        #[strategy(zero())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.is_zero());
    }

    #[proptest]
    fn is_subnormal_if_mag_is_subnormal_and_tan_is_not_nan(
        #[strategy(subnormal())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.is_subnormal());
    }

    #[proptest]
    fn is_normal_if_mag_is_normal_and_tan_is_not_nan(
        #[strategy(normal())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.is_normal());
    }

    #[proptest]
    fn is_real_if_mag_is_not_nan_and_tan_is_zero(
        #[strategy(not_nan())] mag: f64,
        #[strategy(zero())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.is_real());
    }

    #[proptest]
    fn is_not_real_if_mag_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(!p.is_real());
    }

    #[proptest]
    fn is_not_real_if_tan_is_nonzero(#[strategy(any())] mag: f64, #[strategy(nonzero())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(!p.is_real());
    }

    #[proptest]
    fn is_imaginary_if_mag_is_not_nan_and_tan_is_infinite(
        #[strategy(not_nan())] mag: f64,
        #[strategy(infinite())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.is_imaginary());
    }

    #[proptest]
    fn is_not_imaginary_if_mag_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(!p.is_imaginary());
    }

    #[proptest]
    fn is_not_imaginary_if_tan_is_finite(
        #[strategy(any())] mag: f64,
        #[strategy(finite())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(!p.is_imaginary());
    }
}
