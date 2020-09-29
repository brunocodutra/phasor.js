use proptest::{num::f64::*, prelude::*};

pub(crate) fn any() -> Any {
    ANY
}

pub(crate) fn nan() -> Any {
    POSITIVE | NEGATIVE | QUIET_NAN | SIGNALING_NAN
}

pub(crate) fn infinite() -> Any {
    POSITIVE | NEGATIVE | INFINITE
}

pub(crate) fn normal() -> Any {
    POSITIVE | NEGATIVE | NORMAL
}

pub(crate) fn subnormal() -> Any {
    POSITIVE | NEGATIVE | SUBNORMAL
}

pub(crate) fn zero() -> Any {
    POSITIVE | NEGATIVE | ZERO
}

pub(crate) fn positive() -> Any {
    POSITIVE | NORMAL | SUBNORMAL
}

pub(crate) fn negative() -> Any {
    NEGATIVE | NORMAL | SUBNORMAL
}

pub(crate) fn not_nan() -> Any {
    normal() | subnormal() | zero() | infinite()
}

pub(crate) fn finite() -> Any {
    normal() | subnormal() | zero()
}

pub(crate) fn nonzero() -> Any {
    normal() | subnormal() | infinite()
}

pub(crate) fn regular() -> Any {
    normal() | subnormal()
}

pub(crate) fn modulo(period: f64) -> impl Strategy<Value = f64> {
    finite().prop_map(move |n| n % period).boxed()
}
