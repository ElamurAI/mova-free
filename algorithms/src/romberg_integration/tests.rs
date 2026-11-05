use super::*;
#[test]
fn polynomial_oracle() {
    for degree in 0..8 {
        let r = integrate(|x| x.powi(degree), 0.0, 1.0, 1e-12, 15, 10000).unwrap();
        assert_eq!(r.status, Status::Converged);
        assert!(
            (r.value - 1.0 / (degree + 1) as f64).abs() < 1e-12,
            "Romberg polynomial oracle"
        );
        assert_eq!(r.evaluations, (1usize << (r.levels - 1)) + 1);
    }
}
#[test]
fn budget_levels_stagnation() {
    let r = integrate(|x| x * x, 0.0, 1.0, 1e-15, 10, 2).unwrap();
    assert_eq!(r.status, Status::WorkBudget);
    assert_eq!(r.evaluations, 2);
    let r = integrate(|x| x * x, 0.0, 1.0, 1e-15, 1, 20).unwrap();
    assert_eq!(r.status, Status::LevelLimit);
    let a = 1.0_f64;
    let b = f64::from_bits(a.to_bits() + 1);
    let r = integrate(|x| x, a, b, 1e-300, 10, 100).unwrap();
    assert_eq!(r.status, Status::Stagnated);
    assert_eq!(r.evaluations, 2);
}
#[test]
fn invalid_zero_and_aliasing() {
    assert_eq!(
        integrate(|_| 0.0, 0.0, 1.0, 0.0, 10, 20),
        Err(Error::Configuration)
    );
    assert_eq!(
        integrate(|_| 0.0, f64::NAN, 1.0, 1e-6, 10, 20),
        Err(Error::NonFinite)
    );
    assert_eq!(
        integrate(|_| 0.0, 1.0, 0.0, 1e-6, 10, 20),
        Err(Error::Domain)
    );
    assert_eq!(
        integrate(|_| f64::NAN, 0.0, 1.0, 1e-6, 10, 20),
        Err(Error::Callback)
    );
    let r = integrate(|_| panic!("no callback"), 0.0, 0.0, 1e-6, 10, 20).unwrap();
    assert_eq!(r.evaluations, 0);
    // This nonnegative polynomial has analytic integral 1/840, but all first
    // three sampled values vanish: convergence is deliberately only heuristic.
    let r = integrate(
        |x| (x * (x - 0.5) * (x - 1.0)).powi(2),
        0.0,
        1.0,
        1e-6,
        10,
        20,
    )
    .unwrap();
    assert_eq!(r.value, 0.0);
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.estimated_error, Some(0.0));
    assert!((r.value - 1.0 / 840.0).abs() > 1e-6);
}
