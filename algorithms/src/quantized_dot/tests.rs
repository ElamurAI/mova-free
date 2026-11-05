use super::*;
#[test]
fn residual_oracle() {
    for seed in 0..80 {
        let a: Vec<i64> = (0..9)
            .map(|i| ((seed * 173 + i * 37) % 2001) - 1000)
            .collect();
        let b: Vec<i64> = (0..9)
            .map(|i| ((seed * 83 + i * 97) % 2001) - 1000)
            .collect();
        let exact: i128 = a
            .iter()
            .zip(&b)
            .map(|(&x, &y)| i128::from(x) * i128::from(y))
            .sum();
        for p in [1, 2, 5, 10] {
            for step in [1, 2, 5, 10] {
                match dot(&a, &b, step, p, 9) {
                    Ok(c) => {
                        assert!(
                            (exact - c.value).unsigned_abs() <= c.absolute_error,
                            "absolute certificate"
                        );
                        assert!(
                            c.absolute_error * 100 <= c.l1_products * u128::from(p),
                            "normalized certificate"
                        );
                        if step == 1 {
                            assert_eq!(c.value, exact);
                        }
                    }
                    Err(Error::Uncertified) => {}
                    other => panic!("unexpected {other:?}"),
                }
            }
        }
    }
}
#[test]
fn cancellation_and_zero() {
    let c = dot(&[101, 101], &[101, -101], 10, 1, 2);
    assert_eq!(c, Err(Error::Uncertified));
    let c = dot(&[1001, 1001], &[1001, -1001], 10, 1, 2).unwrap();
    assert_eq!(c.value, 0);
    assert!(c.absolute_error > 0, "cancellation-safe metric");
    assert_eq!(dot(&[0, 7], &[9, 0], 100, 1, 2).unwrap().absolute_error, 0);
    assert_eq!(dot(&[], &[], 1, 1, 0).unwrap().value, 0);
}
#[test]
fn tier_boundary_and_validation() {
    // 9/90 = exactly 10 percent with one quantized operand.
    assert!(dot(&[10], &[9], 3, 10, 1).is_ok());
    assert_eq!(dot(&[10], &[9], 3, 5, 1), Err(Error::Uncertified));
    assert_eq!(dot(&[1], &[], 1, 1, 1), Err(Error::LengthMismatch));
    assert_eq!(dot(&[], &[], 0, 1, 0), Err(Error::ZeroStep));
    assert_eq!(dot(&[], &[], 1, 15, 0), Err(Error::UnsupportedPercent));
    assert_eq!(dot(&[1], &[1], 1, 1, 0), Err(Error::WorkBudget));
    assert_eq!(
        dot(&[i64::MIN; 3], &[i64::MIN; 3], 1, 1, 3),
        Err(Error::Overflow)
    );
    assert_eq!(
        dot(&[i64::MIN], &[1], 1, 1, 1).unwrap().value,
        i128::from(i64::MIN)
    );
}
