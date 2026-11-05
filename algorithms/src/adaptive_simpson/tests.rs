use super::*;
#[test]
fn analytic_polynomials() {
    // Loose tolerance isolates Richardson correction instead of refining it away.
    let quartic = integrate(|x| x.powi(4), 0.0, 1.0, 1.0, 5, 0).unwrap();
    assert_eq!(quartic.evaluations, 5);
    assert!((quartic.value - 0.2).abs() < 1e-15, "quartic_correction");
    for degree in 0..=7 {
        let r = integrate(|x| x.powi(degree), 0.0, 1.0, 1e-10, 10000, 30).unwrap();
        assert_eq!(r.status, Status::EstimatedTolerance);
        assert!(
            (r.value - 1.0 / (degree + 1) as f64).abs() < 1e-9,
            "analytic_polynomials"
        );
        assert!(r.evaluations <= 10000);
        assert!(r.estimated_error <= 1.01e-10);
    }
    let r = integrate(|x| x.sin(), 0.0, std::f64::consts::PI, 1e-10, 10000, 30).unwrap();
    assert!((r.value - 2.0).abs() < 1e-9);
}
#[test]
fn budgets_and_stagnation() {
    let mut calls = 0;
    let r = integrate(
        |x| {
            calls += 1;
            x * x * x * x
        },
        0.0,
        1.0,
        1e-30,
        7,
        30,
    )
    .unwrap();
    assert_eq!(r.status, Status::EvaluationBudget);
    assert_eq!(r.evaluations, calls);
    assert!(calls <= 7);
    assert_eq!(r.estimated_error, f64::MAX);
    assert_eq!(
        integrate(|x| x.powi(4), 0.0, 1.0, 1e-30, 100, 0)
            .unwrap()
            .status,
        Status::DepthBudget
    );
    assert_eq!(
        integrate(
            |_| 1.0,
            1.0,
            f64::from_bits(1.0f64.to_bits() + 1),
            1e-10,
            100,
            10
        )
        .unwrap()
        .status,
        Status::Stagnation
    );
    assert_eq!(
        integrate(|_| panic!("zero interval"), 1.0, 1.0, 1e-10, 3, 0)
            .unwrap()
            .value,
        0.0
    );
}
#[test]
fn validation_and_unsampled_fiber() {
    assert_eq!(
        integrate(|x| x, 1.0, 0.0, 1e-5, 100, 5),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        integrate(|x| x, 0.0, 1.0, 0.0, 100, 5),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        integrate(|x| x, 0.0, 1.0, 1e-5, 2, 5),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        integrate(|x| x, 0.0, 1.0, 1e-5, 100, 65),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        integrate(|_| f64::NAN, 0.0, 1.0, 1e-5, 100, 5),
        Err(Error::NonFiniteArithmetic)
    );
    assert_eq!(
        integrate(|_| f64::MAX, 0.0, 4.0, 1e-5, 100, 5),
        Err(Error::NonFiniteArithmetic)
    );
    // Same five samples, different integral: no general accuracy certificate from finite checks.
    let zero = integrate(|_| 0.0, 0.0, 1.0, 1e-12, 100, 10).unwrap();
    let hidden = integrate(
        |x| if x > 0.1 && x < 0.2 { 1.0 } else { 0.0 },
        0.0,
        1.0,
        1e-12,
        100,
        10,
    )
    .unwrap();
    assert_eq!(zero, hidden);
    assert_eq!(hidden.value, 0.0);
}
