#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
use proptest_derive::Arbitrary;

mod add;
mod angle;
mod approx;
mod classify;
mod conj;
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
mod sub;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(all(test, not(target_arch = "wasm32")), derive(Arbitrary))]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Phasor {
    mag: f64,
    tan: f64,
}
