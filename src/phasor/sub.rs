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
    use crate::trig::{cossubatan, tanaddatan};
    use crate::{assert::ulps_or_relative_eq, assert_close_to};
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
        fn subtracting_phasors_with_equal_magnitudes_has_tangent_angle(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(!mag.is_infinite() || cossubatan(t, u) != 1f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 - cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64 + if t < u { -FRAC_PI_2 } else { FRAC_PI_2 },
            );

            if ulps_or_relative_eq(&p, &q, 0f64) {
                assert_close_to!(p - q, -q + p);
            } else {
                assert_close_to!(p - q, r);
                assert_close_to!(q - p, -r);
            }
        }

        #[test]
        fn subtracting_phasors_with_opposite_magnitudes_has_tangent_angle(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(!mag.is_infinite() || cossubatan(t, u) != -1f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag: -mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 + cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64,
            );

            assert_close_to!(p - q, r);
            assert_close_to!(q - p, -r);
        }

        #[test]
        fn subtracting_conjugate_finite_phasors_is_purely_imaginary(mag in not_nan(), tan in not_nan()) {
            prop_assume!(!mag.is_infinite() || cossubatan(tan, -tan) != 1f64);

            let p = Phasor { mag, tan };
            let q = p.conj();
            let r = Phasor { mag: mag * (1f64 - cossubatan(tan, -tan)).sqrt() * SQRT_2, tan: f64::INFINITY.copysign(tan) };

            assert_close_to!(p - q, r);
            assert_close_to!(q - p, -r);
        }

        #[test]
        fn subtracting_equal_finite_phasors_has_zero_magnitude_and_orthogonal_angle(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: 0f64, tan: -tan.recip() };

            assert_close_to!(p - p, r);
        }

        #[test]
        fn subtracting_opposite_phasors_has_double_magnitude(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = -p;
            let r = Phasor { mag: 2f64 * mag, tan };

            assert_close_to!(p - q, r);
            assert_close_to!(q - p, -r);
        }

        #[test]
        fn subtracting_equal_infinite_phasors_is_nan(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };

            assert!((p - p).is_nan());
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
    }
}
