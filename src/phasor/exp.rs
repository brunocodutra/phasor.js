use super::Phasor;

impl Phasor {
    pub fn exp(self) -> Self {
        Phasor::polar(self.real().exp(), self.imag())
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
    fn has_norm_equal_to_exponential_of_real_part(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.exp().norm(), p.real().exp());
    }

    #[proptest]
    fn has_angle_equal_to_imaginary_part(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.exp().angle(), p.imag().sin().atan2(p.imag().cos()));
    }

    #[proptest]
    fn equals_inverse_of_exponential_of_opposite(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        prop_assume!(mag.exp().classify() == (-mag).exp().recip().classify());

        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.exp(), (-p).exp().recip());
    }

    #[proptest]
    fn equals_one_if_phasor_is_zero(#[strategy(zero())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let r = Phasor {
            mag: 1f64,
            tan: 0f64,
        };
        assert_ulps_eq!(p.exp(), r);
    }

    #[proptest]
    fn is_real_if_phasor_is_real(#[strategy(not_nan())] mag: f64, #[strategy(zero())] tan: f64) {
        let p = Phasor { mag, tan };
        let r = Phasor {
            mag: mag.exp(),
            tan,
        };
        assert_ulps_eq!(p.exp(), r);
    }

    #[proptest]
    fn is_nan_if_phasor_is_infinite_and_not_real(
        #[strategy(infinite())] mag: f64,
        #[strategy(nonzero())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert!(p.exp().is_nan());
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.exp().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.exp().is_nan());
    }
}
