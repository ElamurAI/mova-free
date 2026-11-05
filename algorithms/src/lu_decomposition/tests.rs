use super::*;
#[test]
fn pivot_oracle() {
    let a = vec![vec![0.0, 2.0], vec![3.0, 4.0]];
    let r = factor(&a).unwrap();
    assert_eq!(r.permutation, vec![1, 0]);
    assert_eq!(r.lower, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    assert_eq!(r.upper, vec![vec![3.0, 4.0], vec![0.0, 2.0]]);
    assert_eq!(r.reconstruction_residual, 0.0);
}
#[test]
fn exhaustive_small_determinants_and_reconstruction() {
    for a in -2..=2 {
        for b in -2..=2 {
            for c in -2..=2 {
                for d in -2..=2 {
                    let m = vec![vec![a as f64, b as f64], vec![c as f64, d as f64]];
                    let result = factor(&m);
                    if a * d == b * c {
                        assert!(matches!(result, Err(Error::SingularPivot { .. })));
                    } else {
                        let r = result.unwrap();
                        assert!(r.reconstruction_residual < 1e-12);
                        let sign = if r.permutation == vec![0, 1] {
                            1.0
                        } else {
                            -1.0
                        };
                        assert!(
                            (sign * r.upper[0][0] * r.upper[1][1] - (a * d - b * c) as f64).abs()
                                < 1e-12
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn later_swap_moves_completed_lower_columns() {
    let a = vec![
        vec![4.0, 1.0, 2.0],
        vec![1.0, 0.0, 3.0],
        vec![2.0, 5.0, 1.0],
    ];
    let r = factor(&a).unwrap();
    assert_eq!(r.permutation, vec![0, 2, 1]);
    assert_eq!(r.lower[1][0], 0.5);
    assert_eq!(r.lower[2][0], 0.25);
    assert!(r.reconstruction_residual < 1e-12);
}
#[test]
fn tie_and_scalar() {
    let r = factor(&[vec![2.0, 1.0], vec![-2.0, 3.0]]).unwrap();
    assert_eq!(r.permutation, vec![0, 1]);
    let r = factor(&[vec![-3.0]]).unwrap();
    assert_eq!(r.upper[0][0], -3.0);
    assert_eq!(factor(&[vec![0.0]]), Err(Error::SingularPivot { index: 0 }));
}
#[test]
fn invalid_and_helper_boundaries() {
    for a in [
        vec![],
        vec![vec![]],
        vec![vec![f64::NAN]],
        vec![vec![1e101]],
        vec![vec![1.0, 2.0]],
        vec![vec![0.0; 65]; 65],
    ] {
        assert_eq!(factor(&a), Err(Error::InvalidInput));
    }
    assert_eq!(finite(f64::INFINITY), Err(Error::NonFiniteArithmetic));
    assert_eq!(buffer(usize::MAX), Err(Error::AllocationFailed));
    assert_eq!(dot(&[f64::MAX], &[2.0]), Err(Error::NonFiniteArithmetic));
    assert_eq!(control(-1.0, 0), Err(Error::InvalidInput));
    assert_eq!(control(0.0, 4097), Err(Error::InvalidInput));
}
#[test]
fn maximum_dimension_identity() {
    let a = identity(64).unwrap();
    let r = factor(&a).unwrap();
    assert_eq!(r.lower, a);
    assert_eq!(r.reconstruction_residual, 0.0);
}
