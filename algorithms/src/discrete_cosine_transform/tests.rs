use super::*;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-10 * (1.0 + b.abs()), "{a} != {b}");
}
#[test]
fn basis_and_direct_sum() {
    for n in 1..=19 {
        for impulse in 0..n {
            let mut x = vec![0.0; n];
            x[impulse] = 1.0;
            let y = transform(&x, false).unwrap();
            for (k, &v) in y.iter().enumerate() {
                let basis = (std::f64::consts::PI * (2 * impulse + 1) as f64 * k as f64
                    / (2 * n) as f64)
                    .cos();
                close(
                    v,
                    basis * (if k == 0 { 1.0 } else { 2.0 } / n as f64).sqrt(),
                );
            }
        }
    }
}
#[test]
fn roundtrip_and_energy() {
    for n in 0..=65 {
        let x: Vec<_> = (0..n).map(|i| ((i * 17 % 23) as f64) - 11.0).collect();
        let y = transform(&x, false).unwrap();
        let z = transform(&y, true).unwrap();
        for (&a, &b) in x.iter().zip(&z) {
            close(a, b);
        }
        close(x.iter().map(|v| v * v).sum(), y.iter().map(|v| v * v).sum());
    }
}
#[test]
fn dc_and_limits() {
    let y = transform(&[3.0; 8], false).unwrap();
    close(y[0], 3.0 * 8.0_f64.sqrt());
    for &v in &y[1..] {
        close(v, 0.0);
    }
    assert_eq!(transform(&[], true), Ok(vec![]));
    assert_eq!(transform(&vec![0.0; 2049], false), Err(Error::InvalidInput));
    assert!(
        transform(&vec![0.0; 2048], true)
            .unwrap()
            .iter()
            .all(|&v| v == 0.0)
    );
}
#[test]
fn invalid_values() {
    for x in [f64::NAN, f64::INFINITY, -f64::INFINITY, 1.01e50] {
        assert_eq!(transform(&[x], false), Err(Error::InvalidInput));
    }
    assert_eq!(transform(&[1e50], false).unwrap(), vec![1e50]);
}
