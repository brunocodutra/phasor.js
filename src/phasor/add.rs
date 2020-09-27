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
        let k = (c.ln_1p() / 2f64).exp(); // (c + 1f64).sqrt()

        Phasor {
            mag: if m.abs() > n.abs() {
                m.copysign(w + y) * k / i.abs()
            } else {
                n.copysign(w + y) * k / j.abs()
            },

            tan: if m.abs() == n.abs() && ((x > w) ^ (z > y)) {
                (y - w) / (x - z)
            } else {
                (x + z) / (w + y)
            },
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::{assert::ulps_or_relative_eq, assert_close_to};
    use alloc::format;
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

        #[test]
        fn adding_phasors_with_the_same_magnitude_has_bisector_angle(m in not_nan(), t in not_nan(), u in not_nan()) {
            let p = Phasor { mag: m, tan: t };
            let q = Phasor { mag: m, tan: u };

            prop_assume!(!ulps_or_relative_eq(&p, &(-q), 0f64));

            let r = Phasor::polar(
                m * (1f64 + cossubatan(t, u)).sqrt() * SQRT_2,
                (t.atan() + u.atan()) / 2f64,
            );

            assert_close_to!(p + q, r);
            assert_close_to!(q + p, r);
        }

        #[test]
        fn adding_equal_phasors_has_double_magnitude(a in not_nan(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let r = Phasor { mag: 2f64 * a, tan: b };

            assert_close_to!(p + p, r);
        }

        #[test]
        fn adding_opposite_finite_phasors_has_zero_magnitude_and_orthogonal_angle(a in finite(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = -p;
            let r = Phasor { mag: 0f64, tan: -b.recip() };

            assert_close_to!(p + q, r);
            assert_close_to!(q + p, r);
        }

        #[test]
        fn adding_finite_phasor_to_its_conjugate_is_purely_real(a in finite(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.conj();

            assert!((p + q).is_real());
            assert!((q + p).is_real());
        }

        #[test]
        fn adding_opposite_infinite_phasors_is_nan(a in infinite(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = -p;

            assert!((p + q).is_nan());
            assert!((q + p).is_nan());
        }
    }
}
