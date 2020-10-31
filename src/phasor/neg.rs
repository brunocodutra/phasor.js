use super::Phasor;
use core::ops::Neg;

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
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn negates_real_part(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.neg().real(), -p.real());
        }

        #[test]
        fn negates_imaginary_part(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.neg().imag(), -p.imag());
        }

        #[test]
        fn is_its_own_inverse_function(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.neg().neg(), p);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.neg().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.neg().is_nan());
        }
    }
}
