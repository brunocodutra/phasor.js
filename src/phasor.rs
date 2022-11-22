#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(test)]
use test_strategy::Arbitrary;

mod add;
mod angle;
mod approx;
mod classify;
mod conj;
mod cosh;
mod display;
mod div;
mod exp;
mod imag;
mod ln;
mod log;
mod mul;
mod neg;
mod norm;
mod polar;
mod real;
mod recip;
mod rect;
mod sinh;
mod sub;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(inspectable))]
pub struct Phasor {
    pub mag: f64,
    pub tan: f64,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Phasor {
    #[wasm_bindgen(constructor)]
    pub fn new(mag: Option<f64>, tan: Option<f64>) -> Self {
        Phasor {
            mag: mag.unwrap_or_default(),
            tan: tan.unwrap_or_default(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn i(im: Option<f64>) -> Phasor {
    Phasor::new(im.or(Some(1f64)), Some(f64::INFINITY))
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod test {
    use super::*;
    use polar::polar;
    use rect::rect;
    use std::f64::consts::{LN_10, PI};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn constructor() {
        let p = Phasor::new(None, None);
        let q = Phasor::new(Some(5f64), None);
        let r = Phasor::new(None, Some(4f64 / 3f64));
        let s = Phasor::new(Some(5f64), Some(4f64 / 3f64));

        assert!(p.abs_diff_eq(&rect(0f64, None), None));
        assert!(q.abs_diff_eq(&rect(5f64, None), None));
        assert!(s.abs_diff_eq(&rect(3f64, Some(4f64)), None));

        assert!(p.abs_diff_eq(&polar(0f64, None), None));
        assert!(q.abs_diff_eq(&polar(5f64, None), None));
        assert!(r.abs_diff_eq(&polar(0f64, Some(4f64.atan2(3f64))), None));
        assert!(s.abs_diff_eq(&polar(5f64, Some(4f64.atan2(3f64))), None));
    }

    #[wasm_bindgen_test]
    fn real() {
        assert_eq!(i(Some(4f64)).real(), 0f64);
        assert_eq!(rect(3f64, None).real(), 3f64);
        assert_eq!(rect(3f64, Some(4f64)).real(), 3f64);
    }

    #[wasm_bindgen_test]
    fn imag() {
        assert_eq!(i(Some(4f64)).imag(), 4f64);
        assert_eq!(rect(3f64, None).imag(), 0f64);
        assert_eq!(rect(3f64, Some(4f64)).imag(), 4f64);
    }

    #[wasm_bindgen_test]
    fn norm() {
        assert_eq!(i(Some(4f64)).norm(), 4f64);
        assert_eq!(rect(3f64, Some(4f64)).norm(), 5f64);
    }

    #[wasm_bindgen_test]
    fn angle() {
        assert_eq!(i(Some(4f64)).angle(), PI / 2f64);
        assert_eq!(rect(3f64, Some(4f64)).angle(), 4f64.atan2(3f64));
    }

    #[wasm_bindgen_test]
    fn add() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.add(&q).relative_eq(&rect(3f64, Some(4f64)), None, None));
    }

    #[wasm_bindgen_test]
    fn sub() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.sub(&q).relative_eq(&rect(3f64, Some(-4f64)), None, None));
    }

    #[wasm_bindgen_test]
    fn mul() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.mul(&q).relative_eq(&i(Some(12f64)), None, None));
    }

