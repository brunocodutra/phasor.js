#![no_std]

#[cfg(test)]
extern crate alloc;

mod js;
mod phasor;
mod trig;

pub use crate::phasor::Phasor;
pub use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(test)]
mod arbitrary;
