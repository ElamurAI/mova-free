use super::*;
#[test]
fn rank_one_oracle() {
    for r in 1..=4 {
        for c in 1..=4 {
            let matrix: Vec<Vec<f64>> = (0..r)
                .map(|i| (0..c).map(|j| (i + 1) as f64 * (j + 2) as f64).collect())
                .collect();
            let rows: Vec<f64> = (0..r)
                .map(|i| (r - i) as f64 / (r * (r + 1) / 2) as f64)
                .collect();
            let columns: Vec<f64> = (0..c)
                .map(|j| (j + 1) as f64 / (c * (c + 1) / 2) as f64)
                .collect();
            let result = sinkhorn_scaling(&matrix, &rows, &columns, 1e-12, 50).unwrap();
            assert!(result.converged);
            for (i, &a) in rows.iter().enumerate() {
                for (j, &b) in columns.iter().enumerate() {
                    assert!(
                        (result.matrix[i][j] - a * b).abs() < 1e-12,
                        "rank one oracle"
                    );
                }
            }
        }
    }
}
#[test]
fn cross_ratio_and_marginals() {
    let x = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let out = sinkhorn_scaling(&x, &[0.4, 0.6], &[0.7, 0.3], 1e-13, 1000).unwrap();
    assert!(out.converged);
    let y = &out.matrix;
    assert!((y[0][0] * y[1][1] / (y[0][1] * y[1][0]) - 2.0 / 3.0).abs() < 1e-12);
    let discrepancy = (y[0][0] + y[0][1] - 0.4)
        .abs()
        .max((y[1][0] + y[1][1] - 0.6).abs())
        .max((y[0][0] + y[1][0] - 0.7).abs())
        .max((y[0][1] + y[1][1] - 0.3).abs());
    assert_eq!(out.residual, discrepancy);
    let zero = sinkhorn_scaling(&x, &[0.4, 0.6], &[0.7, 0.3], 1e-13, 0).unwrap();
    assert!(!zero.converged);
    assert_eq!(zero.iterations, 0);
    assert_eq!(zero.matrix, x);
    let limited = sinkhorn_scaling(&x, &[0.4, 0.6], &[0.7, 0.3], 1e-15, 1).unwrap();
    assert!(!limited.converged);
    assert_eq!(limited.iterations, 1);
}
#[test]
fn validation() {
    assert_eq!(
        sinkhorn_scaling(&[], &[], &[], 1.0, 0),
        Err(Error::Dimension)
    );
    let one = vec![vec![1.0]];
    assert_eq!(
        sinkhorn_scaling(&one, &[1.0], &[1.0], 0.0, 0),
        Err(Error::Configuration)
    );
    assert_eq!(
        sinkhorn_scaling(&one, &[1.0], &[1.0], 1.0, 10001),
        Err(Error::Configuration)
    );
    for bad in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(
            sinkhorn_scaling(&[vec![bad]], &[1.0], &[1.0], 1e-6, 1),
            Err(Error::Value)
        );
    }
    assert_eq!(
        sinkhorn_scaling(&one, &[1.0], &[2.0], 1e-6, 1),
        Err(Error::MarginalMass)
    );
    assert!(
        sinkhorn_scaling(&one, &[1.0], &[1.0], 1e-6, 0)
            .unwrap()
            .converged
    );
}
