use super::*;
#[test]
fn analytic_oracle() {
    for degree in 0..=4 {
        let r = integrate(|x| x.powi(degree), 0.0, 1.0, 1e-10, 10000).unwrap();
        assert_eq!(r.status, Status::EstimatedTolerance);
        assert!((r.value - 1.0 / (degree + 1) as f64).abs() < 1e-11);
    }
    let r = integrate(f64::sin, 0.0, std::f64::consts::PI, 1e-10, 10000).unwrap();
    assert!((r.value - 2.0).abs() < 1e-10);
}
#[test]
fn budget_and_order() {
    let mut trace = vec![];
    let r = integrate(
        |x| {
            trace.push(x);
            1.0
        },
        0.0,
        1.0,
        1e-20,
        20,
    )
    .unwrap();
    assert_eq!(trace.len(), 20);
    assert_eq!(r.evaluations, 20);
    assert_eq!(r.status, Status::Budget);
    assert!(trace[..7].windows(2).all(|w| w[0] < w[1]));
    assert!(trace[7..].windows(2).all(|w| w[0] < w[1]));
    assert!(trace.iter().all(|x| *x > 0.0 && *x < 1.0));
    assert_eq!(trace[3], 0.5);
    assert_eq!(trace[13], 0.5);
    assert_eq!(trace[0], trace[7]);
    let r = integrate(|_| 1.0, 0.0, 1.0, 1e-10, 7).unwrap();
    assert_eq!(r.evaluations, 7);
    assert_eq!(r.estimated_error, None);
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
            0.0,
            1.0,
            1.0,
            6
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
            7
        ),
        Err(Error::NonFiniteCallback)
    );
    assert_eq!(calls, 1);
    let b = f64::from_bits(1.0_f64.to_bits() + 1);
    assert_eq!(
        integrate(|_| 0.0, 1.0, b, 1e-10, 7).unwrap().status,
        Status::Stagnation
    );
    let z = integrate(|_| 0.0, 0.0, 1.0, 1e-10, 20).unwrap();
    let bump = integrate(
        |x| if x > 0.3 && x < 0.31 { 1.0 } else { 0.0 },
        0.0,
        1.0,
        1e-10,
        20,
    )
    .unwrap();
    assert_eq!(z, bump);
}
