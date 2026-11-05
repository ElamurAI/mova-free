//! Independent Gram-product fixtures for the Cholesky equations (Golub and Van Loan, section 4.2).
use super::*;
#[test]
fn exhaustive_gram_reconstruction() {
    for d in 1..=4 {
        for e in 1..=4 {
            for f in 1..=4 {
                for s in -2..=2 {
                    let l = vec![
                        vec![d as f64, 0.0, 0.0],
                        vec![s as f64, e as f64, 0.0],
                        vec![1.0, -1.0, f as f64],
                    ];
                    let a: Vec<Vec<f64>> = (0..3)
                        .map(|i| {
                            (0..3)
                                .map(|j| (0..3).map(|k| l[i][k] * l[j][k]).sum())
                                .collect()
                        })
                        .collect();
                    let got = factor(&a).unwrap();
                    assert_eq!(got.lower, l);
                    assert_eq!(got.reconstruction_residual, 0.0);
                }
            }
        }
    }
}
#[test]
fn irrational_factor_residual() {
    let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
    let got = factor(&a).unwrap();
    assert!(got.reconstruction_residual < 1e-14);
    assert!(got.lower[0][0] > 0.0);
    assert_eq!(got.lower[0][1], 0.0);
}
#[test]
fn rejects_structure_and_pivots() {
    assert_eq!(
        factor(&[vec![1.0, 1.0], vec![0.0, 1.0]]),
        Err(Error::NotSymmetric)
    );
    for a in [
        vec![vec![0.0]],
        vec![vec![-1.0]],
        vec![vec![1.0, 2.0], vec![2.0, 1.0]],
    ] {
        assert_eq!(factor(&a), Err(Error::NonPositivePivot));
    }
}
#[test]
fn validation_precedes_structure() {
    for a in [
        vec![],
        vec![vec![]],
        vec![vec![f64::NAN]],
        vec![vec![1e101]],
        vec![vec![0.0; 65]; 65],
    ] {
        assert_eq!(factor(&a), Err(Error::InvalidInput));
    }
    assert_eq!(
        factor(&[vec![0.0, 1.0], vec![f64::INFINITY, 0.0]]),
        Err(Error::InvalidInput)
    );
}
#[test]
fn boundary_scales() {
    for v in [f64::from_bits(1), 1e100] {
        let r = factor(&[vec![v]]).unwrap();
        assert!(r.lower[0][0] > 0.0);
        assert!(r.reconstruction_residual.is_finite());
    }
    let mut a = vec![vec![0.0; 64]; 64];
    for (i, row) in a.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    assert_eq!(factor(&a).unwrap().lower, a);
}
