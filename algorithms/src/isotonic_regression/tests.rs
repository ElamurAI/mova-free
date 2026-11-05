use super::*;
fn oracle(a: &[f64]) -> Vec<f64> {
    if a.is_empty() {
        return vec![];
    }
    let mut best = vec![];
    let mut score = f64::INFINITY;
    for mask in 0..1usize << (a.len() - 1) {
        let mut y = vec![];
        let mut start = 0;
        for end in 1..=a.len() {
            if end == a.len() || mask & (1 << (end - 1)) != 0 {
                let mean = a[start..end].iter().sum::<f64>() / (end - start) as f64;
                y.resize(end, mean);
                start = end;
            }
        }
        if y.windows(2).any(|w| w[0] > w[1]) {
            continue;
        }
        let cost = a.iter().zip(&y).map(|(x, y)| (x - y).powi(2)).sum::<f64>();
        if cost < score {
            score = cost;
            best = y
        }
    }
    best
}
#[test]
fn partition_oracle() {
    for code in 0..625 {
        let mut c = code;
        let a: Vec<f64> = (0..4)
            .map(|_| {
                let x = (c % 5) as f64 - 2.0;
                c /= 5;
                x
            })
            .collect();
        let y = isotonic_regression(&a).unwrap();
        let want = oracle(&a);
        assert!(
            y.iter().zip(want).all(|(x, y)| (x - y).abs() < 1e-12),
            "partition oracle"
        );
        let mut residual = 0.0;
        for i in 0..a.len() {
            residual += a[i] - y[i];
            assert!(residual >= -1e-12, "prefix KKT");
        }
        assert!(residual.abs() < 1e-12);
    }
}
#[test]
fn boundaries_and_fibers() {
    assert_eq!(isotonic_regression(&[]), Ok(vec![]));
    assert_eq!(isotonic_regression(&[2.0, 0.0]), Ok(vec![1.0, 1.0]));
    assert_eq!(isotonic_regression(&[1.0, 1.0]), Ok(vec![1.0, 1.0]));
    assert_eq!(
        isotonic_regression(&[f64::MAX, f64::MAX / 2.0]),
        Err(IsotonicError::Arithmetic)
    );
    assert_eq!(
        isotonic_regression(&[-f64::MAX, f64::MAX]),
        Ok(vec![-f64::MAX, f64::MAX])
    );
    assert_eq!(
        isotonic_regression(&[f64::NAN]),
        Err(IsotonicError::NonFinite)
    );
    assert_eq!(
        isotonic_regression(&[f64::INFINITY]),
        Err(IsotonicError::NonFinite)
    );
    assert_eq!(
        isotonic_regression(&vec![0.0; MAX_LEN + 1]),
        Err(IsotonicError::TooLarge)
    );
}
