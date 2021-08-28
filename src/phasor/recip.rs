use super::Phasor;

impl Phasor {
    pub fn recip(self) -> Phasor {
        Phasor {
            mag: self.mag.recip(),
            tan: -self.tan,
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
    fn inverts_norm(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.recip().norm(), p.norm().recip());
    }

    #[proptest]
    fn negates_angle(#[strategy(not_nan())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.recip().angle(), -p.angle());
    }

    #[proptest]
    fn is_its_own_inverse_function(
        #[strategy(normal())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.recip().recip(), p);
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.recip().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.recip().is_nan());
    }
}
