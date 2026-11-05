use super::*;
#[test]
fn bernstein_oracle() {
    for n in 0..12 {
        let c: Vec<_> = (0..=n).map(|i| (i * i) as f64 - 3.0).collect();
        for j in 0..21 {
            let t = j as f64 / 20.0;
            let mut choose = 1.0;
            let mut sum = 0.0;
            for (i, &v) in c.iter().enumerate() {
                if i > 0 {
                    choose *= (n - i + 1) as f64 / i as f64;
                }
                sum += choose * (1.0 - t).powi((n - i) as i32) * t.powi(i as i32) * v;
            }
            assert!(
                (evaluate(&c, t).unwrap() - sum).abs() < 1e-10,
                "Bernstein oracle"
            );
        }
    }
}
#[test]
fn hull_endpoints_and_fibers() {
    let c = [-4.0, 9.0, -2.0, 3.0];
    assert_eq!(evaluate(&c, 0.0), Ok(-4.0));
    assert_eq!(evaluate(&c, 1.0), Ok(3.0));
    for i in 0..101 {
        assert!((-4.0..=9.0).contains(&evaluate(&c, i as f64 / 100.0).unwrap()));
    }
    assert_eq!(evaluate(&[0.0, 2.0], 0.5), evaluate(&[-1.0, 3.0], 0.5));
}
#[test]
fn invalid() {
    assert_eq!(evaluate(&[], f64::NAN), Err(Error::Size));
    assert_eq!(evaluate(&[f64::INFINITY], 0.0), Err(Error::NonFinite));
    assert_eq!(evaluate(&[1.0], -0.1), Err(Error::Domain));
    assert_eq!(evaluate(&[1e101], 0.0), Err(Error::Domain));
}
