use super::*;
#[test]
fn affine_oracle() {
    for n in 2..9 {
        let p: Vec<_> = (0..n)
            .map(|i| {
                let x = i as f64 / (n - 1) as f64;
                (x, 3.0 * x - 2.0)
            })
            .collect();
        for i in 0..101 {
            let x = i as f64 / 100.0;
            assert!(
                (interpolate(&p, x).unwrap() - (3.0 * x - 2.0)).abs() < 2e-14,
                "affine oracle"
            );
        }
    }
}
#[test]
fn shape_oracle() {
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                let p = [
                    (0.0, 0.0),
                    (0.1, a as f64),
                    (0.7, (a + b) as f64),
                    (1.0, (a + b + c) as f64),
                ];
                let mut previous = 0.0;
                for i in 0..201 {
                    let x = i as f64 / 200.0;
                    let y = interpolate(&p, x).unwrap();
                    assert!(
                        y >= previous - 1e-12 && y >= -1e-12 && y <= (a + b + c) as f64 + 1e-12,
                        "shape oracle"
                    );
                    previous = y;
                }
            }
        }
    }
    let p = [(0.0, 0.0), (0.3, 1.0), (1.0, 0.0)];
    for i in 0..101 {
        let y = interpolate(&p, i as f64 / 100.0).unwrap();
        assert!((-1e-12..=1.000000000001).contains(&y));
    }
}
#[test]
fn validation_and_knots() {
    assert_eq!(interpolate(&[], 0.0), Err(Error::Size));
    assert_eq!(
        interpolate(&[(0.0, 0.0), (1.0, f64::NAN)], 0.0),
        Err(Error::NonFinite)
    );
    assert_eq!(
        interpolate(&[(0.0, 0.0), (0.0, 1.0)], 0.0),
        Err(Error::Spacing)
    );
    assert_eq!(
        interpolate(&[(0.0, 0.0), (1.0, 1.0)], 2.0),
        Err(Error::Domain)
    );
    for &(x, y) in &[(0.0, 2.0), (0.3, -2.0), (1.0, 1.0)] {
        assert_eq!(
            interpolate(&[(0.0, 2.0), (0.3, -2.0), (1.0, 1.0)], x),
            Ok(y)
        );
    }
}
