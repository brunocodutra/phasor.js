use super::Phasor;
use core::ops::Sub;

impl Sub for Phasor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::{assert::ulps_or_relative_eq, assert_close_to, trig::cossubatan};
    use alloc::format;
    use core::f64::consts::{FRAC_PI_2, SQRT_2};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn subtracting_finite_nonzero_phasors_equals_difference_of_real_and_imaginary_parts(a in regular(), b in not_nan(), c in regular(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };
            let r = Phasor::rect(p.real() - q.real(), p.imag() - q.imag());

            assert_close_to!(p - q, r);
            assert_close_to!(q - p, -r);
        }

        #[test]
        fn subtracting_zero_has_no_effect(a in nonzero(), b in not_nan(), c in zero(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_close_to!(p - q, p);
            assert_close_to!(q - p, -p);
        }

        #[test]
        fn subtracting_infinity_has_no_effect(a in finite(), b in not_nan(), c in infinite(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_close_to!(p - q, -q);
            assert_close_to!(q - p, q);
        }

        #[test]
        fn subtracting_phasor_that_has_undefined_magnitude_is_nan(a in any(), b in any(), c in nan(), d in any()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p - q).is_nan());
            assert!((q - p).is_nan());
        }

        #[test]
        fn subtracting_phasor_that_has_undefined_tangent_is_nan(a in any(), b in any(), c in any(), d in nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p - q).is_nan());
            assert!((q - p).is_nan());
        }

        #[test]
        fn subtracting_phasors_with_the_same_magnitude_has_tangent_angle(m in not_nan(), t in not_nan(), u in not_nan()) {
            let p = Phasor { mag: m, tan: t };
            let q = Phasor { mag: m, tan: u };

            prop_assume!(!ulps_or_relative_eq(&p, &q, 0f64));

            let r = Phasor::polar(
                m * (1f64 - cossubatan(t, u)).sqrt() * SQRT_2,
                (t.atan() + u.atan()) / 2f64 + if t > u { FRAC_PI_2 } else { -FRAC_PI_2 },
            );

            assert_close_to!(p - q, r);
            assert_close_to!(q - p, -r);
        }

        #[test]
        fn subtracting_opposite_phasors_has_double_magnitude(a in not_nan(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = -p;
            let r = Phasor { mag: 2f64 * a, tan: b };

            assert_close_to!(p - q, r);
        }

        #[test]
        fn subtracting_equal_finite_phasors_has_zero_magnitude_and_orthogonal_angle(a in finite(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let r = Phasor { mag: 0f64, tan: -b.recip() };

            assert_close_to!(p - p, r);
        }

        #[test]
        fn subtracting_finite_phasor_to_its_conjugate_is_purely_imaginary(a in finite(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.conj();

            assert!((p - q).is_imaginary());
            assert!((q - p).is_imaginary());
        }

        #[test]
        fn subtracting_equal_infinite_phasors_is_nan(a in infinite(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };

            assert!((p - p).is_nan());
        }
    }
}
