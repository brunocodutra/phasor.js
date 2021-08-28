use super::Phasor;
use crate::trig::{cosatan, cosatan2, sinatan, sinatan2};
use std::{num::FpCategory::Zero, ops::Add};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::trig::{cossubatan, tanaddatan};
    use approx::{assert_ulps_eq, ulps_eq};
    use proptest::prop_assume;
    use std::f64::consts::{FRAC_PI_2, SQRT_2};
    use test_strategy::proptest;

    #[proptest]
    fn is_commutative(
        #[strategy(not_nan())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(not_nan())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        prop_assume!(!p.is_infinite() || !q.is_infinite() || !ulps_eq!(p, -q));

        assert_ulps_eq!(p + q, q + p);
    }

    #[proptest]
    fn equals_sum_of_real_and_imaginary_parts(
        #[strategy(normal())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(normal())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };
        let r = Phasor::rect(p.real() + q.real(), p.imag() + q.imag());

        assert_ulps_eq!(p + q, r, max_ulps = 800);
        assert_ulps_eq!(q + p, r, max_ulps = 800);
    }

    #[proptest]
    fn has_zero_as_identity(
        #[strategy(nonzero())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(zero())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        assert_ulps_eq!(p + q, p);
        assert_ulps_eq!(q + p, p);
    }

    #[proptest]
    fn has_infinity_as_identity(
        #[strategy(finite())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(infinite())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        assert_ulps_eq!(p + q, q);
        assert_ulps_eq!(q + p, q);
    }

    #[proptest]
    fn has_bisector_angle_if_magnitudes_are_equal(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] t: f64,
        #[strategy(not_nan())] u: f64,
    ) {
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

    #[proptest]
    fn has_bisector_angle_if_magnitudes_are_opposite(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] t: f64,
        #[strategy(not_nan())] u: f64,
    ) {
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

    #[proptest]
    fn is_real_if_phasors_are_conjugate(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        let q = p.conj();

        let r = Phasor {
            mag: p.real() + q.real(),
            tan: 0f64,
        };

        prop_assume!(!p.is_infinite() || !p.is_imaginary());

        assert_ulps_eq!(p + q, r, max_ulps = 40);
        assert_ulps_eq!(q + p, r, max_ulps = 40);
    }

    #[proptest]
    fn has_double_magnitude_if_phasors_are_equal(
        #[strategy(not_nan())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        let r = Phasor {
            mag: 2f64 * mag,
            tan,
        };

        assert_ulps_eq!(p + p, r);
    }

    #[proptest]
    fn is_zero_if_phasors_are_finite_and_opposite(
        #[strategy(finite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        let q = -p;
        let r = Phasor {
            mag: 0f64,
            tan: -tan.recip(),
        };

        assert_ulps_eq!(p + q, r);
        assert_ulps_eq!(q + p, r);
    }

    #[proptest]
    fn is_nan_if_phasors_are_opposite_and_infinite(
        #[strategy(infinite())] mag: f64,
        #[strategy(not_nan())] tan: f64,
    ) {
        let p = Phasor { mag, tan };
        let q = -p;

        assert!((p + q).is_nan());
        assert!((q + p).is_nan());
    }

    #[proptest]
    fn is_nan_if_magnitude_is_nan(
        #[strategy(any())] a: f64,
        #[strategy(any())] b: f64,
        #[strategy(nan())] c: f64,
        #[strategy(any())] d: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        assert!((p + q).is_nan());
        assert!((q + p).is_nan());
    }

    #[proptest]
    fn is_nan_if_tangent_is_nan(
        #[strategy(any())] a: f64,
        #[strategy(any())] b: f64,
        #[strategy(any())] c: f64,
        #[strategy(nan())] d: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        assert!((p + q).is_nan());
        assert!((q + p).is_nan());
    }
}
