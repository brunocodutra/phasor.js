use crate::Phasor;
use core::f64::consts::PI;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
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
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn angle(mag: f64, ang: f64) {
            let p = Phasor::polar(mag, ang);
            assert_close_to!(p.angle().cos(), ang.cos() * mag.signum());
            assert_close_to!(p.angle().sin(), ang.sin() * mag.signum());
        }
    }
}
