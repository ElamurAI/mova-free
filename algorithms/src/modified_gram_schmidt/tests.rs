use super::*;
#[test]
fn orthogonal_oracle() {
    let r = factor(&[vec![3.0, 0.0], vec![0.0, -4.0], vec![0.0, 0.0]], 0.0).unwrap();
    assert_eq!(r.q, vec![vec![1.0, 0.0], vec![0.0, -1.0], vec![0.0, 0.0]]);
    assert_eq!(r.r, vec![vec![3.0, 0.0], vec![0.0, 4.0]]);
    assert_eq!(r.rank, 2);
    assert_eq!(r.orthogonality_residual, 0.0);
    assert_eq!(r.reconstruction_residual, 0.0);
}
#[test]
fn projection_oracle() {
    let r = factor(&[vec![1.0, 1.0], vec![0.0, 2.0]], 0.0).unwrap();
    assert_eq!(r.q, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    assert_eq!(r.r, vec![vec![1.0, 1.0], vec![0.0, 2.0]]);
}
#[test]
fn integer_grid_reconstructs_independently() {
    for a in 1..=4 {
        for b in -3..=3 {
            for c in 1..=4 {
                let m = vec![vec![a as f64, b as f64], vec![0.0, c as f64]];
                let r = factor(&m, 0.0).unwrap();
                for (i, row) in m.iter().enumerate() {
                    for (j, &value) in row.iter().enumerate() {
                        let s: f64 = (0..2).map(|k| r.q[i][k] * r.r[k][j]).sum();
                        assert!((s - value).abs() < 1e-12);
                    }
                }
                assert!(r.orthogonality_residual < 1e-12);
            }
        }
    }
}
#[test]
fn dependent_and_absolute_threshold() {
    assert_eq!(
        factor(&[vec![1.0, 2.0], vec![0.0, 0.0]], 0.0),
        Err(Error::DependentColumn { index: 1 })
    );
    assert_eq!(
        factor(&[vec![0.0]], 0.0),
        Err(Error::DependentColumn { index: 0 })
    );
    assert_eq!(
        factor(&[vec![1.0]], 1.0),
        Err(Error::DependentColumn { index: 0 })
    );
    assert!(factor(&[vec![1e-200]], 0.0).is_ok());
}
#[test]
fn invalid_shape_tolerance_and_input() {
    for a in [
        vec![],
        vec![vec![]],
        vec![vec![1.0, 2.0]],
        vec![vec![f64::INFINITY]],
        vec![vec![0.0; 65]; 65],
        vec![vec![1.0]; 129],
    ] {
        assert_eq!(factor(&a, 0.0), Err(Error::InvalidInput));
    }
    for t in [f64::NAN, -1.0, 1e101] {
        assert_eq!(factor(&[vec![1.0]], t), Err(Error::InvalidInput));
    }
}
#[test]
fn repeated_and_nearly_dependent_observations() {
    let a = vec![vec![1.0, 1.0], vec![1.0, 1.0 + 1e-12], vec![0.0, 1e-12]];
    let r = factor(&a, 0.0).unwrap();
    assert_eq!(r, factor(&a, 0.0).unwrap());
    assert!(r.reconstruction_residual < 1e-12);
    assert!(r.orthogonality_residual.is_finite());
}