    #[wasm_bindgen_test]
    fn div() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.div(&q).relative_eq(&i(Some(-0.75f64)), None, None));
    }

    #[wasm_bindgen_test]
    fn neg() {
        let p = rect(3f64, Some(4f64));
        let q = rect(-3f64, Some(-4f64));
        assert!(p.neg().relative_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn conj() {
        let p = rect(3f64, Some(4f64));
        let q = rect(3f64, Some(-4f64));
        assert!(p.conj().relative_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn recip() {
        let p = rect(3f64, Some(4f64));
        let q = rect(3f64 / 25f64, Some(-4f64 / 25f64));
        assert!(p.recip().relative_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn exp() {
        let p = rect(3f64, Some(4f64));
        let q = polar(3f64.exp(), Some(4f64));
        assert!(p.exp().ulps_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn ln() {
        let p = rect(3f64, Some(4f64));
        let q = rect(5f64.ln(), Some(4f64.atan2(3f64)));
        assert!(p.ln().ulps_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn log() {
        let p = rect(3f64, Some(4f64));
        let q = rect(5f64.log10(), Some(4f64.atan2(3f64) / LN_10));
        assert!(p.log(10f64).ulps_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn sinh() {
        let p = rect(3f64, Some(4f64));
        let q = polar(
            -(3f64.sinh().hypot(4f64.sin())),
            Some(4f64.tan().atan2(3f64.tanh())),
        );

        assert!(p.sinh().ulps_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn cosh() {
        let p = rect(3f64, Some(4f64));
        let q = polar(
            -(3f64.sinh().hypot(4f64.cos())),
            Some(4f64.tan().atan2(3f64.tanh().recip())),
        );

        assert!(p.cosh().ulps_eq(&q, None, None));
    }

    #[wasm_bindgen_test]
    fn is_nan() {
        assert!(!i(Some(0f64)).is_nan());
        assert!(!i(Some(1f64)).is_nan());
        assert!(!i(Some(1E-315f64)).is_nan());
        assert!(!i(Some(f64::INFINITY)).is_nan());
        assert!(i(Some(f64::NAN)).is_nan());
    }

    #[wasm_bindgen_test]
    fn is_infinite() {
        assert!(!i(Some(0f64)).is_infinite());
        assert!(!i(Some(1f64)).is_infinite());
        assert!(!i(Some(1E-315f64)).is_infinite());
        assert!(i(Some(f64::INFINITY)).is_infinite());
        assert!(!i(Some(f64::NAN)).is_infinite());
    }

    #[wasm_bindgen_test]
    fn is_finite() {
        assert!(i(Some(0f64)).is_finite());
        assert!(i(Some(1f64)).is_finite());
        assert!(i(Some(1E-315f64)).is_finite());
        assert!(!i(Some(f64::INFINITY)).is_finite());
        assert!(!i(Some(f64::NAN)).is_finite());
    }

    #[wasm_bindgen_test]
    fn is_zero() {
        assert!(i(Some(0f64)).is_zero());
        assert!(!i(Some(1f64)).is_zero());
        assert!(!i(Some(1E-315f64)).is_zero());
        assert!(!i(Some(f64::INFINITY)).is_zero());
        assert!(!i(Some(f64::NAN)).is_zero());
    }

    #[wasm_bindgen_test]
    fn is_subnormal() {
        assert!(!i(Some(0f64)).is_subnormal());
        assert!(!i(Some(1f64)).is_subnormal());
        assert!(i(Some(1E-315f64)).is_subnormal());
        assert!(!i(Some(f64::INFINITY)).is_subnormal());
        assert!(!i(Some(f64::NAN)).is_subnormal());
    }

    #[wasm_bindgen_test]
    fn is_normal() {
        assert!(!i(Some(0f64)).is_normal());
        assert!(i(Some(1f64)).is_normal());
        assert!(!i(Some(1E-315f64)).is_normal());
        assert!(!i(Some(f64::INFINITY)).is_normal());
        assert!(!i(Some(f64::NAN)).is_normal());
    }

    #[wasm_bindgen_test]
    fn is_real() {
        assert!(rect(3f64, None).is_real());
        assert!(!rect(0f64, Some(4f64)).is_real());
        assert!(!rect(3f64, Some(4f64)).is_real());
    }

    #[wasm_bindgen_test]
    fn is_imaginary() {
        assert!(!rect(3f64, None).is_imaginary());
        assert!(rect(0f64, Some(4f64)).is_imaginary());
        assert!(!rect(3f64, Some(4f64)).is_imaginary());
    }
}
