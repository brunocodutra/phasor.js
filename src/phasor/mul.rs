use super::Phasor;
use crate::trig::tanaddatan;
use core::ops::Mul;

impl Mul for Phasor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (s, c) = tanaddatan(self.tan, rhs.tan);

        Phasor {
            mag: self.mag * rhs.mag * c.signum(),
            tan: s / c,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use crate::assert_close_to;
    use alloc::format;
    use core::num::FpCategory::{Infinite, Zero};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn has_norm_equal_to_product_of_norms(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            prop_assume!(!matches!((a.classify(), c.classify()), (Zero, Infinite) | (Infinite, Zero)));

            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_close_to!((p * q).norm(), p.norm() * q.norm());
            assert_close_to!((q * p).norm(), p.norm() * q.norm());
        }

        #[test]
        fn has_angle_equal_to_sum_of_angles(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            prop_assume!(!matches!((a.classify(), c.classify()), (Zero, Infinite) | (Infinite, Zero)));

            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            let v = p.angle() + q.angle();

            assert_close_to!((p * q).angle().cos(), v.cos(), tol = 1E-12);
            assert_close_to!((p * q).angle().sin(), v.sin(), tol = 1E-12);

            assert_close_to!((q * p).angle().cos(), v.cos(), tol = 1E-12);
            assert_close_to!((q * p).angle().sin(), v.sin(), tol = 1E-12);
        }

        #[test]
        fn is_real_if_phasors_are_conjugate(a in not_nan(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.conj();
            let r = Phasor { mag: a * a, tan: 0f64 };

            assert_close_to!(p * q, r);
            assert_close_to!(q * p, r);
        }

        #[test]
        fn equals_one_if_phasors_are_inverse(a in normal(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.recip();
            let r = Phasor { mag: 1f64, tan: 0f64 };

            assert_close_to!(p * q, r);
            assert_close_to!(q * p, r);
        }

        #[test]
        fn is_nan_if_phasors_are_zero_and_infinite(a in infinite(), b in not_nan(), c in zero(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p * q).is_nan());
            assert!((q * p).is_nan());
        }

        #[test]
        fn is_nan_if_magnitude_is_nan(a in any(), b in any(), c in nan(), d in any()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p * q).is_nan());
            assert!((q * p).is_nan());
        }

        #[test]
        fn is_nan_if_tangent_is_nan(a in any(), b in any(), c in any(), d in nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p * q).is_nan());
            assert!((q * p).is_nan());
        }
    }
}
