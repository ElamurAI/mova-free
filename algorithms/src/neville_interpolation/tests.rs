use super::*;
#[test]
fn polynomial_oracle() {
    for degree in 0..=5 {
        let xs = [-2.0_f64, -1.0, 0.0, 1.0, 2.0, 3.0];
        let ys = xs.map(|x| x.powi(degree));
        for k in -8..=14 {
            let x = k as f64 / 4.0;
            assert!((neville_interpolation(&xs, &ys, x).unwrap() - x.powi(degree)).abs() < 1e-9);
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(neville_interpolation(&[2.0], &[9.0], -7.0), Ok(9.0));
    assert_eq!(
        neville_interpolation(&[], &[], 0.0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        neville_interpolation(&[0.0, 0.0], &[1.0, 2.0], 0.0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        neville_interpolation(&[0.0], &[f64::NAN], 0.0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        neville_interpolation(&[-f64::MAX, f64::MAX], &[1.0, 2.0], 0.0),
        Err(Error::NonFiniteArithmetic)
    );
}
#[test]
fn ambiguous_query() {
    assert_eq!(
        neville_interpolation(&[-1.0, 1.0], &[-1.0, 1.0], 0.0),
        neville_interpolation(&[-1.0, 1.0], &[0.0, 0.0], 0.0)
    );
}
