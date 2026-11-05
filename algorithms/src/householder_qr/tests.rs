//! Independent two-column normal-equation oracle on well-conditioned small integer fixtures.
use super::*;
#[test]
fn tall_least_squares_oracle() {
    for p in -3..=3 {
        for q in -3..=3 {
            for noise in -2..=2 {
                let a = vec![
                    vec![1.0, -1.0],
                    vec![1.0, 0.0],
                    vec![1.0, 1.0],
                    vec![1.0, 2.0],
                ];
                let b = vec![
                    (p - q + noise) as f64,
                    p as f64,
                    (p + q) as f64,
                    (p + 2 * q) as f64,
                ];
                let s: f64 = b.iter().sum();
                let t = -b[0] + b[2] + 2.0 * b[3];
                let expected = [(6.0 * s - 2.0 * t) / 20.0, (4.0 * t - 2.0 * s) / 20.0];
                let got = least_squares(&a, &b).unwrap();
                for (i, &v) in expected.iter().enumerate() {
                    assert!((got.x[i] - v).abs() < 1e-12);
                }
                let residual = a
                    .iter()
                    .zip(&b)
                    .map(|(r, b)| (r[0] * expected[0] + r[1] * expected[1] - b).powi(2))
                    .sum::<f64>()
                    .sqrt();
                assert!((got.residual_norm - residual).abs() < 1e-12);
                assert!(got.normal_residual_norm < 1e-11);
            }
        }
    }
}
#[test]
fn signed_reflector_and_square() {
    for s in [-1.0, 1.0] {
        let a = vec![vec![s * 3.0, 1.0], vec![s * 4.0, 2.0]];
        let b = vec![s * 6.0 + 3.0, s * 8.0 + 6.0];
        let got = least_squares(&a, &b).unwrap();
        assert!((got.x[0] - 2.0).abs() < 1e-13);
        assert!((got.x[1] - 3.0).abs() < 1e-13);
    }
}
#[test]
fn exact_zero_rank_failure() {
    assert_eq!(
        least_squares(&[vec![0.0]], &[1.0]),
        Err(Error::RankDeficient)
    );
    assert_eq!(
        least_squares(&[vec![1.0, 0.0], vec![0.0, 0.0]], &[1.0, 1.0]),
        Err(Error::RankDeficient)
    );
}
#[test]
fn validation_and_arithmetic() {
    for (a, b) in [
        (vec![], vec![]),
        (vec![vec![1.0, 2.0]], vec![1.0]),
        (vec![vec![f64::NAN]], vec![0.0]),
        (vec![vec![1.0]; 129], vec![0.0; 129]),
        (vec![vec![1.0]], vec![1e101]),
    ] {
        assert_eq!(least_squares(&a, &b), Err(Error::InvalidInput));
    }
    assert_eq!(
        least_squares(&[vec![f64::from_bits(1)]], &[1e100]),
        Err(Error::NonFiniteArithmetic)
    );
}
#[test]
fn maximum_shape() {
    let mut a = vec![vec![0.0; 64]; 128];
    for (i, row) in a.iter_mut().take(64).enumerate() {
        row[i] = 1.0;
    }
    let got = least_squares(&a, &[1.0; 128]).unwrap();
    assert!(got.x.iter().all(|v| (*v - 1.0).abs() < 1e-14));
    assert!((got.residual_norm - 8.0).abs() < 1e-13);
}
