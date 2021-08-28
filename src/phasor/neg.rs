use super::Phasor;
use std::ops::Neg;

impl Neg for Phasor {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Phasor {
            mag: -self.mag,
            tan: self.tan,
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
    fn negates_real_part(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.neg().real(), -p.real());
    }

    #[proptest]
    fn negates_imaginary_part(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.neg().imag(), -p.imag());
    }

    #[proptest]
    fn is_its_own_inverse_function(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.neg().neg(), p);
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.neg().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.neg().is_nan());
    }
}
