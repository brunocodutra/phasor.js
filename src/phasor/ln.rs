use super::Phasor;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    pub fn ln(self) -> Self {
        Phasor::rect(self.norm().ln(), self.angle())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use approx::assert_ulps_eq;
    use test_strategy::proptest;

    #[proptest]
    fn has_real_part_equal_to_logarithm_of_norm(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.ln().real(), p.norm().ln());
    }

    #[proptest]
    fn has_imaginary_part_equal_to_angle(
        #[strategy(regular())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.ln().imag(), p.angle());
    }

    #[proptest]
    fn equals_opposite_of_logarithm_of_inverse(
        #[strategy(normal())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.ln(), -p.recip().ln(), epsilon = 1E-15);
    }

    #[proptest]
    fn is_imaginary_if_phasor_is_not_one(
        #[strategy(zero())] mag: f64,
        #[strategy(nonzero())] tan: f64,
    ) {
        let p = Phasor {
            mag: mag.signum(),
            tan,
        };
        let r = Phasor {
            mag: p.angle(),
            tan: f64::INFINITY,
        };
        assert_ulps_eq!(p.ln(), r);
    }

    #[proptest]
    fn is_real_if_phasor_is_real_and_positive(
        #[strategy(not_nan())] mag: f64,
        #[strategy(zero())] tan: f64,
    ) {
        let p = Phasor {
            mag: mag.abs(),
            tan,
        };
        let r = Phasor {
            mag: mag.abs().ln(),
            tan,
        };
        assert_ulps_eq!(p.ln(), r);
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.ln().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.ln().is_nan());
    }
}
