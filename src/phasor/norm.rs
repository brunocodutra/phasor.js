use crate::Phasor;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn norm(&self) -> f64 {
        self.mag.abs()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn norm(mag: f64, ang: f64) {
            let p = Phasor::polar(mag, ang);
            assert_close_to!(p.norm(), mag.abs());
        }
    }
}
