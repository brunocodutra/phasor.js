#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod phasor;
mod trig;

#[cfg(test)]
mod assert;

pub use crate::phasor::Phasor;
