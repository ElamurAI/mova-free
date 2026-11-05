use super::*;
#[test]
fn diagonal_clipping_oracle() {
    for b in -6..=6 {
        let q = vec![vec![2.0, 0.0], vec![0.0, 4.0]];
        let linear = [b as f64, -b as f64];
        let r = minimize(
            &q,
            &linear,
            &[-1.0, -2.0],
            &[1.0, 2.0],
            &[9.0, 9.0],
            1e-10,
            200,
        )
        .unwrap();
        assert_eq!(r.status, Status::Converged);
        assert!((r.x[0] - (linear[0] / 2.0).clamp(-1.0, 1.0)).abs() < 1e-8);
        assert!((r.x[1] - (linear[1] / 4.0).clamp(-2.0, 2.0)).abs() < 1e-8);
    }
}
#[test]
fn dense_stationary_oracle() {
    let q = vec![vec![3.0, 1.0], vec![1.0, 2.0]];
    let r = minimize(
        &q,
        &[5.0, 5.0],
        &[-10.0; 2],
        &[10.0; 2],
        &[0.0; 2],
        1e-12,
        1000,
    )
    .unwrap();
    assert_eq!(r.status, Status::Converged);
    assert!((r.x[0] - 1.0).abs() < 1e-9);
    assert!((r.x[1] - 2.0).abs() < 1e-9);
    assert!((r.value + 7.5).abs() < 1e-9);
}
#[test]
fn linear_box_and_budget() {
    let q = vec![vec![0.0]];
    let r = minimize(&q, &[2.0], &[-1.0], &[3.0], &[0.0], 1e-12, 10).unwrap();
    assert_eq!(r.x, [3.0]);
    assert_eq!(r.status, Status::Converged);
    let r = minimize(&q, &[2.0], &[-1.0], &[3.0], &[0.0], 1e-12, 0).unwrap();
    assert_eq!(r.status, Status::BudgetExhausted);
}
#[test]
fn validation() {
    let q = vec![vec![-1.0]];
    assert_eq!(
        minimize(&q, &[1.0], &[0.0], &[1.0], &[0.0], 1e-8, 1),
        Err(Error::InvalidInput)
    );
    let q = vec![vec![2.0, 1.0], vec![0.0, 2.0]];
    assert_eq!(
        minimize(&q, &[1.0; 2], &[0.0; 2], &[1.0; 2], &[0.0; 2], 1e-8, 1),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(&[], &[], &[], &[], &[], 1e-8, 1),
        Err(Error::InvalidInput)
    );
}
