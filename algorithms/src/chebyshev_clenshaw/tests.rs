use super::*;
#[test]
fn basis_oracle() {
    for n in 0..20 {
        for j in -16..=16 {
            let x = j as f64 / 16.0;
            let c: Vec<_> = (0..n).map(|i| ((i * 7) % 11) as f64 - 5.0).collect();
            let (mut t0, mut t1) = (1.0, x);
            let mut expected = 0.0;
            for (i, &a) in c.iter().enumerate() {
                let t = if i == 0 {
                    t0
                } else if i == 1 {
                    t1
                } else {
                    let next = 2.0 * x * t1 - t0;
                    t0 = t1;
                    t1 = next;
                    next
                };
                expected += a * t;
            }
            assert!(
                (evaluate(&c, x).unwrap() - expected).abs() < 1e-10,
                "basis_oracle"
            );
        }
    }
}
#[test]
fn boundaries_and_fibers() {
    assert_eq!(evaluate(&[], 0.0), Ok(0.0));
    assert_eq!(evaluate(&[7.0], 1.0), Ok(7.0));
    assert_eq!(evaluate(&[2.0, 3.0], 0.0), evaluate(&[2.0, 9.0], 0.0));
    assert_eq!(evaluate(&[1.0, 2.0, 3.0], -1.0), Ok(2.0));
    assert_eq!(evaluate(&[f64::NAN], 0.0), Err(Error::InvalidInput));
    assert_eq!(evaluate(&[], 1.01), Err(Error::InvalidInput));
    assert_eq!(
        evaluate(&[f64::MAX; 4], 1.0),
        Err(Error::NonFiniteArithmetic)
    );
}
