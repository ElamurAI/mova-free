use super::*;
#[test]
fn polynomial_oracle() {
    for degree in 0..6 {
        let nodes: Vec<_> = (0..7)
            .map(|i| {
                let x = -1.0 + i as f64 / 3.0;
                (x, x.powi(degree))
            })
            .collect();
        for i in 0..41 {
            let x = -1.0 + i as f64 / 20.0;
            assert!(
                (interpolate(&nodes, x).unwrap() - x.powi(degree)).abs() < 1e-12,
                "polynomial oracle"
            );
        }
        for &(x, y) in &nodes {
            assert_eq!(interpolate(&nodes, x), Ok(y));
        }
    }
}
#[test]
fn validation_and_fibers() {
    assert_eq!(interpolate(&[], f64::NAN), Err(Error::Size));
    assert_eq!(interpolate(&[(0.0, 1.0)], f64::NAN), Err(Error::NonFinite));
    assert_eq!(
        interpolate(&[(0.0, 1.0), (0.0, 2.0)], 0.0),
        Err(Error::Spacing)
    );
    assert_eq!(interpolate(&[(0.0, 1.0)], 1.0), Err(Error::Domain));
    assert_eq!(
        interpolate(&[(-1.0, 2.0), (1.0, 4.0)], 0.0),
        interpolate(&[(-1.0, 1.0), (1.0, 5.0)], 0.0)
    );
    let crowded: Vec<_> = (0..64).map(|i| (i as f64 * 1.1e-8, 1.0)).collect();
    assert_eq!(interpolate(&crowded, 0.5e-8), Err(Error::Arithmetic));
}
