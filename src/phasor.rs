use crate::trig::*;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

mod approx;
mod display;

#[cfg(all(test, not(target_arch = "wasm32")))]
use proptest_derive::Arbitrary;

#[wasm_bindgen]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(all(test, not(target_arch = "wasm32")), derive(Arbitrary))]
pub struct Phasor {
    mag: f64,
    tan: f64,
}

#[wasm_bindgen]
impl Phasor {
    #[wasm_bindgen]
    pub fn polar(mag: f64, mut angle: f64) -> Self {
        angle %= 2f64 * PI;

        let n = (2f64 * angle / PI).abs();

        Phasor {
            mag: if n <= 1f64 || n > 3f64 { mag } else { -mag },
            tan: angle.tan(),
        }
    }

    #[wasm_bindgen]
    #[allow(clippy::float_cmp)]
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

    #[wasm_bindgen]
    pub fn real(&self) -> f64 {
        self.mag * cosatan(self.tan)
    }

    #[wasm_bindgen]
    pub fn imag(&self) -> f64 {
        self.mag * sinatan(self.tan)
    }

    #[wasm_bindgen]
    pub fn norm(&self) -> f64 {
        self.mag.abs()
    }

    #[wasm_bindgen]
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
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn real(re: f32, im: f32) {
            let p = Phasor::rect(re as f64, im as f64);
            assert_close_to!(p.real(), re as f64);
        }

        #[test]
        fn imag(re: f32, im: f32) {
            let p = Phasor::rect(re as f64, im as f64);
            assert_close_to!(p.imag(), im as f64);
        }

        #[test]
        fn norm(mag: f64, ang: f64) {
            let p = Phasor::polar(mag, ang);
            assert_close_to!(p.norm(), mag.abs());
        }

        #[test]
        fn angle(mag: f64, mut ang: f64) {
            let p = Phasor::polar(mag, ang);

            ang %= 2f64 * PI;
            assert_close_to!(p.angle().cos(), ang.cos() * mag.signum());
            assert_close_to!(p.angle().sin(), ang.sin() * mag.signum());
        }
    }
}
