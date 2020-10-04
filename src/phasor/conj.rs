use super::Phasor;

impl Phasor {
    pub fn conj(self) -> Phasor {
        Phasor {
            mag: self.mag,
            tan: -self.tan,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::assert_close_to;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn preserves_real_part(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.conj().real(), p.real());
        }

        #[test]
        fn negates_imaginary_part(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.conj().imag(), -p.imag());
        }

        #[test]
        fn is_its_own_inverse_function(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            assert_close_to!(p.conj().conj(), p);
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(mag in nan(), tan in any()) {
            let p = Phasor { mag, tan };
            assert!(p.conj().is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(mag in any(), tan in nan()) {
            let p = Phasor { mag, tan };
            assert!(p.conj().is_nan());
        }
    }
}
