use super::Phasor;

impl Phasor {
    pub fn log(self, base: f64) -> Self {
        self.ln() / Phasor::polar(base.ln(), 0f64)
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
        fn is_proportional_to_natural_logarithm(mag in not_nan(), tan in not_nan(), b in positive()) {
            prop_assume!(b != 1f64);

            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.log(b).norm(), p.ln().norm() / b.ln().abs());
        }

        #[test]
        fn equals_opposite_of_logarithm_of_inverse(mag in normal(), tan in not_nan(), b in positive()) {
            prop_assume!(b != 1f64);

            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.log(b), -p.recip().log(b), epsilon = 1E-15);
        }

        #[test]
        fn is_zero_if_base_is_zero_and_phasor_is_finite_and_nonzero(mag in regular(), tan in not_nan(), b in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_zero());
        }

        #[test]
        fn is_nan_if_base_is_zero_and_phasor_is_infinite(mag in infinite(), tan in not_nan(), b in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn is_nan_if_base_is_zero_and_phasor_is_zero(mag in zero(), tan in not_nan(), b in zero()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn is_zero_if_base_is_infinite_and_phasor_is_finite_and_nonzero(mag in regular(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(f64::INFINITY).is_zero());
        }

        #[test]
        fn is_nan_if_base_is_infinite_and_phasor_is_infinite(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(f64::INFINITY).is_nan());
        }

        #[test]
        fn is_nan_if_base_is_infinite_and_phasor_is_zero(mag in zero(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(f64::INFINITY).is_nan());
        }

        #[test]
        fn is_infinite_if_base_is_one_and_phasor_is_not_one(mag in not_nan(), tan in not_nan()) {
            prop_assume!(mag != 1f64 || tan != 0f64);

            let p = Phasor { mag, tan };
            assert!(p.log(1f64).is_infinite());
        }

        #[test]
        fn is_nan_if_base_is_one_and_phasor_is_one(tan in zero()) {
            let p = Phasor { mag: 1f64, tan };
            assert!(p.log(1f64).is_nan());
        }

        #[test]
        fn is_nan_if_base_is_negative(mag in any(), tan in any(), b in negative()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn is_nan_if_base_is_nan(mag in any(), tan in any(), b in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any(), b in any()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan(), b in any()) {
            let p = Phasor { mag, tan };
            assert!(p.log(b).is_nan());
        }
    }
}
