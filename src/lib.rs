#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod phasor;

#[cfg(test)]
mod cmp;

pub use crate::phasor::Phasor;
