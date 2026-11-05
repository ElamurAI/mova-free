use super::*;
#[test]
fn direct_oracle() {
    for n in [1, 2, 4, 8, 16, 32] {
        let a: Vec<_> = (0..n)
            .map(|i| Complex {
                re: (i * i % 7) as f64 - 3.0,
                im: (i % 3) as f64,
            })
            .collect();
        for inverse in [false, true] {
            let out = transform(&a, inverse).unwrap();
            for (k, z) in out.iter().enumerate() {
                let mut r = Complex { re: 0.0, im: 0.0 };
                for (j, x) in a.iter().enumerate() {
                    let t =
                        (if inverse { 2.0 } else { -2.0 }) * std::f64::consts::PI * (j * k) as f64
                            / n as f64;
                    let (s, c) = t.sin_cos();
                    r.re += x.re * c - x.im * s;
                    r.im += x.re * s + x.im * c;
                }
                if inverse {
                    r.re /= n as f64;
                    r.im /= n as f64;
                }
                assert!(
                    (z.re - r.re).abs() < 1e-10 && (z.im - r.im).abs() < 1e-10,
                    "DFT oracle"
                );
            }
        }
        let back = transform(&transform(&a, false).unwrap(), true).unwrap();
        for (x, y) in a.iter().zip(back) {
            assert!((x.re - y.re).abs() < 1e-12 && (x.im - y.im).abs() < 1e-12);
        }
    }
}
#[test]
fn boundaries() {
    let z = Complex { re: 0.0, im: 0.0 };
    assert_eq!(transform(&[], false), Err(Error::InvalidLength));
    assert_eq!(transform(&[z; 3], false), Err(Error::InvalidLength));
    assert_eq!(transform(&vec![z; 65537], false), Err(Error::TooLarge));
    assert_eq!(
        transform(
            &[Complex {
                re: f64::NAN,
                im: 0.0
            }],
            false
        ),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        transform(&[Complex { re: 1e101, im: 0.0 }], false),
        Err(Error::InvalidValue)
    );
    assert_eq!(transform(&vec![z; 65536], false).unwrap(), vec![z; 65536]);
}
