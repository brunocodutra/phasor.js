#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod phasor;
mod polar;
mod rect;
mod trig;

#[cfg(test)]
mod cmp;

pub use crate::phasor::Phasor;
pub use crate::polar::Polar;
pub use crate::rect::Rect;
