use super::Phasor;
use std::num::FpCategory::Zero;

impl Phasor {
    pub fn sinh(self) -> Self {
        let re = self.real();
        let im = self.imag();

        if re.classify() != Zero {
            Phasor {
                mag: re.sinh().hypot(im.sin()).copysign(im.cos()) * 1f64.copysign(re),
                tan: im.tan() / re.tanh(),
            }
        } else if im.classify() != Zero {
            Phasor {
                mag: im.sin(),
                tan: f64::INFINITY,
            }
        } else {
            self
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
    fn has_expected_real_part(#[strategy(finite())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(
            p.sinh().real(),
            if p.imag().cos().abs() > 0f64 {
                p.real().sinh() * p.imag().cos()
            } else {
                0f64
            }
        );
    }

    #[proptest]
    fn has_expected_imaginary_part(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(
            p.sinh().imag(),
            if p.imag().sin().abs() > 0f64 {
                p.real().cosh() * p.imag().sin()
            } else {
                0f64
            }
        );
    }

    #[proptest]
    fn equals_subtraction_of_exponentials(
        #[strategy(normal())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        let r = (p - p.recip()) / Phasor::polar(2f64, 0f64);
        assert_ulps_eq!(p.ln().sinh(), r, epsilon = 1E-11, max_ulps = 4_000);
    }

    #[proptest]
    fn is_odd(#[strategy(finite())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.sinh(), -(-p).sinh());
    }

    #[proptest]
    fn is_real_if_phasor_is_real(#[strategy(not_nan())] mag: f64, #[strategy(zero())] tan: f64) {
        let p = Phasor { mag, tan };
        let r = Phasor {
            mag: mag.sinh(),
            tan,
        };
        assert_ulps_eq!(p.sinh(), r);
    }

    #[proptest]
    fn is_imaginary_if_phasor_is_imaginary(
        #[strategy(regular())] mag: f64,
        #[strategy(infinite())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        let r = Phasor {
            mag: mag.sin(),
            tan,
        };
        assert_ulps_eq!(p.sinh(), r);
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.sinh().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.sinh().is_nan());
    }
}
