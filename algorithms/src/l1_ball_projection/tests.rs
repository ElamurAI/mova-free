use super::*;
#[test]
fn kkt_grid() {
    for code in 0..625 {
        let mut c = code;
        let a: Vec<f64> = (0..4)
            .map(|_| {
                let x = (c % 5) as f64 - 2.0;
                c /= 5;
                x
            })
            .collect();
        for r in [0.0, 0.5, 2.0, 10.0] {
            let y = project_l1_ball(&a, r).unwrap();
            let norm = y.iter().map(|x| x.abs()).sum::<f64>();
            assert!(norm <= r + 1e-12, "ball feasibility");
            if a.iter().map(|x| x.abs()).sum::<f64>() <= r {
                assert_eq!(a, y);
                continue;
            }
            if r == 0.0 {
                assert!(y.iter().all(|x| *x == 0.0));
                continue;
            }
            let j = y.iter().position(|x| *x != 0.0).unwrap();
            let lambda = a[j].abs() - y[j].abs();
            assert!(lambda >= 0.0);
            for (&x, &z) in a.iter().zip(&y) {
                if z != 0.0 {
                    assert!(
                        x * z > 0.0 && ((x - z) - lambda * z.signum()).abs() < 1e-12,
                        "KKT stationarity"
                    );
                } else {
                    assert!(x.abs() <= lambda + 1e-12, "KKT zero");
                }
            }
            assert!((norm - r).abs() < 1e-12);
        }
    }
}
#[test]
fn boundaries_and_fibers() {
    assert_eq!(project_l1_ball(&[3.0, -1.0], 2.0), Ok(vec![2.0, -0.0]));
    assert_eq!(project_l1_ball(&[4.0, -1.0], 2.0), Ok(vec![2.0, -0.0]));
    assert_eq!(project_l1_ball(&[], 3.0), Ok(vec![]));
    assert_eq!(
        project_l1_ball(&[f64::MAX, f64::MAX], 1.0),
        Err(ProjectionError::Arithmetic)
    );
    assert_eq!(project_l1_ball(&[f64::MAX], 0.0), Ok(vec![0.0]));
    assert_eq!(
        project_l1_ball(&[f64::NAN], 0.0),
        Err(ProjectionError::NonFinite)
    );
    assert_eq!(
        project_l1_ball(&[1.0], -1.0),
        Err(ProjectionError::NegativeRadius)
    );
    assert_eq!(
        project_l1_ball(&[1.0], f64::INFINITY),
        Err(ProjectionError::NonFinite)
    );
}
