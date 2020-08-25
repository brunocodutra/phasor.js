#[macro_export]
macro_rules! assert_close_to {
    ($lhs:expr, $rhs:expr) => {
        assert_close_to!($lhs, $rhs, tol = 1E-15)
    };

    ($lhs:expr, $rhs:expr, tol = $tol:expr) => {
        let lhs = $lhs;
        let rhs = $rhs;

        assert!(
            ::approx::RelativeEq::relative_eq(&lhs, &rhs, $tol, $tol)
                || ::approx::UlpsEq::ulps_eq(&lhs, &rhs, $tol, ($tol / f64::EPSILON) as u32),
            r#"expected `lhs` to be approximately equal to `rhs`:
    lhs: `{:e} = {}`,
    rhs: `{:e} = {}`
    tol: `{:e}`
"#,
            lhs,
            stringify!($lhs),
            rhs,
            stringify!($rhs),
            $tol
        )
    };
}
