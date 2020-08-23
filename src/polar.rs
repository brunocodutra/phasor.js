use crate::{trig::*, Phasor};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

mod display;

#[cfg(test)]
use proptest_derive::Arbitrary;

#[wasm_bindgen]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Polar {
    mag: f64,
    tan: f64,
}

#[wasm_bindgen]
impl Polar {
    #[wasm_bindgen(constructor)]
    pub fn new(mag: f64, mut angle: f64) -> Self {
        angle %= 2f64 * PI;

        let n = (2f64 * angle / PI).abs();

        Polar {
            mag: if n <= 1f64 || n > 3f64 { mag } else { -mag },
            tan: angle.tan(),
        }
    }
}

impl Phasor for Polar {
    fn real(&self) -> f64 {
        self.mag * cosatan(self.tan)
    }

    fn imag(&self) -> f64 {
        self.mag * sinatan(self.tan)
    }

    fn norm(&self) -> f64 {
        self.mag.abs()
    }

    fn angle(&self) -> f64 {
        if self.mag.is_sign_positive() {
            self.tan.atan()
        } else {
            self.tan.atan() - PI.copysign(self.tan)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn real(mag: f64, mut ang: f64) {
            let polar = Polar::new(mag, ang);

            ang %= 2f64 * PI;
            assert_close_to!(polar.real(), mag * ang.cos());
        }

        #[test]
        fn imag(mag: f64, mut ang: f64) {
            let polar = Polar::new(mag, ang);

            ang %= 2f64 * PI;
            assert_close_to!(polar.imag(), mag * ang.sin());
        }

        #[test]
        fn norm(mag: f64, ang: f64) {
            let polar = Polar::new(mag, ang);
            assert_close_to!(polar.norm(), mag.abs());
        }

        #[test]
        fn angle(mag: f64, mut ang: f64) {
            let polar = Polar::new(mag, ang);

            ang %= 2f64 * PI;
            assert_close_to!(polar.angle().cos(), ang.cos() * mag.signum());
            assert_close_to!(polar.angle().sin(), ang.sin() * mag.signum());
        }
    }
}
