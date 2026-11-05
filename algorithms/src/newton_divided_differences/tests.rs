use super::*;
#[test]
fn polynomial_oracle() {
    for degree in 0..=5 {
        let xs = [-2.0_f64, -1.0, 0.0, 1.0, 2.0, 3.0];
        let ys = xs.map(|x| x.powi(degree));
        let p = NewtonPolynomial::new(&xs, &ys).unwrap();
        for k in -8..=14 {
            let x = k as f64 / 4.0;
            assert!((p.evaluate(x).unwrap() - x.powi(degree)).abs() < 1e-9);
        }
    }
    let p = NewtonPolynomial::new(&[0.0, 1.0, 2.0], &[1.0, 4.0, 9.0]).unwrap();
    assert_eq!(p.coefficients(), &[1.0, 3.0, 1.0]);
}
#[test]
fn boundaries() {
    assert_eq!(
        NewtonPolynomial::new(&[], &[]).unwrap_err(),
        Error::InvalidInput
    );
    assert_eq!(
        NewtonPolynomial::new(&[1.0, 0.0], &[1.0, 2.0]).unwrap_err(),
        Error::InvalidInput
    );
    assert_eq!(
        NewtonPolynomial::new(&[0.0, 1.0], &[-f64::MAX, f64::MAX]).unwrap_err(),
        Error::NonFiniteArithmetic
    );
    let p = NewtonPolynomial::new(&[0.0, 1.0], &[0.0, 2.0]).unwrap();
    assert_eq!(p.evaluate(f64::MAX), Err(Error::NonFiniteArithmetic));
    assert_eq!(p.evaluate(f64::NAN), Err(Error::InvalidInput));
}
#[test]
fn ambiguous_query() {
    let a = NewtonPolynomial::new(&[-1.0, 1.0], &[-1.0, 1.0]).unwrap();
    let b = NewtonPolynomial::new(&[-1.0, 1.0], &[0.0, 0.0]).unwrap();
    assert_eq!(a.evaluate(0.0), b.evaluate(0.0));
}
