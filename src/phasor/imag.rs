use crate::{trig::sinatan, Phasor};

impl Phasor {
    pub fn imag(&self) -> f64 {
        let s = sinatan(self.tan);
        if s.abs() > 0f64 {
            s * self.mag
        } else {
            s * self.mag.signum()
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
    fn equals_product_of_magnitude_and_sine_of_angle(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.imag(), mag * sinatan(tan));
    }

    #[proptest]
    fn is_zero_if_phasor_is_real(#[strategy(not_nan())] mag: f64, #[strategy(zero())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.imag(), 0f64);
    }

    #[proptest]
    fn equals_magnitude_if_phasor_is_imaginary(
        #[strategy(not_nan())] mag: f64,
        #[strategy(infinite())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.imag(), mag * tan.signum());
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.imag().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.imag().is_nan());
    }
}
