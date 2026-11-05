use super::*;
#[test]
fn quadratic_minimum_oracle() {
    for k in -20..=20 {
        let center = f64::from(k) / 4.0;
        let r = minimize(|x| (x - center) * (x - center), -10.0, 10.0, 1e-9, 100).unwrap();
        assert_eq!(r.status, Status::Converged);
        assert!(
            r.left <= center && center <= r.right,
            "minimum bracket oracle"
        );
        assert!(
            (r.estimate - center).abs() < 1e-9,
            "minimum coordinate oracle"
        );
    }
}
#[test]
fn boundary_and_budget() {
    let r = minimize(|x| x, -2.0, 4.0, 1e-8, 100).unwrap();
    assert!((r.estimate + 2.0).abs() < 1e-8);
    assert_eq!(r.status, Status::Converged);
    assert_eq!(
        minimize(|x| x * x, -1.0, 2.0, 1e-10, 0).unwrap().status,
        Status::BudgetExhausted
    );
    let mut count = 0;
    minimize(
        |x| {
            count += 1;
            x * x
        },
        -1.0,
        2.0,
        1e-100,
        3,
    )
    .unwrap();
    assert_eq!(count, 5);
}
#[test]
fn ties_stagnation_and_invalid() {
    let a = 1.0_f64;
    let b = f64::from_bits(a.to_bits() + 1);
    assert_eq!(
        minimize(|x| x, a, b, f64::MIN_POSITIVE, 10).unwrap().status,
        Status::Stagnated
    );
    let a = minimize(|_| 1.0, 0.0, 1.0, 0.01, 10).unwrap();
    let b = minimize(|_| 1.0, 0.0, 1.0, 0.01, 10).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.left, 0.0);
    assert_eq!(
        minimize(|_| f64::NAN, 0.0, 1.0, 0.1, 10),
        Err(Error::NonFiniteEvaluation)
    );
    for tol in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(minimize(|x| x, 0.0, 1.0, tol, 10), Err(Error::InvalidInput));
    }
    assert_eq!(minimize(|x| x, 1.0, 0.0, 0.1, 10), Err(Error::InvalidInput));
    assert_eq!(
        minimize(|x| x, 0.0, 1.0, 0.1, 100001),
        Err(Error::InvalidInput)
    );
}
