use super::*;
#[test]
fn analytic_oracle() {
    let s = NaturalCubicSpline::new(&[0.0, 1.0, 2.0], &[0.0, 1.0, 0.0]).unwrap();
    for k in 0..=16 {
        let x = k as f64 / 8.0;
        let t = if x <= 1.0 { x } else { 2.0 - x };
        let expected = 1.5 * t - 0.5 * t * t * t;
        assert!((s.evaluate(x).unwrap() - expected).abs() < 1e-14);
    }
    let linear = NaturalCubicSpline::new(&[-2.0, -0.5, 3.0], &[-3.0, 0.0, 7.0]).unwrap();
    assert!((linear.evaluate(0.3).unwrap() - 1.6).abs() < 1e-14);
}
#[test]
fn boundaries() {
    assert_eq!(
        NaturalCubicSpline::new(&[0.0], &[0.0]).unwrap_err(),
        Error::InvalidInput
    );
    assert_eq!(
        NaturalCubicSpline::new(&[0.0, 0.0], &[0.0, 1.0]).unwrap_err(),
        Error::InvalidInput
    );
    assert_eq!(
        NaturalCubicSpline::new(&[-f64::MAX, f64::MAX], &[0.0, 0.0]).unwrap_err(),
        Error::NonFiniteArithmetic
    );
    let s = NaturalCubicSpline::new(&[0.0, 1.0], &[2.0, 4.0]).unwrap();
    assert_eq!(s.evaluate(0.5), Ok(3.0));
    assert_eq!(s.evaluate(-1.0), Err(Error::InvalidInput));
    assert_eq!(s.evaluate(f64::NAN), Err(Error::InvalidInput));
}
#[test]
fn knots_and_smoothness() {
    let s = NaturalCubicSpline::new(&[0.0, 1.0, 2.0], &[0.0, 1.0, 0.0]).unwrap();
    assert_eq!(s.second, &[0.0, -3.0, 0.0]);
    for (x, y) in [(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)] {
        assert_eq!(s.evaluate(x), Ok(y));
    }
    let e = 1e-5;
    assert!((s.evaluate(1.0 - e).unwrap() - s.evaluate(1.0 + e).unwrap()).abs() < 1e-14);
}
