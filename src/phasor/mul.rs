use super::Phasor;
use crate::trig::tanaddatan;
use std::ops::Mul;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Phasor {
    pub fn mul(&self, rhs: &Phasor) -> Phasor {
        std::ops::Mul::mul(*self, *rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{any, *};
    use approx::assert_ulps_eq;
    use proptest::prop_assume;
    use std::num::FpCategory::{Infinite, Zero};
    use test_strategy::proptest;

    #[proptest]
    fn is_commutative(
        #[strategy(not_nan())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(not_nan())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        prop_assume!(!matches!(
            (a.classify(), c.classify()),
            (Zero, Infinite) | (Infinite, Zero)
        ));

        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        assert_ulps_eq!(p * q, q * p);
    }

    #[proptest]
    fn has_norm_equal_to_product_of_norms(
        #[strategy(not_nan())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(not_nan())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        prop_assume!(!matches!(
            (a.classify(), c.classify()),
            (Zero, Infinite) | (Infinite, Zero)
        ));

        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        assert_ulps_eq!((p * q).norm(), p.norm() * q.norm());
        assert_ulps_eq!((q * p).norm(), p.norm() * q.norm());
    }

    #[proptest]
    fn has_angle_equal_to_sum_of_angles(
        #[strategy(not_nan())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(not_nan())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        prop_assume!(!matches!(
            (a.classify(), c.classify()),
            (Zero, Infinite) | (Infinite, Zero)
        ));

        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        let v = p.angle() + q.angle();

        assert_ulps_eq!(
            (p * q).angle().cos(),
            v.cos(),
            epsilon = 8f64 * f64::EPSILON
        );
        assert_ulps_eq!(
            (p * q).angle().sin(),
            v.sin(),
            epsilon = 8f64 * f64::EPSILON
        );

        assert_ulps_eq!(
            (q * p).angle().cos(),
            v.cos(),
            epsilon = 8f64 * f64::EPSILON
        );
        assert_ulps_eq!(
            (q * p).angle().sin(),
            v.sin(),
            epsilon = 8f64 * f64::EPSILON
        );
    }

    #[proptest]
    fn is_real_if_phasors_are_conjugate(
        #[strategy(not_nan())] a: f64,
        #[strategy(not_nan())] b: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = p.conj();
        let r = Phasor {
            mag: a * a,
            tan: 0f64,
        };

        assert_ulps_eq!(p * q, r);
        assert_ulps_eq!(q * p, r);
    }

    #[proptest]
    fn equals_one_if_phasors_are_inverse(
        #[strategy(normal())] a: f64,
        #[strategy(not_nan())] b: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = p.recip();
        let r = Phasor {
            mag: 1f64,
            tan: 0f64,
        };

        assert_ulps_eq!(p * q, r);
        assert_ulps_eq!(q * p, r);
    }

    #[proptest]
    fn is_nan_if_phasors_are_zero_and_infinite(
        #[strategy(infinite())] a: f64,
        #[strategy(not_nan())] b: f64,
        #[strategy(zero())] c: f64,
        #[strategy(not_nan())] d: f64,
    ) {
        let p = Phasor { mag: a, tan: b };
        let q = Phasor { mag: c, tan: d };

        assert!((p * q).is_nan());
        assert!((q * p).is_nan());
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

        assert!((p * q).is_nan());
        assert!((q * p).is_nan());
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

        assert!((p * q).is_nan());
        assert!((q * p).is_nan());
    }
}
