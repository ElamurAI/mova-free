use super::*;
#[test]
fn cancellation_oracle() {
    assert_eq!(sum(&[1e16, 1.0, -1e16]), Ok(1.0), "cancellation_oracle");
    assert_eq!(sum(&[1.0, 1e16, -1e16]), Ok(1.0));
    for n in 0..100 {
        let v: Vec<i64> = (0..n).map(|i| (i * 7919) % 101 - 50).collect();
        let exact: i64 = v.iter().sum();
        let f: Vec<f64> = v.iter().map(|&x| x as f64).collect();
        assert_eq!(sum(&f), Ok(exact as f64));
    }
}
#[test]
fn boundaries_and_fibers() {
    assert_eq!(sum(&[]), Ok(0.0));
    assert_eq!(sum(&[1.0, 2.0]), sum(&[3.0]));
    assert_eq!(sum(&[f64::NAN]), Err(Error::InvalidInput));
    assert_eq!(sum(&[f64::INFINITY]), Err(Error::InvalidInput));
    assert_eq!(sum(&[f64::MAX, f64::MAX]), Err(Error::NonFiniteArithmetic));
    assert_eq!(
        sum(&[f64::from_bits(1), f64::from_bits(1)]),
        Ok(f64::from_bits(2))
    );
}
