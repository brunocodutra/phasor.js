use crate::trig::*;
use core::num::FpCategory;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
use proptest_derive::Arbitrary;

mod add;
mod angle;
mod approx;
mod classify;
mod display;
mod div;
mod imag;
mod mul;
mod norm;
mod real;
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
