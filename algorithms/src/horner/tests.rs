use super::*;
#[test]
fn power_oracle() {
    for n in 0..15 {
        for j in -8..=8 {
            let x = j as f64 / 8.0;
            let c: Vec<_> = (0..n).map(|i| ((i * 7) % 13) as f64 - 6.0).collect();
            let v: f64 = c
                .iter()
                .enumerate()
                .map(|(i, a)| a * x.powi(i as i32))
                .sum();
            let d: f64 = c
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, a)| i as f64 * a * x.powi(i as i32 - 1))
                .sum();
            let got = evaluate(&c, x).unwrap();
            assert!(
                (got.0 - v).abs() < 1e-10 && (got.1 - d).abs() < 1e-10,
                "power_oracle"
            );
        }
    }
}
#[test]
fn boundaries_and_fibers() {
    assert_eq!(evaluate(&[], 0.0), Ok((0.0, 0.0)));
    assert_eq!(evaluate(&[2.0, 3.0, 7.0], 0.0), Ok((2.0, 3.0)));
    assert_eq!(
        evaluate(&[2.0, 3.0, 7.0], 0.0),
        evaluate(&[2.0, 3.0, 99.0], 0.0)
    );
    assert_eq!(evaluate(&[f64::NAN], 0.0), Err(Error::InvalidInput));
    assert_eq!(evaluate(&[], f64::INFINITY), Err(Error::InvalidInput));
    assert_eq!(
        evaluate(&[f64::MAX; 3], 2.0),
        Err(Error::NonFiniteArithmetic)
    );
}

fn reference_per_step(coefficients: &[f64], x: f64) -> Result<(f64, f64), Error> {
    if !x.is_finite() || coefficients.iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput);
    }
    let (mut value, mut derivative) = (0.0, 0.0);
    for &coefficient in coefficients.iter().rev() {
        derivative = derivative * x + value;
        value = value * x + coefficient;
        if !value.is_finite() || !derivative.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
    }
    Ok((value, derivative))
}

#[test]
fn final_pair_gate_preserves_reference_observations() {
    // The value cancels to zero while only the derivative overflows.
    let derivative_overflow = [-f64::MAX, f64::MAX / 2.0, f64::MAX / 2.0];
    assert_eq!(
        evaluate(&derivative_overflow, 1.0),
        Err(Error::NonFiniteArithmetic),
        "W20-HORNER-DERIVATIVE-GATE"
    );
    let alphabet = [
        -f64::MAX,
        -1e200,
        -1.0,
        -0.0,
        0.0,
        f64::from_bits(1),
        f64::MIN_POSITIVE,
        1.0,
        1e200,
        f64::MAX,
    ];
    let arguments = [
        -f64::MAX,
        -2.0,
        -1.0,
        -0.0,
        0.0,
        f64::from_bits(1),
        0.5,
        1.0,
        2.0,
        f64::MAX,
        f64::INFINITY,
        f64::NAN,
    ];
    for len in 0..=4u32 {
        for mut code in 0..alphabet.len().pow(len) {
            let coefficients: Vec<_> = (0..len)
                .map(|_| {
                    let value = alphabet[code % alphabet.len()];
                    code /= alphabet.len();
                    value
                })
                .collect();
            for x in arguments {
                match (
                    evaluate(&coefficients, x),
                    reference_per_step(&coefficients, x),
                ) {
                    (Ok(a), Ok(b)) => {
                        assert_eq!(
                            (a.0.to_bits(), a.1.to_bits()),
                            (b.0.to_bits(), b.1.to_bits())
                        );
                    }
                    (Err(a), Err(b)) => assert_eq!(a, b),
                    pair => panic!("W20-HORNER-REFERENCE: {pair:?}"),
                }
            }
        }
    }
    for coefficients in [[f64::NAN, f64::MAX], [f64::MAX, f64::INFINITY]] {
        assert_eq!(
            evaluate(&coefficients, 2.0),
            reference_per_step(&coefficients, 2.0)
        );
    }
}
