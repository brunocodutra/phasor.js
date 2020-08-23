use crate::Phasor;
use wasm_bindgen::prelude::*;

mod display;

#[cfg(test)]
use proptest_derive::Arbitrary;

#[wasm_bindgen]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Rect {
    real: f64,
    imag: f64,
}

#[wasm_bindgen]
impl Rect {
    #[wasm_bindgen(constructor)]
    pub fn new(real: f64, imag: f64) -> Self {
        Rect { real, imag }
    }
}

impl Phasor for Rect {
    fn real(&self) -> f64 {
        self.real
    }

    fn imag(&self) -> f64 {
        self.imag
    }

    fn norm(&self) -> f64 {
        self.real.hypot(self.imag)
    }

    fn angle(&self) -> f64 {
        self.imag.atan2(self.real)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn real(re: f64, mut im: f64) {
            let rect = Rect::new(re, im);
            assert_close_to!(rect.real(), re);
        }

        #[test]
        fn imag(re: f64, mut im: f64) {
            let rect = Rect::new(re, im);
            assert_close_to!(rect.imag(), im);
        }

        #[test]
        fn norm(re: f64, im: f64) {
            let rect = Rect::new(re, im);
            assert_close_to!(rect.norm(), re.hypot(im));
        }

        #[test]
        fn angle(re: f64, mut im: f64) {
            let rect = Rect::new(re, im);
            assert_close_to!(rect.angle(), im.atan2(re));
        }
    }
}
