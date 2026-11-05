use super::*;
#[test]
fn polynomial_oracle() {
    for order in 1..=5 {
        for degree in 0..2 * order {
            let a = -0.7_f64;
            let b = 1.3_f64;
            let exact =
                (b.powi(degree as i32 + 1) - a.powi(degree as i32 + 1)) / (degree + 1) as f64;
            let mut calls = 0;
            let y = integrate(
                |x| {
                    calls += 1;
                    x.powi(degree as i32)
                },
                a,
                b,
                order,
            )
            .unwrap();
            assert!((y - exact).abs() < 1e-12, "Gaussian polynomial oracle");
            assert_eq!(calls, order);
        }
    }
}
#[test]
fn validation_calls_and_fibers() {
    let mut calls = 0;
    assert_eq!(
        integrate(
            |_| {
                calls += 1;
                1.0
            },
            0.0,
            0.0,
            5
        ),
        Ok(0.0)
    );
    assert_eq!(calls, 0);
    assert_eq!(integrate(|_| 1.0, 0.0, 1.0, 0), Err(Error::Order));
    assert_eq!(integrate(|_| 1.0, 1.0, 0.0, 1), Err(Error::Domain));
    assert_eq!(integrate(|_| 1.0, f64::NAN, 0.0, 1), Err(Error::NonFinite));
    assert_eq!(integrate(|_| f64::NAN, 0.0, 1.0, 1), Err(Error::Callback));
    assert_eq!(
        integrate(|x| x, -1.0, 1.0, 3),
        integrate(|_| 0.0, -1.0, 1.0, 3)
    );
}
