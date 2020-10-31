use super::Phasor;
use crate::trig::{cosatan, cosatan2, sinatan, sinatan2};
use core::{num::FpCategory::Zero, ops::Add};

impl Add for Phasor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let u = sinatan2(self.mag, rhs.mag);
        let v = cosatan2(self.mag, rhs.mag);

        let (ure, uim) = (u * cosatan(self.tan), u * sinatan(self.tan));
        let (vre, vim) = (v * cosatan(rhs.tan), v * sinatan(rhs.tan));
        let (re, im) = (ure + vre, uim + vim);

        Phasor {
            mag: if u.abs() > v.abs() {
                (re / u).hypot(im / u) * self.mag.copysign(re)
            } else {
                (re / v).hypot(im / v) * rhs.mag.copysign(re)
            },

            tan: if im.classify() != Zero || re.classify() != Zero {
                im / re
            } else if ure.is_sign_negative() == vre.is_sign_negative() {
                ure
            } else if uim.is_sign_negative() == vim.is_sign_negative() {
                uim.recip()
            } else {
                -self.tan.recip()
            },
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::trig::{cossubatan, tanaddatan};
    use alloc::format;
    use approx::{assert_ulps_eq, ulps_eq};
    use core::f64::consts::{FRAC_PI_2, SQRT_2};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn is_commutative(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            prop_assume!(!p.is_infinite() || !q.is_infinite() || !ulps_eq!(p, -q));

            assert_ulps_eq!(p + q, q + p);
        }

        #[test]
        fn equals_sum_of_real_and_imaginary_parts(a in normal(), b in not_nan(), c in normal(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };
            let r = Phasor::rect(p.real() + q.real(), p.imag() + q.imag());

            assert_ulps_eq!(p + q, r, max_ulps = 800);
            assert_ulps_eq!(q + p, r, max_ulps = 800);
        }

        #[test]
        fn has_zero_as_identity(a in nonzero(), b in not_nan(), c in zero(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_ulps_eq!(p + q, p);
            assert_ulps_eq!(q + p, p);
        }

        #[test]
        fn has_infinity_as_identity(a in finite(), b in not_nan(), c in infinite(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_ulps_eq!(p + q, q);
            assert_ulps_eq!(q + p, q);
        }

        #[test]
        fn has_bisector_angle_if_magnitudes_are_equal(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(cossubatan(t, u) > -0.9f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 + cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64,
            );

            assert_ulps_eq!(p + q, r, max_ulps = 80);
            assert_ulps_eq!(q + p, r, max_ulps = 80);
        }

        #[test]
        fn has_bisector_angle_if_magnitudes_are_opposite(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(cossubatan(t, u) < 0.9f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag: -mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 - cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64 + if t < u { -FRAC_PI_2 } else { FRAC_PI_2 },
            );

            assert_ulps_eq!(p + q, r, max_ulps = 80);
            assert_ulps_eq!(q + p, r, max_ulps = 80);
        }

        #[test]
        fn is_real_if_phasors_are_conjugate(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = p.conj();

            let r = Phasor {
                mag: p.real() + q.real(),
                tan: 0f64
            };

            prop_assume!(!p.is_infinite() || !p.is_imaginary());

            assert_ulps_eq!(p + q, r, max_ulps = 40);
            assert_ulps_eq!(q + p, r, max_ulps = 40);
        }

        #[test]
        fn has_double_magnitude_if_phasors_are_equal(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: 2f64 * mag, tan };

            assert_ulps_eq!(p + p, r);
        }

        #[test]
        fn is_zero_if_phasors_are_finite_and_opposite(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = -p;
            let r = Phasor { mag: 0f64, tan: -tan.recip() };

            assert_ulps_eq!(p + q, r);
            assert_ulps_eq!(q + p, r);
        }

        #[test]
        fn is_nan_if_phasors_are_opposite_and_infinite(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = -p;

            assert!((p + q).is_nan());
            assert!((q + p).is_nan());
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(a in any(), b in any(), c in nan(), d in any()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p + q).is_nan());
            assert!((q + p).is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(a in any(), b in any(), c in any(), d in nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p + q).is_nan());
            assert!((q + p).is_nan());
        }
    }
}
