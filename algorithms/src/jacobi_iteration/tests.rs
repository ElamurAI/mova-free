//! Independent manufactured solutions and exact one-sweep recurrence witnesses (Varga, 1962).
use super::*;
#[test]
fn one_sweep_dependency_order() {
    let r = solve(
        &[vec![4.0, 1.0], vec![1.0, 3.0]],
        &[1.0, 2.0],
        &[0.0, 0.0],
        1e-30,
        1,
    )
    .unwrap();
    assert_eq!(r.x, vec![0.25, 2.0 / 3.0]);
    assert_eq!(r.iterations, 1);
    assert_eq!(r.status, Status::BudgetExhausted);
}
#[test]
fn exhaustive_diagonal_dominance() {
    for p in -4..=4 {
        for q in -4..=4 {
            for s in -2..=2 {
                let a = vec![vec![5.0, s as f64], vec![1.0, 4.0]];
                let b = vec![(5 * p + s * q) as f64, (p + 4 * q) as f64];
                let r = solve(&a, &b, &[0.0, 0.0], 1e-10, 200).unwrap();
                assert_eq!(r.status, Status::Converged);
                assert!((r.x[0] - p as f64).abs() < 1e-10);
                assert!((r.x[1] - q as f64).abs() < 1e-10);
                let fresh = ((5.0 * r.x[0] + s as f64 * r.x[1] - b[0]).powi(2)
                    + (r.x[0] + 4.0 * r.x[1] - b[1]).powi(2))
                .sqrt();
                assert!((r.residual_norm - fresh).abs() < 1e-20);
            }
        }
    }
}
#[test]
fn budget_and_initial_convergence() {
    let a = [vec![2.0]];
    let r = solve(&a, &[2.0], &[0.0], 1e-10, 0).unwrap();
    assert_eq!(r.status, Status::BudgetExhausted);
    assert_eq!(r.residual_norm, 2.0);
    let r = solve(&a, &[2.0], &[1.0], 1e-10, 0).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.iterations, 0);
}
#[test]
fn stagnation_is_not_convergence() {
    let mut witnessed = false;
    for d in 2..=200 {
        let r = solve(&[vec![d as f64]], &[1.0], &[0.0], f64::MIN_POSITIVE, 10).unwrap();
        if r.status == Status::Stagnated {
            assert!(r.residual_norm > f64::MIN_POSITIVE);
            assert_eq!(r.iterations, 2);
            witnessed = true;
        }
    }
    assert!(witnessed);
}
#[test]
fn invalid_and_nonfinite_arithmetic() {
    assert_eq!(solve(&[], &[], &[], 1.0, 0), Err(Error::InvalidInput));
    assert_eq!(
        solve(&[vec![0.0]], &[0.0], &[0.0], 1.0, 0),
        Err(Error::ZeroDiagonal)
    );
    assert_eq!(
        solve(&[vec![0.0]], &[f64::NAN], &[0.0], 1.0, 0),
        Err(Error::InvalidInput)
    );
    for tol in [0.0, -1.0, f64::NAN] {
        assert_eq!(
            solve(&[vec![1.0]], &[1.0], &[0.0], tol, 0),
            Err(Error::InvalidInput)
        );
    }
    assert_eq!(
        solve(&[vec![1.0]], &[1.0], &[0.0], 1.0, 10001),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        solve(&[vec![f64::from_bits(1)]], &[1e100], &[0.0], 1.0, 1),
        Err(Error::NonFiniteArithmetic)
    );
}
#[test]
fn bounded_divergence_and_maximum_dimension() {
    let r = solve(
        &[vec![1.0, 2.0], vec![2.0, 1.0]],
        &[1.0, 1.0],
        &[0.0, 0.0],
        1e-10,
        5,
    )
    .unwrap();
    assert_eq!(r.status, Status::BudgetExhausted);
    assert!(r.residual_norm > 1.0);
    let mut a = vec![vec![0.0; 64]; 64];
    for (i, row) in a.iter_mut().enumerate() {
        row[i] = 2.0;
    }
    let r = solve(&a, &[4.0; 64], &[0.0; 64], 1e-10, 1).unwrap();
    assert_eq!(r.x, vec![2.0; 64]);
    assert_eq!(r.status, Status::Converged);
}
