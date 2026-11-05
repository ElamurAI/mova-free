//! Manufactured integer solutions independently multiply the triangular fixture.
use super::*;
#[test]
fn exhaustive_both_directions() {
    for d in 1..=4 {
        for e in 1..=4 {
            for s in -2..=2 {
                for p in -2..=2 {
                    for q in -2..=2 {
                        let lower = vec![vec![d as f64, 0.0], vec![s as f64, e as f64]];
                        let upper = vec![vec![d as f64, s as f64], vec![0.0, e as f64]];
                        for (a, t) in [(lower, Triangle::Lower), (upper, Triangle::Upper)] {
                            let wanted = vec![p as f64, q as f64];
                            let b: Vec<f64> = a
                                .iter()
                                .map(|r| r.iter().zip(&wanted).map(|(a, x)| a * x).sum())
                                .collect();
                            let report = solve(&a, &b, t).unwrap();
                            assert_eq!(report.x, wanted);
                            assert_eq!(report.residual_norm, 0.0);
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn structure_precedes_zero_diagonal() {
    assert_eq!(
        solve(
            &[vec![0.0, 1.0], vec![0.0, 0.0]],
            &[1.0, 1.0],
            Triangle::Lower
        ),
        Err(Error::NotTriangular)
    );
    assert_eq!(
        solve(&[vec![0.0]], &[1.0], Triangle::Upper),
        Err(Error::ZeroDiagonal)
    );
}
#[test]
fn validation_and_overflow() {
    for (a, b) in [
        (vec![], vec![]),
        (vec![vec![1.0]], vec![]),
        (vec![vec![f64::NAN]], vec![0.0]),
        (vec![vec![1e101]], vec![0.0]),
        (vec![vec![0.0; 65]; 65], vec![0.0; 65]),
    ] {
        assert_eq!(solve(&a, &b, Triangle::Lower), Err(Error::InvalidInput));
    }
    assert_eq!(
        solve(&[vec![f64::from_bits(1)]], &[1e100], Triangle::Lower),
        Err(Error::NonFiniteArithmetic)
    );
}
#[test]
fn signed_diagonal_and_bound() {
    let mut a = vec![vec![0.0; 64]; 64];
    for (i, r) in a.iter_mut().enumerate() {
        r[i] = -2.0;
    }
    let r = solve(&a, &[4.0; 64], Triangle::Lower).unwrap();
    assert_eq!(r.x, vec![-2.0; 64]);
    assert_eq!(r.residual_norm, 0.0);
}
