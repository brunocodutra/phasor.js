use crate::{trig::cosatan, Phasor};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn real(&self) -> f64 {
        self.mag * cosatan(self.tan)
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
    }
}
