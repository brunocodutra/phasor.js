#![cfg(any(target_arch = "wasm32", test))]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wee_alloc::WeeAlloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(inspectable))]
#[derive(Copy, Clone)]
pub struct Phasor {
    pub mag: f64,
    pub tan: f64,
}

#[allow(clippy::from_over_into)]
impl Into<super::Phasor> for Phasor {
    #[inline]
    fn into(self) -> super::Phasor {
        super::Phasor {
            mag: self.mag,
            tan: self.tan,
        }
    }
}

impl From<super::Phasor> for Phasor {
    #[inline]
    fn from(p: super::Phasor) -> Self {
        Phasor {
            mag: p.mag,
            tan: p.tan,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn i(im: Option<f64>) -> Phasor {
    Phasor::new(im.or(Some(1f64)), Some(f64::INFINITY))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn polar(mag: f64, angle: Option<f64>) -> Phasor {
    super::Phasor::polar(mag, angle.unwrap_or(0f64)).into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn rect(re: f64, im: Option<f64>) -> Phasor {
    super::Phasor::rect(re, im.unwrap_or(0f64)).into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(mag: Option<f64>, tan: Option<f64>) -> Self {
        Phasor {
            mag: mag.unwrap_or_default(),
            tan: tan.unwrap_or_default(),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn norm(&self) -> f64 {
        let p: super::Phasor = (*self).into();
        p.norm()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn angle(&self) -> f64 {
        let p: super::Phasor = (*self).into();
        p.angle()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn real(&self) -> f64 {
        let p: super::Phasor = (*self).into();
        p.real()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn imag(&self) -> f64 {
        let p: super::Phasor = (*self).into();
        p.imag()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn add(&self, rhs: &Phasor) -> Phasor {
        use std::ops::Add;
        let p: super::Phasor = (*self).into();
        let q: super::Phasor = (*rhs).into();
        p.add(q).into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn sub(&self, rhs: &Phasor) -> Phasor {
        use std::ops::Sub;
        let p: super::Phasor = (*self).into();
        let q: super::Phasor = (*rhs).into();
        p.sub(q).into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn mul(&self, rhs: &Phasor) -> Phasor {
        use std::ops::Mul;
        let p: super::Phasor = (*self).into();
        let q: super::Phasor = (*rhs).into();
        p.mul(q).into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn div(&self, rhs: &Phasor) -> Phasor {
        use std::ops::Div;
        let p: super::Phasor = (*self).into();
        let q: super::Phasor = (*rhs).into();
        p.div(q).into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn neg(&self) -> Phasor {
        use std::ops::Neg;
        let p: super::Phasor = (*self).into();
        p.neg().into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn conj(&self) -> Phasor {
        let p: super::Phasor = (*self).into();
        p.conj().into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn recip(&self) -> Phasor {
        let p: super::Phasor = (*self).into();
        p.recip().into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn ln(&self) -> Phasor {
        let p: super::Phasor = (*self).into();
        p.ln().into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn log(&self, base: f64) -> Phasor {
        let p: super::Phasor = (*self).into();
        p.log(base).into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn exp(&self) -> Phasor {
        let p: super::Phasor = (*self).into();
        p.exp().into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn sinh(&self) -> Phasor {
        let p: super::Phasor = (*self).into();
        p.sinh().into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn cosh(&self) -> Phasor {
        let p: super::Phasor = (*self).into();
        p.cosh().into()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isNaN"))]
    pub fn is_nan(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_nan()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isInfinite"))]
    pub fn is_infinite(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_infinite()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isFinite"))]
    pub fn is_finite(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_finite()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isZero"))]
    pub fn is_zero(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_zero()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isSubnormal"))]
    pub fn is_subnormal(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_subnormal()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isNormal"))]
    pub fn is_normal(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_normal()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isReal"))]
    pub fn is_real(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_real()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "isImaginary"))]
    pub fn is_imaginary(&self) -> bool {
        let p: super::Phasor = (*self).into();
        p.is_imaginary()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "absDiffEq"))]
    pub fn abs_diff_eq(&self, rhs: &Phasor, e: Option<f64>) -> bool {
        use crate::AbsDiffEq;
        let p: super::Phasor = (*self).into();
        let q: super::Phasor = (*rhs).into();
        let e = e.unwrap_or_else(super::Phasor::default_epsilon);

        p.abs_diff_eq(&q, e)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "relativeEq"))]
    pub fn relative_eq(&self, rhs: &Phasor, e: Option<f64>, rel: Option<f64>) -> bool {
        use crate::{AbsDiffEq, RelativeEq};
        let p: super::Phasor = (*self).into();
        let q: super::Phasor = (*rhs).into();
        let e = e.unwrap_or_else(super::Phasor::default_epsilon);
        let rel = rel.unwrap_or_else(super::Phasor::default_max_relative);

        p.relative_eq(&q, e, rel)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "ulpsEq"))]
    pub fn ulps_eq(&self, rhs: &Phasor, e: Option<f64>, ulps: Option<f64>) -> bool {
        use crate::{AbsDiffEq, UlpsEq};
        let p: super::Phasor = (*self).into();
        let q: super::Phasor = (*rhs).into();
        let e = e.unwrap_or_else(super::Phasor::default_epsilon);
        let ulps = ulps
            .map(|u| u.max(0f64).min(u32::MAX as f64) as u32)
            .unwrap_or_else(super::Phasor::default_max_ulps);

        p.ulps_eq(&q, e, ulps)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::LN_10;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn new() {
        let p = Phasor::new(None, None);
        let q = Phasor::new(Some(5f64), None);
        let r = Phasor::new(Some(5f64), Some(4f64 / 3f64));

        assert!(p.abs_diff_eq(&rect(0f64, None), None));
        assert!(q.abs_diff_eq(&rect(5f64, None), None));
        assert!(r.abs_diff_eq(&rect(3f64, Some(4f64)), None));

        assert!(p.abs_diff_eq(&polar(0f64, None), None));
        assert!(q.abs_diff_eq(&polar(5f64, None), None));
        assert!(r.abs_diff_eq(&polar(5f64, Some(4f64.atan2(3f64))), None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn real() {
        assert_eq!(rect(3f64, None).real(), 3f64);
        assert_eq!(rect(3f64, Some(4f64)).real(), 3f64);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn imag() {
        assert_eq!(rect(3f64, None).imag(), 0f64);
        assert_eq!(rect(3f64, Some(4f64)).imag(), 4f64);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn norm() {
        assert_eq!(rect(3f64, Some(4f64)).norm(), 5f64);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn angle() {
        assert_eq!(rect(3f64, Some(4f64)).angle(), 4f64.atan2(3f64));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.add(&q).relative_eq(&rect(3f64, Some(4f64)), None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sub() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.sub(&q).relative_eq(&rect(3f64, Some(-4f64)), None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn mul() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.mul(&q).relative_eq(&i(Some(12f64)), None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn div() {
        let p = rect(3f64, None);
        let q = rect(0f64, Some(4f64));
        assert!(p.div(&q).relative_eq(&i(Some(-0.75f64)), None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn neg() {
        let p = rect(3f64, Some(4f64));
        let q = rect(-3f64, Some(-4f64));
        assert!(p.neg().relative_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn conj() {
        let p = rect(3f64, Some(4f64));
        let q = rect(3f64, Some(-4f64));
        assert!(p.conj().relative_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn recip() {
        let p = rect(3f64, Some(4f64));
        let q = rect(3f64 / 25f64, Some(-4f64 / 25f64));
        assert!(p.recip().relative_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn exp() {
        let p = rect(3f64, Some(4f64));
        let q = polar(3f64.exp(), Some(4f64));
        assert!(p.exp().ulps_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn ln() {
        let p = rect(3f64, Some(4f64));
        let q = rect(5f64.ln(), Some(4f64.atan2(3f64)));
        assert!(p.ln().ulps_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn log() {
        let p = rect(3f64, Some(4f64));
        let q = rect(5f64.log10(), Some(4f64.atan2(3f64) / LN_10));
        assert!(p.log(10f64).ulps_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sinh() {
        let p = rect(3f64, Some(4f64));
        let q = polar(
            -(3f64.sinh().hypot(4f64.sin())),
            Some(4f64.tan().atan2(3f64.tanh())),
        );

        assert!(p.sinh().ulps_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn cosh() {
        let p = rect(3f64, Some(4f64));
        let q = polar(
            -(3f64.sinh().hypot(4f64.cos())),
            Some(4f64.tan().atan2(3f64.tanh().recip())),
        );

        assert!(p.cosh().ulps_eq(&q, None, None));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_nan() {
        assert!(!i(Some(0f64)).is_nan());
        assert!(!i(Some(1f64)).is_nan());
        assert!(!i(Some(1E-315f64)).is_nan());
        assert!(!i(Some(f64::INFINITY)).is_nan());
        assert!(i(Some(f64::NAN)).is_nan());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_infinite() {
        assert!(!i(Some(0f64)).is_infinite());
        assert!(!i(Some(1f64)).is_infinite());
        assert!(!i(Some(1E-315f64)).is_infinite());
        assert!(i(Some(f64::INFINITY)).is_infinite());
        assert!(!i(Some(f64::NAN)).is_infinite());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_finite() {
        assert!(i(Some(0f64)).is_finite());
        assert!(i(Some(1f64)).is_finite());
        assert!(i(Some(1E-315f64)).is_finite());
        assert!(!i(Some(f64::INFINITY)).is_finite());
        assert!(!i(Some(f64::NAN)).is_finite());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_zero() {
        assert!(i(Some(0f64)).is_zero());
        assert!(!i(Some(1f64)).is_zero());
        assert!(!i(Some(1E-315f64)).is_zero());
        assert!(!i(Some(f64::INFINITY)).is_zero());
        assert!(!i(Some(f64::NAN)).is_zero());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_subnormal() {
        assert!(!i(Some(0f64)).is_subnormal());
        assert!(!i(Some(1f64)).is_subnormal());
        assert!(i(Some(1E-315f64)).is_subnormal());
        assert!(!i(Some(f64::INFINITY)).is_subnormal());
        assert!(!i(Some(f64::NAN)).is_subnormal());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_normal() {
        assert!(!i(Some(0f64)).is_normal());
        assert!(i(Some(1f64)).is_normal());
        assert!(!i(Some(1E-315f64)).is_normal());
        assert!(!i(Some(f64::INFINITY)).is_normal());
        assert!(!i(Some(f64::NAN)).is_normal());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_real() {
        assert!(rect(3f64, None).is_real());
        assert!(!rect(0f64, Some(4f64)).is_real());
        assert!(!rect(3f64, Some(4f64)).is_real());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn is_imaginary() {
        assert!(!rect(3f64, None).is_imaginary());
        assert!(rect(0f64, Some(4f64)).is_imaginary());
        assert!(!rect(3f64, Some(4f64)).is_imaginary());
    }
}
