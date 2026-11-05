use super::*;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-8 * (1.0 + b.abs()), "{a} != {b}");
}
#[test]
fn polynomial_reproduction() {
    for degree in 0_i32..=6 {
        let x: Vec<_> = (0..41)
            .map(|i| ((i as f64 - 20.0) / 20.0).powi(degree))
            .collect();
        let y = smooth(&x, 9, degree as usize).unwrap();
        for (&a, &b) in x.iter().zip(&y) {
            close(a, b);
        }
    }
}
#[test]
fn published_quadratic_weights() {
    let x = [2.0, -3.0, 7.0, 1.0, 9.0, -4.0, 8.0];
    let y = smooth(&x, 5, 2).unwrap();
    for i in 2..5 {
        close(
            y[i],
            (-3.0 * x[i - 2] + 12.0 * x[i - 1] + 17.0 * x[i] + 12.0 * x[i + 1] - 3.0 * x[i + 2])
                / 35.0,
        );
    }
}
#[test]
fn shifted_means_and_identity() {
    let x = [2.0, 8.0, -1.0, 4.0, 9.0];
    let y = smooth(&x, 3, 0).unwrap();
    for (a, b) in y.iter().zip([3.0, 3.0, 11.0 / 3.0, 4.0, 4.0]) {
        close(*a, b);
    }
    for (a, b) in smooth(&x, 1, 0).unwrap().iter().zip(x) {
        close(*a, b);
    }
    assert_eq!(smooth(&vec![0.0; 4096], 31, 6).unwrap(), vec![0.0; 4096]);
}
#[test]
fn rejected_domains() {
    for (n, w, d) in [
        (0, 1, 0),
        (3, 0, 0),
        (3, 2, 0),
        (3, 5, 1),
        (35, 33, 1),
        (9, 9, 7),
        (3, 3, 3),
        (4097, 3, 1),
    ] {
        assert_eq!(smooth(&vec![0.0; n], w, d), Err(Error::InvalidInput));
    }
    for v in [f64::NAN, f64::INFINITY, 1.01e50] {
        assert_eq!(smooth(&[v], 1, 0), Err(Error::InvalidInput));
    }
}
