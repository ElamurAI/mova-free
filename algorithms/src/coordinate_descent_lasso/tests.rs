use super::*;
#[test]
fn orthogonal_oracle() {
    let a = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
    for y in -8..=8 {
        let y = y as f64;
        let r = minimize(&a, &[y, -y], 1.0, 1e-10, 10).unwrap();
        let shrink = |v: f64| (v.abs() - 1.0).max(0.0) * v.signum();
        assert_eq!(r.status, Status::Converged);
        assert!((r.x[0] - shrink(2.0 * y) / 4.0).abs() < 1e-10);
        assert!((r.x[1] - shrink(-3.0 * y) / 9.0).abs() < 1e-10);
    }
}
#[test]
fn correlated_kkt_and_grid() {
    let a = vec![vec![1.0, 0.5], vec![0.5, 1.0], vec![1.0, -1.0]];
    let y = [2.0, 1.0, 1.0];
    let r = minimize(&a, &y, 0.25, 1e-10, 1000).unwrap();
    assert_eq!(r.status, Status::Converged);
    for i in -10..=30 {
        for j in -10..=30 {
            let x = [i as f64 / 10.0, j as f64 / 10.0];
            let value = 0.5
                * a.iter()
                    .zip(y)
                    .map(|(row, y)| (row[0] * x[0] + row[1] * x[1] - y).powi(2))
                    .sum::<f64>()
                + 0.25 * (x[0].abs() + x[1].abs());
            assert!(r.value <= value + 1e-9);
        }
    }
    assert!(r.residual <= 1e-10);
}
#[test]
fn zero_columns_and_budget() {
    let a = vec![vec![0.0, 1.0]];
    assert_eq!(minimize(&a, &[2.0], 1.0, 1e-9, 10).unwrap().x, [0.0, 1.0]);
    assert_eq!(
        minimize(&a, &[2.0], 1.0, 1e-9, 0).unwrap().status,
        Status::BudgetExhausted
    );
    assert_eq!(
        minimize(&a, &[2.0], 3.0, 1e-9, 0).unwrap().status,
        Status::Converged
    );
}
#[test]
fn invalid_and_repeatable() {
    assert_eq!(
        minimize(&[vec![1.0]], &[1.0], -1.0, 1e-9, 10),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(&[vec![f64::NAN]], &[1.0], 0.0, 1e-9, 10),
        Err(Error::InvalidInput)
    );
    let run = || {
        minimize(
            &[vec![1.0, 2.0], vec![3.0, 4.0]],
            &[2.0, 1.0],
            0.2,
            1e-9,
            1000,
        )
        .unwrap()
    };
    assert_eq!(run(), run());
}
#[test]
fn underflowing_nonzero_column_is_not_zero() {
    assert_eq!(
        minimize(&[vec![1e-200]], &[1.0], 0.0, 1e-250, 10),
        Err(Error::NonFiniteArithmetic)
    );
}
