use proptest::num::f64::*;

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

pub(crate) fn not_nan() -> Any {
    infinite() | normal() | subnormal() | zero()
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
