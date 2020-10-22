use super::Phasor;
use core::num::FpCategory::Zero;

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

#[cfg(all(test, not(target_arch = "wasm32")))]
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
                p.sinh().real(),
                if p.imag().cos().abs() > 0f64 {
                    p.real().sinh() * p.imag().cos()
                } else {
                    0f64
                }
            );
        }

        #[test]
        fn has_expected_imaginary_part(mag in finite(), tan in not_nan()) {
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

        #[test]
        fn equals_subtraction_of_exponentials(mag in normal(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = (p - p.recip()) / Phasor::polar(2f64, 0f64);
            assert_ulps_eq!(p.ln().sinh(), r, epsilon = 1E-11, max_ulps = 4_000);
        }

        #[test]
        fn is_odd(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_ulps_eq!(p.sinh(), -(-p).sinh());
        }

        #[test]
        fn is_real_if_phasor_is_real(mag in not_nan(), tan in zero()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: mag.sinh(), tan };
            assert_ulps_eq!(p.sinh(), r);
        }

        #[test]
        fn is_imaginary_if_phasor_is_imaginary(mag in regular(), tan in infinite()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: mag.sin(), tan };
            assert_ulps_eq!(p.sinh(), r);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.sinh().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.sinh().is_nan());
        }
    }
}
