use super::*;
#[test]
fn subgradient_grid() {
    for i in -100..=100 {
        for lambda in [0.0, 0.25, 1.0, 8.0] {
            let x = i as f64 / 8.0;
            let y = soft_threshold(&[x], lambda).unwrap()[0];
            if y == 0.0 {
                assert!(x.abs() <= lambda, "zero subgradient");
            } else {
                assert!(
                    (y - x + lambda * y.signum()).abs() < 1e-12,
                    "nonzero subgradient"
                );
            }
            for j in -120..=120 {
                let z = j as f64 / 8.0;
                assert!(
                    0.5 * (y - x).powi(2) + lambda * y.abs()
                        <= 0.5 * (z - x).powi(2) + lambda * z.abs() + 1e-12,
                    "objective oracle"
                );
            }
        }
    }
}
#[test]
fn boundaries_and_fibers() {
    assert_eq!(
        soft_threshold(&[-1.0, 0.0, 1.0], 1.0),
        Ok(vec![-0.0, 0.0, 0.0])
    );
    assert_eq!(
        soft_threshold(&[f64::MAX, -f64::MAX], f64::MAX),
        Ok(vec![0.0, -0.0])
    );
    assert_eq!(
        soft_threshold(&[f64::from_bits(1)], 0.0),
        Ok(vec![f64::from_bits(1)])
    );
    assert_eq!(soft_threshold(&[], 0.0), Ok(vec![]));
    assert_eq!(
        soft_threshold(&[f64::NAN], 0.0),
        Err(ThresholdError::NonFinite)
    );
    assert_eq!(
        soft_threshold(&[1.0], f64::INFINITY),
        Err(ThresholdError::NonFinite)
    );
    assert_eq!(
        soft_threshold(&[1.0], -1.0),
        Err(ThresholdError::NegativeLambda)
    );
    assert_eq!(
        soft_threshold(&vec![0.0; MAX_LEN + 1], 0.0),
        Err(ThresholdError::TooLarge)
    );
}
