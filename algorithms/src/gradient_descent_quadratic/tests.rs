use super::*;
#[test]
fn diagonal_division_oracle() {
    for k in 1..20 {
        let d = [1.0, f64::from(k), 2.0];
        let b = [3.0, -4.0, 5.0];
        let r = minimize_diagonal(&d, &b, &[0.0; 3], 1e-10, 10000).unwrap();
        assert_eq!(r.status, Status::Converged);
        for i in 0..3 {
            assert!(
                (r.x[i] - b[i] / d[i]).abs() < 1e-9,
                "diagonal coordinate oracle"
            );
        }
        let norm = (0..3)
            .map(|i| (d[i] * r.x[i] - b[i]).powi(2))
            .sum::<f64>()
            .sqrt();
        assert!((norm - r.residual_norm).abs() < 1e-20);
    }
}
#[test]
fn budget_zero_and_stagnation() {
    assert_eq!(
        minimize_diagonal(&[1.0], &[0.0], &[0.0], 1e-10, 0)
            .unwrap()
            .status,
        Status::Converged
    );
    assert_eq!(
        minimize_diagonal(&[1.0], &[1.0], &[0.0], 1e-10, 0)
            .unwrap()
            .status,
        Status::BudgetExhausted
    );
    let r = minimize_diagonal(&[1e-100, 1e100], &[0.0, 0.0], &[1.0, 0.0], 1e-110, 5).unwrap();
    assert_eq!(r.status, Status::Stagnated);
}
#[test]
fn domain_rejection() {
    for d in [0.0, -1.0, f64::NAN, f64::INFINITY, 1e-101] {
        assert_eq!(
            minimize_diagonal(&[d], &[1.0], &[0.0], 0.1, 1),
            Err(Error::InvalidInput)
        );
    }
    assert_eq!(
        minimize_diagonal(&[], &[], &[], 0.1, 1),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.0], &[], &[0.0], 0.1, 1),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.0], &[1.0], &[f64::NAN], 0.1, 1),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.0], &[1.0], &[0.0], 0.0, 1),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.0], &[1.0], &[0.0], 0.1, 100001),
        Err(Error::InvalidInput)
    );
}
