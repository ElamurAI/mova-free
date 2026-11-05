use super::*;
#[test]
fn quadratic_oracle() {
    for center in -8..=8 {
        let c = center as f64 / 3.0;
        let r = minimize(|x| (x - c).powi(2), -5.0, 5.0, 1e-9, 300).unwrap();
        assert_eq!(r.status, Status::Converged);
        assert!((r.estimate - c).abs() < 1e-7);
        assert!(r.left <= c + 1e-7 && r.right >= c - 1e-7);
        assert!(r.evaluations <= 301);
    }
}
#[test]
fn nonsmooth_and_endpoint() {
    let r = minimize(|x| (x - 0.137).abs(), -1.0, 1.0, 1e-10, 300).unwrap();
    assert!((r.estimate - 0.137).abs() < 1e-8);
    let r = minimize(|x| x, 0.0, 1.0, 1e-10, 300).unwrap();
    assert!(r.estimate < 1e-8);
}
#[test]
fn budget_validation_and_counts() {
    let mut calls = 0;
    let r = minimize(
        |x| {
            calls += 1;
            x * x
        },
        -1.0,
        1.0,
        1e-10,
        0,
    )
    .unwrap();
    assert_eq!(r.status, Status::BudgetExhausted);
    assert_eq!(calls, r.evaluations);
    assert_eq!(calls, 1);
    assert_eq!(
        minimize(|_| panic!(), 1.0, 0.0, 1e-4, 3),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(|_| f64::NAN, 0.0, 1.0, 1e-4, 3),
        Err(Error::NonFiniteEvaluation)
    );
}
#[test]
fn adjacent_stagnation_and_determinism() {
    let a = 1.0_f64;
    let b = f64::from_bits(a.to_bits() + 1);
    let r = minimize(|x| x, a, b, f64::MIN_POSITIVE, 30).unwrap();
    assert_eq!(r.status, Status::Stagnated);
    let run = || minimize(|x| x.exp() - 3.0 * x, -2.0, 3.0, 1e-9, 300).unwrap();
    assert_eq!(run(), run());
}
