#![cfg_attr(test, allow(clippy::float_cmp, clippy::eq_op))]

mod phasor;
mod trig;

#[cfg(test)]
mod arbitrary;

pub use crate::phasor::Phasor;
pub use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
