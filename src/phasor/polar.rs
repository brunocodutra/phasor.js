use super::Phasor;

impl Phasor {
    pub fn polar(mag: f64, angle: f64) -> Self {
        Phasor {
            mag: mag * angle.cos().signum(),
            tan: angle.tan(),
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
    fn preserves_modulus_of_magnitude(
        #[strategy(not_nan())] mag: f64,
        #[strategy(finite())] ang: f64,
    ) {
        let p = Phasor::polar(mag, ang);
        assert_ulps_eq!(p.norm(), mag.abs());
    }

    #[proptest]
    fn preserves_angle_if_magnitude_is_positive(
        #[strategy(not_nan())] mag: f64,
        #[strategy(finite())] ang: f64,
    ) {
        let p = Phasor::polar(mag.abs(), ang);
        assert_ulps_eq!(p.angle(), ang.sin().atan2(ang.cos()));
    }

    #[proptest]
    fn is_real_if_angle_is_zero(#[strategy(not_nan())] mag: f64, #[strategy(zero())] ang: f64) {
        let p = Phasor::polar(mag, ang);
        assert!(p.is_real());
    }

    #[proptest]
    fn is_nan_if_angle_is_infinite(
        #[strategy(not_nan())] mag: f64,
        #[strategy(infinite())] ang: f64,
    ) {
        let p = Phasor::polar(mag, ang);
        assert!(p.is_nan());
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] ang: f64) {
        let p = Phasor::polar(mag, ang);
        assert!(p.is_nan());
    }

    #[proptest]
    fn is_nan_if_angle_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] ang: f64) {
        let p = Phasor::polar(mag, ang);
        assert!(p.is_nan());
    }
}
