use super::*;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-10 * (1.0 + b.abs()), "{a} != {b}");
}
#[test]
fn direct_haar_basis() {
    for n in [1, 2, 4, 8, 16, 32] {
        let x: Vec<_> = (0..n).map(|i| (i * i % 17) as f64 - 8.0).collect();
        let y = transform(&x, false).unwrap();
        close(y[0], x.iter().sum::<f64>() / (n as f64).sqrt());
        let mut count = 1;
        while count < n {
            let width = n / count;
            for k in 0..count {
                let start = k * width;
                let expected = (x[start..start + width / 2].iter().sum::<f64>()
                    - x[start + width / 2..start + width].iter().sum::<f64>())
                    / (width as f64).sqrt();
                close(y[count + k], expected);
            }
            count *= 2;
        }
    }
}
#[test]
fn reconstruction_energy() {
    for n in [1, 2, 4, 8, 64, 1024] {
        let x: Vec<_> = (0..n).map(|i| (i % 31) as f64 - 15.0).collect();
        let y = transform(&x, false).unwrap();
        let z = transform(&y, true).unwrap();
        for (&a, &b) in x.iter().zip(&z) {
            close(a, b);
        }
        close(x.iter().map(|v| v * v).sum(), y.iter().map(|v| v * v).sum());
    }
}
#[test]
fn sign_layout() {
    let y = transform(&[1.0, 0.0], false).unwrap();
    close(y[1], std::f64::consts::FRAC_1_SQRT_2);
    let y = transform(&[0.0, 1.0], false).unwrap();
    close(y[1], -std::f64::consts::FRAC_1_SQRT_2);
    assert_eq!(transform(&[7.0], true), Ok(vec![7.0]));
    assert!(
        transform(&vec![0.0; 65536], false)
            .unwrap()
            .iter()
            .all(|&v| v == 0.0)
    );
}
#[test]
fn rejected_domains() {
    for n in [0, 3, 65537] {
        assert_eq!(transform(&vec![0.0; n], false), Err(Error::InvalidInput));
    }
    for x in [f64::NAN, f64::INFINITY, 1.01e50] {
        assert_eq!(transform(&[x], false), Err(Error::InvalidInput));
    }
}
