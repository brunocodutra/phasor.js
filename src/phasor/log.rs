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
    use proptest::prop_assume;
    use test_strategy::proptest;

    #[proptest]
    fn is_proportional_to_natural_logarithm(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] tan: f64,
        #[strategy(positive())] b: f64,
    ) {
        prop_assume!(b != 1f64);

        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.log(b).norm(), p.ln().norm() / b.ln().abs());
    }

    #[proptest]
    fn equals_opposite_of_logarithm_of_inverse(
        #[strategy(normal())] mag: f64,
        #[strategy(not_nan())] tan: f64,
        #[strategy(positive())] b: f64,
    ) {
        prop_assume!(b != 1f64);

        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.log(b), -p.recip().log(b), epsilon = 1E-15);
    }

    #[proptest]
    fn is_zero_if_base_is_zero_and_phasor_is_finite_and_nonzero(
        #[strategy(regular())] mag: f64,
        #[strategy(not_nan())] tan: f64,
        #[strategy(zero())] b: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(b).is_zero());
    }

    #[proptest]
    fn is_nan_if_base_is_zero_and_phasor_is_infinite(
        #[strategy(infinite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
        #[strategy(zero())] b: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(b).is_nan());
    }

    #[proptest]
    fn is_nan_if_base_is_zero_and_phasor_is_zero(
        #[strategy(zero())] mag: f64,
        #[strategy(not_nan())] tan: f64,
        #[strategy(zero())] b: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(b).is_nan());
    }

    #[proptest]
    fn is_zero_if_base_is_infinite_and_phasor_is_finite_and_nonzero(
        #[strategy(regular())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(f64::INFINITY).is_zero());
    }

    #[proptest]
    fn is_nan_if_base_is_infinite_and_phasor_is_infinite(
        #[strategy(infinite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(f64::INFINITY).is_nan());
    }

    #[proptest]
    fn is_nan_if_base_is_infinite_and_phasor_is_zero(
        #[strategy(zero())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(f64::INFINITY).is_nan());
    }

    #[proptest]
    fn is_infinite_if_base_is_one_and_phasor_is_not_one(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        prop_assume!(mag != 1f64 || tan != 0f64);

        let p = Phasor { mag, tan };
        assert!(p.log(1f64).is_infinite());
    }

    #[proptest]
    fn is_nan_if_base_is_one_and_phasor_is_one(#[strategy(zero())] tan: f64) {
        let p = Phasor { mag: 1f64, tan };
        assert!(p.log(1f64).is_nan());
    }

    #[proptest]
    fn is_nan_if_base_is_negative(
        #[strategy(any())] mag: f64,
        #[strategy(any())] tan: f64,
        #[strategy(negative())] b: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(b).is_nan());
    }

    #[proptest]
    fn is_nan_if_base_is_nan(
        #[strategy(any())] mag: f64,
        #[strategy(any())] tan: f64,
        #[strategy(nan())] b: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(b).is_nan());
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(
        #[strategy(nan())] mag: f64,
        #[strategy(any())] tan: f64,
        #[strategy(any())] b: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(b).is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(
        #[strategy(any())] mag: f64,
        #[strategy(nan())] tan: f64,
        #[strategy(any())] b: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.log(b).is_nan());
    }
}
