#![no_std]

#[cfg(test)]
extern crate alloc;

mod phasor;
mod trig;

pub use crate::phasor::Phasor;
pub use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(target_arch = "wasm32")]
mod js;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod arbitrary;
