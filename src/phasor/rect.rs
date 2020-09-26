use super::Phasor;
use core::num::FpCategory;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Phasor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn rect(re: f64, im: f64) -> Self {
        Phasor {
            mag: re.hypot(im).copysign(re),

            tan: if im.classify() == FpCategory::Zero {
                im / re.signum() // := +-{0, PI}
            } else if re.is_infinite() && im.is_infinite() {
                im.signum() / re.signum() // := +-1
            } else {
                im / re
            },
        }
    }
}
