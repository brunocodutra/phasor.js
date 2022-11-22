use super::Phasor;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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
    use test_strategy::proptest;

    #[proptest]
    fn has_expected_real_part(#[strategy(finite())] mag: f64, #[strategy(not_nan())] tan: f64) {
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

    #[proptest]
    fn has_expected_imaginary_part(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
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

    #[proptest]
    fn equals_sum_of_exponentials(#[strategy(normal())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        let r = (p + p.recip()) / Phasor::polar(2f64, 0f64);
        assert_ulps_eq!(p.ln().cosh(), r, epsilon = 1E-11, max_ulps = 4_000);
    }

    #[proptest]
    fn is_even(#[strategy(finite())] mag: f64, #[strategy(not_nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert_ulps_eq!(p.cosh(), (-p).cosh());
    }

    #[proptest]
    fn is_real_if_phasor_is_real(#[strategy(not_nan())] mag: f64, #[strategy(zero())] tan: f64) {
        let p = Phasor { mag, tan };
        let r = Phasor {
            mag: mag.cosh(),
            tan,
        };
        assert_ulps_eq!(p.cosh(), r);
    }

    #[proptest]
    fn is_real_if_phasor_is_imaginary(
        #[strategy(regular())] mag: f64,
        #[strategy(infinite())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        let r = Phasor {
            mag: mag.cos(),
            tan: 0f64,
        };
        assert_ulps_eq!(p.cosh(), r);
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(#[strategy(nan())] mag: f64, #[strategy(any())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.cosh().is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(#[strategy(any())] mag: f64, #[strategy(nan())] tan: f64) {
        let p = Phasor { mag, tan };
        assert!(p.cosh().is_nan());
    }
}
