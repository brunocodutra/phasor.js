use super::Phasor;
use crate::trig::{cosatan, cosatan2, cossubatan, sinatan, sinatan2};
use core::{f64::consts::SQRT_2, ops::Add};

impl Add for Phasor {
    type Output = Self;

    #[allow(clippy::float_cmp, clippy::many_single_char_names)]
    fn add(self, rhs: Self) -> Self::Output {
        let m = self.mag;
        let n = rhs.mag;

        let i = sinatan2(m, n);
        let j = cosatan2(m, n);

        let (u, v) = if m.abs() == n.abs() {
            (m.signum(), n.signum())
        } else {
            (i * SQRT_2, j * SQRT_2)
        };

        let w = u * cosatan(self.tan);
        let x = u * sinatan(self.tan);
        let y = v * cosatan(rhs.tan);
        let z = v * sinatan(rhs.tan);

        let c = u * v * cossubatan(self.tan, rhs.tan);
        let k = (c.min(1f64).max(-1f64).ln_1p() / 2f64).exp(); // (c + 1f64).sqrt()

        Phasor {
            mag: if m.abs() > n.abs() {
                m.copysign(w + y) * k / i.abs()
            } else {
                n.copysign(w + y) * k / j.abs()
            },

            tan: if x != -z || w != -y {
                (x + z) / (w + y)
            } else if 1f64.copysign(w) == 1f64.copysign(y) {
                w
            } else if 1f64.copysign(x) == 1f64.copysign(z) {
                f64::INFINITY.copysign(x)
            } else {
                -w / x
            },
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::{assert::ulps_or_relative_eq, assert_close_to, trig::tanaddatan};
    use alloc::format;
    use core::f64::consts::FRAC_PI_2;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn adding_finite_nonzero_phasors_equals_sum_of_real_and_imaginary_parts(a in regular(), b in not_nan(), c in regular(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };
            let r = Phasor::rect(p.real() + q.real(), p.imag() + q.imag());

            assert_close_to!(p + q, r);
            assert_close_to!(q + p, r);
        }

        #[test]
        fn adding_zero_has_no_effect(a in nonzero(), b in not_nan(), c in zero(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_close_to!(p + q, p);
            assert_close_to!(q + p, p);
        }

        #[test]
        fn adding_infinity_has_no_effect(a in finite(), b in not_nan(), c in infinite(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_close_to!(p + q, q);
            assert_close_to!(q + p, q);
        }

        #[test]
        fn adding_phasors_with_equal_magnitudes_has_bisector_angle(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(!mag.is_infinite() || cossubatan(t, u) != -1f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 + cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64,
            );

            assert_close_to!(p + q, r);
            assert_close_to!(q + p, r);
        }

        #[test]
        fn adding_phasors_with_opposite_magnitudes_has_bisector_angle(mag in not_nan(), t in not_nan(), u in not_nan()) {
            prop_assume!(!mag.is_infinite() || cossubatan(t, u) != 1f64);

            let p = Phasor { mag, tan: t };
            let q = Phasor { mag: -mag, tan: u };

            let (s, c) = tanaddatan(t, u);

            let r = Phasor::polar(
                mag * (1f64 - cossubatan(t, u)).sqrt() * SQRT_2,
                s.atan2(c) / 2f64 + if t < u { -FRAC_PI_2 } else { FRAC_PI_2 },
            );

            if ulps_or_relative_eq(&p, &(-q), 0f64) {
                assert_close_to!(p + q, q + p);
            } else {
                assert_close_to!(p + q, r);
                assert_close_to!(q + p, r);
            }
        }

        #[test]
        fn adding_conjugate_finite_phasors_is_purely_real(mag in not_nan(), tan in not_nan()) {
            prop_assume!(!mag.is_infinite() || cossubatan(tan, -tan) != -1f64);

            let p = Phasor { mag, tan };
            let q = p.conj();
            let r = Phasor { mag: mag * (1f64 + cossubatan(tan, -tan)).sqrt() * SQRT_2, tan: 0f64 };

            assert_close_to!(p + q, r);
            assert_close_to!(q + p, r);
        }

        #[test]
        fn adding_equal_phasors_has_double_magnitude(mag in not_nan(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let r = Phasor { mag: 2f64 * mag, tan };

            assert_close_to!(p + p, r);
        }

        #[test]
        fn adding_opposite_finite_phasors_has_zero_magnitude_and_orthogonal_angle(mag in finite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = -p;
            let r = Phasor { mag: 0f64, tan: -tan.recip() };

            assert_close_to!(p + q, r);
            assert_close_to!(q + p, r);
        }

        #[test]
        fn adding_opposite_infinite_phasors_is_nan(mag in infinite(), tan in not_nan()) {
            let p = Phasor { mag, tan };
            let q = -p;

            assert!((p + q).is_nan());
            assert!((q + p).is_nan());
        }

        #[test]
        fn adding_phasor_that_has_undefined_magnitude_is_nan(a in any(), b in any(), c in nan(), d in any()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p + q).is_nan());
            assert!((q + p).is_nan());
        }

        #[test]
        fn adding_phasor_that_has_undefined_tangent_is_nan(a in any(), b in any(), c in any(), d in nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p + q).is_nan());
            assert!((q + p).is_nan());
        }
    }
}
