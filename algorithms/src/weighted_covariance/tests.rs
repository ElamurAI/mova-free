use super::*;
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() < 1e-10 * (1.0 + b.abs()),
        "moments oracle: {a} vs {b}"
    );
}
#[test]
fn independent_batch_moments() {
    let points = [[1.0, -2.0], [3.0, 4.0], [-5.0, 2.0], [0.0, 7.0]];
    for code in 1..256usize {
        let weights: Vec<f64> = (0..4).map(|i| ((code >> (2 * i)) & 3) as f64).collect();
        let total: f64 = weights.iter().sum();
        if total == 0.0 {
            continue;
        }
        let mean: Vec<f64> = (0..2)
            .map(|d| (0..4).map(|i| weights[i] * points[i][d]).sum::<f64>() / total)
            .collect();
        let mut all = WeightedCovariance::new(2).unwrap();
        for i in 0..4 {
            all.push(&points[i], weights[i]).unwrap();
        }
        close(all.weight(), total);
        for (d, &expected) in mean.iter().enumerate() {
            close(all.mean().unwrap()[d], expected);
        }
        let cov = all.covariance().unwrap();
        for d in 0..2 {
            for e in 0..2 {
                let expected = (0..4)
                    .map(|i| weights[i] * (points[i][d] - mean[d]) * (points[i][e] - mean[e]))
                    .sum::<f64>()
                    / total;
                close(cov[d][e], expected);
            }
        }
        for split in 0..=4 {
            let mut a = WeightedCovariance::new(2).unwrap();
            let mut b = a.clone();
            for i in 0..split {
                a.push(&points[i], weights[i]).unwrap();
            }
            for i in split..4 {
                b.push(&points[i], weights[i]).unwrap();
            }
            a.merge(&b).unwrap();
            for d in 0..2 {
                close(a.mean().unwrap()[d], mean[d]);
                for (e, &expected) in cov[d].iter().enumerate() {
                    close(a.covariance().unwrap()[d][e], expected);
                }
            }
        }
    }
}
#[test]
fn zeros_errors_and_atomicity() {
    assert_eq!(WeightedCovariance::new(0), Err(Error::Dimension));
    assert_eq!(WeightedCovariance::new(65), Err(Error::Dimension));
    assert!(WeightedCovariance::new(64).is_ok());
    let mut a = WeightedCovariance::new(1).unwrap();
    assert_eq!(a.mean(), None);
    assert_eq!(a.covariance(), None);
    a.push(&[1.0], 0.0).unwrap();
    assert_eq!(a.weight(), 0.0);
    a.push(&[2.0], 1.0).unwrap();
    let saved = a.clone();
    assert_eq!(a.push(&[], f64::NAN), Err(Error::Dimension));
    assert_eq!(a.push(&[f64::NAN], 0.0), Err(Error::Value));
    assert_eq!(a.push(&[1.0], -1.0), Err(Error::Weight));
    assert_eq!(
        a.merge(&WeightedCovariance::new(2).unwrap()),
        Err(Error::Dimension)
    );
    assert_eq!(a, saved);
    let mut huge = WeightedCovariance::new(1).unwrap();
    huge.push(&[-1e100], 1e100).unwrap();
    huge.push(&[1e100], 1e100).unwrap();
    assert!(huge.covariance().unwrap()[0][0].is_finite());
}
#[test]
fn overflowing_merge_is_atomic() {
    let mut a = WeightedCovariance::new(1).unwrap();
    a.push(&[-1e100], 1e100).unwrap();
    // Repeated self-merges build huge mass without changing the mean.
    for _ in 0..650 {
        let b = a.clone();
        a.merge(&b).unwrap();
    }
    let mut b = a.clone();
    b.mean[0] = 1e100;
    let before = a.clone();
    assert_eq!(a.merge(&b), Err(Error::Arithmetic));
    assert_eq!(a, before);
}
