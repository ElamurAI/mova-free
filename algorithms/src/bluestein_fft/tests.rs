use super::*;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-9 * (1.0 + b.abs()), "{a} != {b}");
}
#[test]
fn independent_dft() {
    for n in 1..=35 {
        let x: Vec<_> = (0..n)
            .map(|i| Complex {
                re: (i * 7 % 13) as f64 - 6.0,
                im: (i % 3) as f64,
            })
            .collect();
        for inv in [false, true] {
            let y = transform(&x, inv).unwrap();
            for (k, z) in y.iter().enumerate() {
                let mut re = 0.0;
                let mut im = 0.0;
                for (j, a) in x.iter().enumerate() {
                    let theta =
                        (if inv { 2.0 } else { -2.0 }) * std::f64::consts::PI * (j * k) as f64
                            / n as f64;
                    re += a.re * theta.cos() - a.im * theta.sin();
                    im += a.re * theta.sin() + a.im * theta.cos();
                }
                let scale = if inv { n as f64 } else { 1.0 };
                close(z.re, re / scale);
                close(z.im, im / scale);
            }
        }
    }
}
#[test]
fn roundtrip() {
    for n in [1, 2, 3, 7, 17, 31, 128, 257] {
        let x: Vec<_> = (0..n)
            .map(|i| Complex {
                re: (i % 11) as f64,
                im: -(i as f64) / 13.0,
            })
            .collect();
        let z = transform(&transform(&x, false).unwrap(), true).unwrap();
        for (a, b) in x.iter().zip(z) {
            close(a.re, b.re);
            close(a.im, b.im);
        }
    }
}
#[test]
fn impulse_boundary() {
    let mut x = vec![Complex { re: 0.0, im: 0.0 }; 16384];
    x[0].re = 1.0;
    for z in transform(&x, false).unwrap() {
        close(z.re, 1.0);
        close(z.im, 0.0);
    }
}
#[test]
fn rejected_domains() {
    assert_eq!(transform(&[], false), Err(Error::InvalidInput));
    assert_eq!(
        transform(&vec![Complex { re: 0.0, im: 0.0 }; 16385], false),
        Err(Error::InvalidInput)
    );
    for v in [f64::NAN, f64::INFINITY, 1.01e50] {
        assert_eq!(
            transform(&[Complex { re: v, im: 0.0 }], false),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            transform(&[Complex { re: 0.0, im: v }], true),
            Err(Error::InvalidInput)
        );
    }
}
