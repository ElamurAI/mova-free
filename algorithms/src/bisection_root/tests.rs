use super::*;
#[test]
fn analytic_roots_and_brackets() {
    for k in 1..40 {
        let a = f64::from(k);
        let r = solve(|x| x * x - a, 0.0, a + 1.0, 1e-10, 100).unwrap();
        let exact = a.sqrt();
        assert_eq!(r.status, Status::Converged);
        assert!(
            r.left <= exact + 1e-14 && exact <= r.right + 1e-14,
            "root bracket oracle"
        );
        assert!(
            (r.estimate - exact).abs() <= 1e-10,
            "root coordinate oracle"
        );
        assert!(r.right - r.left <= 1e-10);
    }
}
#[test]
fn endpoints_budget_and_stagnation() {
    let r = solve(|x| x, 0.0, 1.0, 1e-10, 0).unwrap();
    assert_eq!(r.estimate, 0.0);
    assert_eq!(r.status, Status::Converged);
    assert_eq!(
        solve(|x| x * x - 2.0, 0.0, 2.0, 1e-10, 0).unwrap().status,
        Status::BudgetExhausted
    );
    let a = 1.0_f64;
    let b = f64::from_bits(a.to_bits() + 1);
    assert_eq!(
        solve(
            |x| if x == a { -1.0 } else { 1.0 },
            a,
            b,
            f64::MIN_POSITIVE,
            8
        )
        .unwrap()
        .status,
        Status::Stagnated
    );
}
#[test]
fn invalid_and_evaluation_limits() {
    assert_eq!(
        solve(|x| x * x + 1.0, -1.0, 1.0, 0.1, 10),
        Err(Error::NotBracketed)
    );
    for tol in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(solve(|x| x, -1.0, 1.0, tol, 10), Err(Error::InvalidInput));
    }
    assert_eq!(
        solve(|_| f64::NAN, -1.0, 1.0, 0.1, 10),
        Err(Error::NonFiniteEvaluation)
    );
    assert_eq!(
        solve(
            |x| if x == 0.0 { f64::INFINITY } else { x },
            -1.0,
            1.0,
            0.1,
            10
        ),
        Err(Error::NonFiniteEvaluation)
    );
    let mut count = 0;
    let r = solve(
        |x| {
            count += 1;
            x * x - 2.0
        },
        0.0,
        2.0,
        1e-100,
        4,
    )
    .unwrap();
    assert_eq!(count, 6);
    assert_eq!(r.iterations, 4);
    assert_eq!(solve(|x| x, 1.0, 1.0, 0.1, 2), Err(Error::InvalidInput));
    assert_eq!(
        solve(|x| x, -1.0, 1.0, 0.1, 100001),
        Err(Error::InvalidInput)
    );
}
