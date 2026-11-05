use super::*;
#[test]
fn two_by_two_oracle() {
    let r = solve(&[vec![2.0, 1.0], vec![1.0, 2.0]], 1e-12, 10).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.rotations, 1);
    let mut values = r.eigenvalues.clone();
    values.sort_by(f64::total_cmp);
    assert!((values[0] - 1.0).abs() < 1e-12);
    assert!((values[1] - 3.0).abs() < 1e-12);
    assert!(r.orthogonality_residual < 1e-12);
    assert!(r.residual < 1e-12);
}
#[test]
fn analytic_integer_grid() {
    for a in -3..=3 {
        for b in -2..=2 {
            for d in -3..=3 {
                let m = vec![vec![a as f64, b as f64], vec![b as f64, d as f64]];
                let r = solve(&m, 1e-12, 20).unwrap();
                let center = (a + d) as f64 / 2.0;
                let radius = (((a - d) as f64 / 2.0).powi(2) + (b * b) as f64).sqrt();
                let mut values = r.eigenvalues;
                values.sort_by(f64::total_cmp);
                assert!((values[0] - (center - radius)).abs() < 1e-12);
                assert!((values[1] - (center + radius)).abs() < 1e-12);
                assert_eq!(r.status, Status::Converged);
            }
        }
    }
}
#[test]
fn repeated_eigenvalues_and_scalar() {
    let r = solve(&[vec![2.0, 0.0], vec![0.0, 2.0]], 0.0, 0).unwrap();
    assert_eq!(r.rotations, 0);
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.eigenvectors, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    assert_eq!(
        solve(&[vec![-2.0]], 0.0, 0).unwrap().eigenvalues,
        vec![-2.0]
    );
}
#[test]
fn budget_and_true_residual() {
    let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
    let r = solve(&a, 0.0, 0).unwrap();
    assert_eq!(r.status, Status::Budget);
    assert!((r.residual - 2.0_f64.sqrt()).abs() < 1e-15);
}
#[test]
fn independent_reconstruction_and_determinism() {
    let a = vec![
        vec![4.0, 1.0, 2.0],
        vec![1.0, 3.0, -1.0],
        vec![2.0, -1.0, 2.0],
    ];
    let r = solve(&a, 1e-11, 100).unwrap();
    assert_eq!(r.status, Status::Converged);
    for (i, row) in a.iter().enumerate() {
        for (j, &entry) in row.iter().enumerate() {
            let s: f64 = (0..3)
                .map(|k| r.eigenvectors[i][k] * r.eigenvalues[k] * r.eigenvectors[j][k])
                .sum();
            assert!((s - entry).abs() < 1e-10);
        }
    }
    assert_eq!(r, solve(&a, 1e-11, 100).unwrap());
    for j in 0..3 {
        let mut p = 0;
        for i in 1..3 {
            if r.eigenvectors[i][j].abs() > r.eigenvectors[p][j].abs() {
                p = i;
            }
        }
        assert!(r.eigenvectors[p][j] >= 0.0);
    }
}
#[test]
fn rejected_domain() {
    assert_eq!(solve(&[], 0.0, 0), Err(Error::InvalidInput));
    assert_eq!(
        solve(&[vec![1.0, 2.0], vec![0.0, 1.0]], 0.0, 0),
        Err(Error::NotSymmetric)
    );
    for t in [-1.0, f64::NAN] {
        assert_eq!(solve(&[vec![1.0]], t, 0), Err(Error::InvalidInput));
    }
    assert_eq!(solve(&[vec![1.0]], 0.0, 4097), Err(Error::InvalidInput));
    assert_eq!(
        solve(&[vec![f64::INFINITY]], 0.0, 0),
        Err(Error::InvalidInput)
    );
}
