use super::*;
#[test]
fn analytic_oracle() {
    for degree in 0..=4 {
        let r = integrate(|x| x.powi(degree), 0.0, 1.0, 1e-8, 100_000).unwrap();
        assert_eq!(r.status, Status::EstimatedTolerance);
        assert!((r.value - 1.0 / (degree + 1) as f64).abs() < 2e-8);
    }
    let r = integrate(f64::sin, 0.0, std::f64::consts::PI, 1e-8, 100_000).unwrap();
    assert!((r.value - 2.0).abs() < 2e-8);
}
#[test]
fn budget_and_order() {
    let mut trace = vec![];
    let r = integrate(
        |x| {
            trace.push(x);
            x * x
        },
        0.0,
        1.0,
        1e-20,
        5,
    )
    .unwrap();
    assert_eq!(trace, vec![0.0, 1.0, 0.5, 0.25, 0.75]);
    assert_eq!(r.evaluations, 5);
    assert_eq!(r.status, Status::Budget);
    assert_eq!(r.value, 0.34375);
    let r = integrate(|_| 1.0, 0.0, 1.0, 1e-10, 2).unwrap();
    assert_eq!(r.estimated_error, None);
    assert_eq!(r.evaluations, 2);
}
#[test]
fn failure_and_alias() {
    let mut calls = 0;
    assert_eq!(
        integrate(
            |_| {
                calls += 1;
                0.0
            },
            1.0,
            0.0,
            1.0,
            10
        ),
        Err(Error::InvalidInput)
    );
    assert_eq!(calls, 0);
    assert_eq!(
        integrate(
            |_| {
                calls += 1;
                f64::NAN
            },
            0.0,
            1.0,
            1.0,
            10
        ),
        Err(Error::NonFiniteCallback)
    );
    assert_eq!(calls, 1);
    assert_eq!(
        integrate(|_| f64::MAX, 0.0, 2.0, 1.0, 10),
        Err(Error::NonFiniteArithmetic)
    );
    let b = f64::from_bits(1.0_f64.to_bits() + 1);
    assert_eq!(
        integrate(|_| 0.0, 1.0, b, 1e-10, 3).unwrap().status,
        Status::Stagnation
    );
    let z = integrate(|_| 0.0, 0.0, 1.0, 1e-10, 10).unwrap();
    let bump = integrate(
        |x| if x > 0.1 && x < 0.2 { 1.0 } else { 0.0 },
        0.0,
        1.0,
        1e-10,
        10,
    )
    .unwrap();
    assert_eq!(z, bump);
}
