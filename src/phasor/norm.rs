use super::Phasor;
use std::f64::NAN;

impl Phasor {
    pub fn norm(&self) -> f64 {
        if self.is_nan() {
            NAN
        } else {
            self.mag.abs()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use approx::assert_ulps_eq;
    use test_strategy::proptest;

    #[proptest]
    fn is_never_negative(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(!p.norm().is_sign_negative());
    }

    #[proptest]
    fn equals_modulus_of_magnitude(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.norm(), mag.abs());
    }

    #[proptest]
    fn equals_modulus_of_real_part_if_phasor_is_real(
        #[strategy(not_nan())] mag: f64,
        #[strategy(zero())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.norm(), p.real().abs());
    }

    #[proptest]
    fn equals_modulus_of_imaginary_part_if_imaginary(
        #[strategy(not_nan())] mag: f64,
        #[strategy(infinite())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.norm(), p.imag().abs());
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.norm().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.norm().is_nan());
    }
}
