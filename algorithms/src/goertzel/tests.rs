use super::*;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-8 * (1.0 + b.abs()), "{a} != {b}");
}
#[test]
fn independent_dft() {
    for n in 1..=55 {
        let x: Vec<_> = (0..n).map(|i| (i * 7 % 19) as f64 - 9.0).collect();
        let indices: Vec<_> = (0..n).rev().collect();
        for (&k, z) in indices.iter().zip(bins(&x, &indices).unwrap()) {
            let re = x
                .iter()
                .enumerate()
                .map(|(j, &a)| a * (2.0 * std::f64::consts::PI * (j * k) as f64 / n as f64).cos())
                .sum();
            let im = x
                .iter()
                .enumerate()
                .map(|(j, &a)| -a * (2.0 * std::f64::consts::PI * (j * k) as f64 / n as f64).sin())
                .sum();
            close(z.re, re);
            close(z.im, im);
        }
    }
}
#[test]
fn phase_and_duplicates() {
    let y = bins(&[0.0, 1.0, 0.0, 0.0], &[1, 0, 1, 2]).unwrap();
    close(y[0].re, 0.0);
    close(y[0].im, -1.0);
    assert_eq!(y[0], y[2]);
    close(y[1].re, 1.0);
    close(y[3].re, -1.0);
}
#[test]
fn dc_nyquist_limits() {
    let x = vec![1.0; 65536];
    let y = bins(&x, &[0, 32768]).unwrap();
    close(y[0].re, 65536.0);
    close(y[1].re, 0.0);
    assert_eq!(bins(&[], &[]), Ok(vec![]));
    assert_eq!(bins(&[3.0], &[]), Ok(vec![]));
    assert_eq!(bins(&[3.0], &vec![0; 65536]).unwrap().len(), 65536);
}
#[test]
fn rejected_domains() {
    assert_eq!(bins(&[], &[0]), Err(Error::InvalidInput));
    assert_eq!(bins(&[1.0], &[1]), Err(Error::InvalidInput));
    assert_eq!(bins(&vec![0.0; 65537], &[]), Err(Error::InvalidInput));
    assert_eq!(bins(&[0.0], &vec![0; 65537]), Err(Error::InvalidInput));
    for v in [f64::NAN, f64::INFINITY, 1.01e50] {
        assert_eq!(bins(&[v], &[]), Err(Error::InvalidInput));
    }
}
