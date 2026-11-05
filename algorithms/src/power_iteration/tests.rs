use super::*;
#[test]
fn diagonal_oracle() {
    let a = vec![vec![4.0, 0.0], vec![0.0, 1.0]];
    let r = solve(&a, &[1.0, 1.0], 1e-10, 100).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert!((r.eigenvalue - 4.0).abs() < 1e-10);
    assert!(r.residual <= 1e-10);
    assert!(r.eigenvector[0] > 0.0);
}
#[test]
fn missing_dominant_component_is_not_dominance_certificate() {
    let a = vec![vec![4.0, 0.0], vec![0.0, 1.0]];
    let r = solve(&a, &[0.0, -1.0], 0.0, 10).unwrap();
    assert_eq!(r.eigenvalue, 1.0);
    assert_eq!(r.eigenvector, vec![0.0, 1.0]);
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.iterations, 0);
}
#[test]
fn zero_budget_true_residual() {
    let a = vec![vec![2.0, 0.0], vec![0.0, 4.0]];
    let r = solve(&a, &[1.0, 1.0], 0.0, 0).unwrap();
    assert_eq!(r.status, Status::Budget);
    assert!((r.eigenvalue - 3.0).abs() < 1e-14);
    assert!((r.residual - 1.0).abs() < 1e-14);
    assert_eq!(r.iterations, 0);
}
#[test]
fn negative_eigenvalue_and_zero_image() {
    let r = solve(&[vec![-5.0, 0.0], vec![0.0, 1.0]], &[-1.0, 1.0], 1e-9, 100).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert!((r.eigenvalue + 5.0).abs() < 1e-9);
    let r = solve(&[vec![0.0]], &[1.0], 0.0, 5).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.eigenvalue, 0.0);
}
#[test]
fn rotation_exhausts_budget_and_residual_is_fresh() {
    let a = vec![vec![0.0, -1.0], vec![1.0, 0.0]];
    let r = solve(&a, &[1.0, 0.0], 1e-12, 7).unwrap();
    assert_eq!(r.status, Status::Budget);
    assert_eq!(r.iterations, 7);
    assert_eq!(r.residual, 1.0);
    let [x, y] = [r.eigenvector[0], r.eigenvector[1]];
    assert_eq!(
        r.residual,
        (-y - r.eigenvalue * x).hypot(x - r.eigenvalue * y)
    );
}
#[test]
fn validation_and_repeatability() {
    let a = vec![vec![1.0]];
    assert_eq!(solve(&a, &[0.0], 0.0, 0), Err(Error::ZeroInitialVector));
    for s in [vec![], vec![f64::NAN], vec![1e101]] {
        assert_eq!(solve(&a, &s, 0.0, 0), Err(Error::InvalidInput));
    }
    assert_eq!(solve(&[], &[], 0.0, 0), Err(Error::InvalidInput));
    assert_eq!(solve(&a, &[1.0], 0.0, 4097), Err(Error::InvalidInput));
    assert_eq!(solve(&a, &[1.0], -1.0, 0), Err(Error::InvalidInput));
    let r = solve(&a, &[-1.0], 0.0, 0).unwrap();
    assert_eq!(r, solve(&a, &[-1.0], 0.0, 0).unwrap());
}
