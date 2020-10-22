use super::Phasor;

impl Phasor {
    pub fn ln(self) -> Self {
        Phasor::rect(self.norm().ln(), self.angle())
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
        fn has_real_part_equal_to_logarithm_of_norm(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.ln().real(), p.norm().ln());
        }

        #[test]
        fn has_imaginary_part_equal_to_angle(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.ln().imag(), p.angle());
        }

        #[test]
        fn equals_opposite_of_logarithm_of_inverse(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.ln(), -p.recip().ln(), epsilon = 1E-15);
        }

        #[test]
        fn is_imaginary_if_phasor_is_not_one(mag in zero(), tan in nonzero()) {
            let p = Phasor { mag: mag.signum(), tan };
            let r = Phasor { mag: p.angle(), tan: f64::INFINITY };
            assert_ulps_eq!(p.ln(), r);
        }

        #[test]
        fn is_real_if_phasor_is_real_and_positive(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag: mag.abs(), tan };
            let r = Phasor { mag: mag.abs().ln(), tan };
            assert_ulps_eq!(p.ln(), r);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.ln().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.ln().is_nan());
        }
    }
}
