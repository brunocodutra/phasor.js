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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use alloc::format;
    use approx::assert_ulps_eq;
    use core::num::FpCategory::{Infinite, Zero};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn is_commutative(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            prop_assume!(!matches!((a.classify(), c.classify()), (Zero, Infinite) | (Infinite, Zero)));

            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_ulps_eq!(p * q, q * p);
        }

        #[test]
        fn has_norm_equal_to_product_of_norms(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            prop_assume!(!matches!((a.classify(), c.classify()), (Zero, Infinite) | (Infinite, Zero)));

            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            assert_ulps_eq!((p * q).norm(), p.norm() * q.norm());
            assert_ulps_eq!((q * p).norm(), p.norm() * q.norm());
        }

        #[test]
        fn has_angle_equal_to_sum_of_angles(a in not_nan(), b in not_nan(), c in not_nan(), d in not_nan()) {
            prop_assume!(!matches!((a.classify(), c.classify()), (Zero, Infinite) | (Infinite, Zero)));

            let p = Phasor { mag: a, tan: b };
            let q = Phasor { mag: c, tan: d };

            let v = p.angle() + q.angle();

            assert_ulps_eq!((p * q).angle().cos(), v.cos(), epsilon = 8f64 * f64::EPSILON);
            assert_ulps_eq!((p * q).angle().sin(), v.sin(), epsilon = 8f64 * f64::EPSILON);

            assert_ulps_eq!((q * p).angle().cos(), v.cos(), epsilon = 8f64 * f64::EPSILON);
            assert_ulps_eq!((q * p).angle().sin(), v.sin(), epsilon = 8f64 * f64::EPSILON);
        }

        #[test]
        fn is_real_if_phasors_are_conjugate(a in not_nan(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.conj();
            let r = Phasor { mag: a * a, tan: 0f64 };

            assert_ulps_eq!(p * q, r);
            assert_ulps_eq!(q * p, r);
        }

        #[test]
        fn equals_one_if_phasors_are_inverse(a in normal(), b in not_nan()) {
            let p = Phasor { mag: a, tan: b };
            let q = p.recip();
            let r = Phasor { mag: 1f64, tan: 0f64 };

            assert_ulps_eq!(p * q, r);
            assert_ulps_eq!(q * p, r);
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
