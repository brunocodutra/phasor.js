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
        fn multiplying_phasors_has_norm_equal_to_product_of_norms(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            prop_assume!(!matches!((a.classify(), c.classify()), (Zero, Infinite) | (Infinite, Zero)));

            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_close_to!((p * q).norm(), p.norm() * q.norm());
            assert_close_to!((q * p).norm(), p.norm() * q.norm());
        }

        #[test]
        fn multiplying_phasors_has_angle_equal_to_sum_of_angles(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
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
        fn multiplying_phasor_by_its_conjugate_equals_real_phasor_with_the_magnitude_squared(a in not_nan(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.conj();
            let r = Phasor { mag: a * a, tan: 0f64 };

            assert_close_to!(p * q, r);
            assert_close_to!(q * p, r);
        }

        #[test]
        fn multiplying_normal_phasor_by_its_inverse_equals_one(a in normal(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.recip();
            let r = Phasor { mag: 1f64, tan: 0f64 };

            assert_close_to!(p * q, r);
            assert_close_to!(q * p, r);
        }

        #[test]
        fn multiplying_zero_by_infinite_phasors_is_nan(a in infinite(), b in not_nan(), c in zero(), d in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p * q).is_nan());
            assert!((q * p).is_nan());
        }

        #[test]
        fn multiplying_by_phasor_that_has_undefined_magnitude_is_nan(a in any(), b in any(), c in nan(), d in any()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p * q).is_nan());
            assert!((q * p).is_nan());
        }

        #[test]
        fn multiplying_by_phasor_that_has_undefined_tangent_is_nan(a in any(), b in any(), c in any(), d in nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert!((p * q).is_nan());
            assert!((q * p).is_nan());
        }
    }
}
