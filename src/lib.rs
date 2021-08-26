#![no_std]
#![cfg_attr(test, allow(clippy::float_cmp, clippy::eq_op))]

#[cfg(test)]
extern crate alloc;

mod js;
mod phasor;
mod trig;

pub use crate::phasor::Phasor;
pub use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(test)]
mod arbitrary;
