use super::*;
#[test]
fn diagonal_full_span_oracle() {
    let a = vec![vec![1.0, 0.0], vec![0.0, 3.0]];
    let r = tridiagonalize(&a, &[1.0, 1.0], 1e-14, 2).unwrap();
    assert_eq!(r.status, Status::Complete);
    assert_eq!(r.steps, 2);
    assert!((r.diagonal[0] - 2.0).abs() < 1e-14);
    assert!((r.diagonal[1] - 2.0).abs() < 1e-14);
    assert!((r.off_diagonal[0] - 1.0).abs() < 1e-14);
    assert!(r.residual < 1e-14);
    assert!(r.orthogonality_residual < 1e-14);
}
#[test]
fn invariant_subspace_breakdown() {
    let r = tridiagonalize(&[vec![2.0, 0.0], vec![0.0, 3.0]], &[-1.0, 0.0], 0.0, 2).unwrap();
    assert_eq!(r.status, Status::Breakdown);
    assert_eq!(r.steps, 1);
    assert_eq!(r.residual, 0.0);
    assert_eq!(r.diagonal, vec![2.0]);
    assert!(r.off_diagonal.is_empty());
}
#[test]
fn zero_and_short_budgets_do_not_claim_convergence() {
    let a = vec![vec![1.0, 0.0], vec![0.0, 3.0]];
    let r = tridiagonalize(&a, &[1.0, 1.0], 0.0, 0).unwrap();
    assert_eq!(r.status, Status::Budget);
    assert_eq!(r.steps, 0);
    assert_eq!(r.residual, 0.0);
    let r = tridiagonalize(&a, &[1.0, 1.0], 0.0, 1).unwrap();
    assert_eq!(r.status, Status::Budget);
    assert!((r.residual - 1.0).abs() < 1e-14);
}
#[test]
fn tridiagonal_matrix_from_coordinate_start() {
    let a = vec![
        vec![2.0, 1.0, 0.0],
        vec![1.0, 3.0, 2.0],
        vec![0.0, 2.0, 4.0],
    ];
    let r = tridiagonalize(&a, &[1.0, 0.0, 0.0], 0.0, 99).unwrap();
    assert_eq!(r.diagonal, vec![2.0, 3.0, 4.0]);
    assert_eq!(r.off_diagonal, vec![1.0, 2.0]);
    assert_eq!(r.residual, 0.0);
    assert_eq!(r.orthogonality_residual, 0.0);
    assert_eq!(r.status, Status::Complete);
}
#[test]
fn independent_projection_and_orthogonality() {
    let a = vec![
        vec![2.0, 1.0, 2.0],
        vec![1.0, -1.0, 3.0],
        vec![2.0, 3.0, 4.0],
    ];
    let r = tridiagonalize(&a, &[1.0, 2.0, 3.0], 1e-12, 3).unwrap();
    assert_eq!(r.status, Status::Complete);
    assert!(r.residual < 1e-12);
    assert!(r.orthogonality_residual < 1e-12);
    for i in 0..3 {
        for j in 0..3 {
            let mut s = 0.0;
            for (u, row) in a.iter().enumerate() {
                for (v, &entry) in row.iter().enumerate() {
                    s += r.basis[i][u] * entry * r.basis[j][v];
                }
            }
            let expected = if i == j {
                r.diagonal[i]
            } else if i + 1 == j {
                r.off_diagonal[i]
            } else if j + 1 == i {
                r.off_diagonal[j]
            } else {
                0.0
            };
            assert!((s - expected).abs() < 1e-12);
        }
    }
    assert_eq!(r, tridiagonalize(&a, &[1.0, 2.0, 3.0], 1e-12, 3).unwrap());
}
#[test]
fn scalar_zero_and_threshold_precedence() {
    let r = tridiagonalize(&[vec![0.0]], &[1.0], 0.0, 1).unwrap();
    assert_eq!(r.status, Status::Complete);
    let r = tridiagonalize(&[vec![1.0, 0.0], vec![0.0, 3.0]], &[1.0, 1.0], 2.0, 1).unwrap();
    assert_eq!(r.status, Status::Breakdown);
    assert!(r.residual > 0.9);
}
#[test]
fn invalid_inputs() {
    assert_eq!(tridiagonalize(&[], &[], 0.0, 0), Err(Error::InvalidInput));
    assert_eq!(
        tridiagonalize(&[vec![1.0]], &[0.0], 0.0, 0),
        Err(Error::ZeroInitialVector)
    );
    assert_eq!(
        tridiagonalize(&[vec![1.0]], &[], 0.0, 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        tridiagonalize(&[vec![1.0]], &[1.0], 0.0, 4097),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        tridiagonalize(&[vec![1.0, 2.0], vec![0.0, 1.0]], &[1.0, 0.0], 0.0, 2),
        Err(Error::NotSymmetric)
    );
}
