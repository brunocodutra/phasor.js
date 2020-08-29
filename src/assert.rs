#[cfg(any(test, target_arch = "wasm32"))]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn ulps_or_relative_eq<T>(x: &T, y: &T, e: f64) -> bool
where
    T: AbsDiffEq<Epsilon = f64> + UlpsEq + RelativeEq,
{
    x.ulps_eq(&y, e, (e / T::default_epsilon()) as u32) || x.relative_eq(&y, e, e)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[macro_export]
macro_rules! assert_close_to {
    ($lhs:expr, $rhs:expr) => {
        assert_close_to!($lhs, $rhs, tol = 1E-15)
    };

    ($lhs:expr, $rhs:expr, tol = $tol:expr) => {
        assert!(
            $crate::assert::ulps_or_relative_eq(&$lhs, &$rhs, $tol),
            r#"expected `lhs` to be approximately equal to `rhs`:
    lhs: `{:e} = {}`,
    rhs: `{:e} = {}`
    tol: `{:e}`
"#,
            $lhs,
            stringify!($lhs),
            $rhs,
            stringify!($rhs),
            $tol
        )
    };
}
