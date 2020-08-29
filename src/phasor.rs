use crate::trig::*;
use core::f64::consts::PI;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
use proptest_derive::Arbitrary;

mod add;
mod approx;
mod display;
mod div;
mod mul;
mod sub;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(all(test, not(target_arch = "wasm32")), derive(Arbitrary))]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Phasor {
    mag: f64,
    tan: f64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn polar(mag: f64, angle: f64) -> Self {
        Phasor {
            mag: mag * angle.cos().signum(),
            tan: angle.tan(),
        }
    }

    #[allow(clippy::float_cmp)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn rect(re: f64, im: f64) -> Self {
        Phasor {
            mag: re.hypot(im).copysign(re),

            tan: if im.abs() == re.abs() {
                // tan(atan2(+-0, +-0) := +-1
                // tan(atan2(+-inf, +-inf) := +-1
                im.signum() / re.signum()
            } else {
                im / re
            },
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn real(&self) -> f64 {
        self.mag * cosatan(self.tan)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn imag(&self) -> f64 {
        self.mag * sinatan(self.tan)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn norm(&self) -> f64 {
        self.mag.abs()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn angle(&self) -> f64 {
        if self.mag.is_sign_positive() {
            self.tan.atan()
        } else {
            self.tan.atan() - PI.copysign(self.tan)
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use alloc::format;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn real(re: f64, im: f64) {
            let p = Phasor::rect(re as f64, im as f64);
            prop_assume!(p.real().is_normal());
            assert_close_to!(p.real(), re as f64);
        }

        #[test]
        fn imag(re: f64, im: f64) {
            let p = Phasor::rect(re as f64, im as f64);
            prop_assume!(p.real().is_normal());
            assert_close_to!(p.imag(), im as f64);
        }

        #[test]
        fn norm(mag: f64, ang: f64) {
            let p = Phasor::polar(mag, ang);
            assert_close_to!(p.norm(), mag.abs());
        }

        #[test]
        fn angle(mag: f64, ang: f64) {
            let p = Phasor::polar(mag, ang);

            assert_close_to!(p.angle().cos(), ang.cos() * mag.signum());
            assert_close_to!(p.angle().sin(), ang.sin() * mag.signum());
        }
    }
}
