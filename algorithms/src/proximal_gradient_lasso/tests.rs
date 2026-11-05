use super::*;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-8, "{a} != {b}");
}
#[test]
fn exact_recurrence() {
    let a = vec![vec![1., 0.], vec![0., 2.]];
    let r = minimize(&a, &[2., -3.], 0.5, 1e-14, 1).unwrap();
    close(r.x[0], 0.3);
    close(r.x[1], -1.1);
    let r = minimize(&a, &[2., -3.], 0.5, 1e-14, 3).unwrap();
    let beta = 0.0;
    close(r.x[0], 0.732 + 0.192 * beta);
    close(r.x[1], -1.364 - 0.044 * beta);
    assert_eq!(r.status, Status::BudgetExhausted);
}
#[test]
fn independent_diagonal_oracle() {
    for y in [-3.0_f64, -0.5, 0., 0.5, 3.] {
        for lambda in [0., 0.5, 4.] {
            let a = vec![vec![1., 0.], vec![0., 2.]];
            let r = minimize(&a, &[y, y], lambda, 1e-10, 2000).unwrap();
            // Solve each signed scalar KKT equation independently.
            let scalar = |scale: f64| {
                let correlation = scale * y;
                if correlation > lambda {
                    (correlation - lambda) / (scale * scale)
                } else if correlation < -lambda {
                    (correlation + lambda) / (scale * scale)
                } else {
                    0.
                }
            };
            close(r.x[0], scalar(1.));
            close(r.x[1], scalar(2.));
            assert_eq!(r.status, Status::Converged);
            assert!(r.residual <= 1e-10);
            let value = 0.5 * ((r.x[0] - y).powi(2) + (2. * r.x[1] - y).powi(2))
                + lambda * (r.x[0].abs() + r.x[1].abs());
            close(r.value, value);
        }
    }
}
#[test]
fn correlated_and_nonunique() {
    let a = vec![vec![1., 0.], vec![1., 1.], vec![0., 1.]];
    let r = minimize(&a, &[1.25, -1., -2.25], 0.25, 1e-10, 3000).unwrap();
    assert_eq!(r.status, Status::Converged);
    close(r.x[0], 1.);
    close(r.x[1], -2.);
    close(r.value, 0.8125);
    let r = minimize(&[vec![1., 1.]], &[2.], 0.5, 1e-12, 20).unwrap();
    close(r.x[0] + r.x[1], 1.5);
    close(r.x[0], r.x[1]);
    let r = minimize(&[vec![0., 1.]], &[2.], 0.5, 1e-12, 20).unwrap();
    assert_eq!(r.x[0], 0.);
    close(r.x[1], 1.5);
}
#[test]
fn status_and_zero_design() {
    let r = minimize(&[vec![0., 0.]], &[3.], 0., 1e-12, 0).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.value, 4.5);
    let r = minimize(&[vec![1.]], &[2.], 0.5, 1.5, 0).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.iterations, 0);
    let r = minimize(&[vec![1.]], &[2.], 0.5, 1., 0).unwrap();
    assert_eq!(r.status, Status::BudgetExhausted);
    assert_eq!(r.x, vec![0.]);
    let r = minimize(&vec![vec![0.; 64]; 256], &[0.; 256], 0., 1., 0).unwrap();
    assert_eq!(r.status, Status::Converged);
}
#[test]
fn validation_precedes_work() {
    for a in [
        vec![],
        vec![vec![]],
        vec![vec![1.], vec![]],
        vec![vec![f64::NAN]],
        vec![vec![1e7]],
        vec![vec![1e-7]],
        vec![vec![1e-200]],
    ] {
        assert_eq!(minimize(&a, &[0.], 0., 1., 0), Err(Error::InvalidInput));
    }
    for lambda in [-1., f64::NAN, f64::INFINITY, 1e7] {
        assert_eq!(
            minimize(&[vec![1.]], &[0.], lambda, 1., 0),
            Err(Error::InvalidInput)
        );
    }
    for tol in [0., -1., f64::NAN, f64::INFINITY] {
        assert_eq!(
            minimize(&[vec![1.]], &[0.], 0., tol, 0),
            Err(Error::InvalidInput)
        );
    }
    assert_eq!(
        minimize(&[vec![1.]], &[], 0., 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(&[vec![1.]], &[f64::NAN], 0., 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(&[vec![1.]], &[0.], 0., 1., 100001),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(&[vec![0.; 65]], &[0.], 0., 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize(&vec![vec![0.]; 257], &[0.; 257], 0., 1., 0),
        Err(Error::InvalidInput)
    );
    assert!(minimize(&[vec![1e-6]], &[0.], 0., 1., 0).is_ok());
}
