use super::Phasor;
use std::num::FpCategory;

impl Phasor {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use approx::assert_ulps_eq;
    use proptest::prop_assume;
    use test_strategy::proptest;

    #[proptest]
    fn preserves_finite_real_part(#[strategy(finite())] re: f64, #[strategy(finite())] im: f64) {
        prop_assume!(re.abs() >= f64::MIN_POSITIVE * im.abs());
        prop_assume!(re.abs() <= f64::MAX * im.abs());
        prop_assume!(re.hypot(im).is_finite());

        let p = Phasor::rect(re, im);
        assert_ulps_eq!(p.real(), re);
    }

    #[proptest]
    fn preserves_finite_imaginary_part(
        #[strategy(finite())] re: f64,
        #[strategy(finite())] im: f64,
    ) {
        prop_assume!(re.abs() >= f64::MIN_POSITIVE * im.abs());
        prop_assume!(re.abs() <= f64::MAX * im.abs());
        prop_assume!(re.hypot(im).is_finite());

        let p = Phasor::rect(re, im);
        assert_ulps_eq!(p.imag(), im);
    }

    #[proptest]
    fn is_real_if_imaginary_part_is_zero(
        #[strategy(not_nan())] re: f64,
        #[strategy(zero())] im: f64,
    ) {
        let p = Phasor::rect(re, im);
        assert!(p.is_real());
    }

    #[proptest]
    fn is_imaginary_if_real_part_is_zero_and_imaginary_part_is_nonzero(
        #[strategy(zero())] re: f64,
        #[strategy(nonzero())] im: f64,
    ) {
        let p = Phasor::rect(re, im);
        assert!(p.is_imaginary());
    }

    #[proptest]
    fn is_nan_if_real_and_imaginary_parts_are_infinite(
        #[strategy(infinite())] re: f64,
        #[strategy(infinite())] im: f64,
    ) {
        let p = Phasor::rect(re, im);
        assert!(p.is_nan());
    }

    #[proptest]
    fn is_nan_if_real_part_is_nan(#[strategy(nan())] re: f64, #[strategy(any())] im: f64) {
        let p = Phasor::rect(re, im);
        assert!(p.is_nan());
    }

    #[proptest]
    fn is_nan_if_imaginary_part_is_nan(#[strategy(any())] re: f64, #[strategy(nan())] im: f64) {
        let p = Phasor::rect(re, im);
        assert!(p.is_nan());
    }
}
