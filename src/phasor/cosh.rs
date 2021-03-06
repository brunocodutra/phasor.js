use super::Phasor;

impl Phasor {
    pub fn cosh(self) -> Self {
        let re = self.real();
        let im = self.imag();

        Phasor {
            mag: re.sinh().hypot(im.cos()).copysign(im.cos()),
            tan: im.tan() * re.tanh(),
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
        fn has_expected_real_part(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(
                p.cosh().real(),
                if p.imag().cos().abs() > 0f64 {
                    p.real().cosh() * p.imag().cos()
                } else {
                    0f64
                }
            );
        }

        #[test]
        fn has_expected_imaginary_part(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(
                p.cosh().imag(),
                if p.imag().sin().abs() > 0f64 {
                    p.real().sinh() * p.imag().sin()
                } else {
                    0f64
                }
            );
        }

        #[test]
        fn equals_sum_of_exponentials(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = (p + p.recip()) / Phasor::polar(2f64, 0f64);
            assert_ulps_eq!(p.ln().cosh(), r, epsilon = 1E-11, max_ulps = 4_000);
        }

        #[test]
        fn is_even(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.cosh(), (-p).cosh());
        }

        #[test]
        fn is_real_if_phasor_is_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: mag.cosh(), tan };
            assert_ulps_eq!(p.cosh(), r);
        }

        #[test]
        fn is_real_if_phasor_is_imaginary(mag in regular(), tan in infinite()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: mag.cos(), tan: 0f64 };
            assert_ulps_eq!(p.cosh(), r);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.cosh().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.cosh().is_nan());
        }
    }
}
