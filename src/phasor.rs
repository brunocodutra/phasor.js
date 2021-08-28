#[cfg(test)]
use test_strategy::Arbitrary;

mod add;
mod angle;
mod approx;
mod classify;
mod conj;
mod cosh;
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
mod sinh;
mod sub;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Phasor {
    pub mag: f64,
    pub tan: f64,
}
