use super::*;
#[test]
fn two_dimensional_oracle() {
    for b in -4..=4 {
        let q = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let linear = [b as f64, 0.0];
        let r = minimize(&q, &linear, &[0.5, 0.5], 1e-10, 100).unwrap();
        let t = ((2.0 + b as f64) / 4.0).clamp(0.0, 1.0);
        assert_eq!(r.status, Status::Converged);
        assert!((r.x[0] - t).abs() < 1e-8);
        assert!((r.x.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!(r.x.iter().all(|x| *x >= 0.0));
    }
}
#[test]
fn linear_vertex_and_tie() {
    let q = vec![vec![0.0; 3]; 3];
    let r = minimize(&q, &[2.0, 2.0, 0.0], &[0.0, 0.0, 1.0], 1e-12, 10).unwrap();
    assert_eq!(r.x, [1.0, 0.0, 0.0]);
    assert_eq!(r.value, -2.0);
    assert_eq!(r.status, Status::Converged);
}
#[test]
fn dense_grid_comparison() {
    let q = vec![vec![3.0, 1.0], vec![1.0, 2.0]];
    let b = [0.5, 0.25];
    let r = minimize(&q, &b, &[1.0, 0.0], 1e-10, 1000).unwrap();
    for k in 0..=100 {
        let a = k as f64 / 100.0;
        let c = 1.0 - a;
        let value = 0.5 * (3.0 * a * a + 2.0 * a * c + 2.0 * c * c) - b[0] * a - b[1] * c;
        assert!(r.value <= value + 1e-9);
    }
}
#[test]
fn budget_and_invalid() {
    let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    assert_eq!(
        minimize(&q, &[0.0; 2], &[1.0, 0.0], 1e-12, 0)
            .unwrap()
            .status,
        Status::BudgetExhausted
    );
    assert_eq!(
        minimize(&q, &[0.0; 2], &[1.0, 1.0], 1e-12, 10),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(&q, &[0.0; 2], &[-0.1, 1.1], 1e-12, 10),
        Err(Error::InvalidInput)
    );
}
