use super::Phasor;
use std::ops::Sub;

impl Sub for Phasor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::trig::{cossubatan, tanaddatan};
    use approx::{assert_ulps_eq, ulps_eq};
    use proptest::prelude::*;
    use std::f64::consts::{FRAC_PI_2, SQRT_2};

    proptest! {
        #[test]
        fn is_anti_commutative(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            prop_assume!(!ulps_eq!(p, q));

            assert_ulps_eq!(p - q, -(q - p));
        }

        #[test]
        fn equals_subtraction_of_real_and_imaginary_parts(a in normal(), b in not_nan(), c in normal(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };
            let r = Phasor::rect(p.real() - q.real(), p.imag() - q.imag());

            assert_ulps_eq!(p - q, r, max_ulps = 800);
            assert_ulps_eq!(q - p, -r, max_ulps = 800);
        }

        #[test]
        fn has_zero_as_identity(a in nonzero(), b in not_nan(), c in zero(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_ulps_eq!(p - q, p);
            assert_ulps_eq!(q - p, -p);
        }

        #[test]
        fn has_infinity_as_identity(a in finite(), b in not_nan(), c in infinite(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_ulps_eq!(p - q, -q);
            assert_ulps_eq!(q - p, q);
        }

        #[test]
        fn has_tangent_angle_if_magnitudes_are_equal(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(cossubatan(t, u) < 0.9f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 - cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64 + if t < u { -FRAC_PI_2 } else { FRAC_PI_2 },
            );

            assert_ulps_eq!(p - q, r, max_ulps = 80);
            assert_ulps_eq!(q - p, -r, max_ulps = 80);
        }

        #[test]
        fn has_tangent_angle_if_magnitudes_are_opposite(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(cossubatan(t, u) > -0.9f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag: -mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 + cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64,
            );

            assert_ulps_eq!(p - q, r, max_ulps = 80);
            assert_ulps_eq!(q - p, -r, max_ulps = 80);
        }

        #[test]
        fn is_imaginary_if_phasors_are_conjugate(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = p.conj();

            let r = Phasor {
                mag: (p.imag() - q.imag()).copysign(mag),
                tan: f64::INFINITY.copysign(tan)
            };

            prop_assume!(!p.is_infinite() || !p.is_real());

            assert_ulps_eq!(p - q, r, max_ulps = 40);
            assert_ulps_eq!(q - p, -r, max_ulps = 40);
        }

        #[test]
        fn has_zero_magnitude_and_orthogonal_angle_if_phasors_are_equal(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: 0f64, tan: -tan.recip() };

            assert_ulps_eq!(p - p, r);
        }

        #[test]
        fn has_double_magnitude_if_phasors_are_opposite(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = -p;
            let r = Phasor { mag: 2f64 * mag, tan };

            assert_ulps_eq!(p - q, r);
            assert_ulps_eq!(q - p, -r);
        }

        #[test]
        fn is_nan_if_phasors_are_equal_and_infinite(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };

            assert!((p - p).is_nan());
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(a in any(), b in any(), c in nan(), d in any()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p - q).is_nan());
            assert!((q - p).is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(a in any(), b in any(), c in any(), d in nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p - q).is_nan());
            assert!((q - p).is_nan());
        }
    }
}
