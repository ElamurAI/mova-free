use super::*;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-8, "{a} != {b}");
}
#[test]
fn exact_recurrence() {
    let one = minimize_diagonal(&[1., 4.], &[1., 4.], &[0., 0.], 1e-14, 1).unwrap();
    close(one.x[0], 4.0 / 9.0);
    close(one.x[1], 16.0 / 9.0);
    let two = minimize_diagonal(&[1., 4.], &[1., 4.], &[0., 0.], 1e-14, 2).unwrap();
    close(two.x[0], 20.0 / 27.0);
    close(two.x[1], 16.0 / 27.0);
    assert_eq!(two.status, Status::BudgetExhausted);
    close(
        two.residual_norm,
        (two.x[0] - 1.0).hypot(4.0 * two.x[1] - 4.0),
    );
}
#[test]
fn closed_form_grid() {
    for d in [1., 2., 9.] {
        for b in [-3., 0., 5.] {
            for start in [-2., 0., 4.] {
                let r = minimize_diagonal(&[d, 4.], &[b, -2.], &[start, 1.], 1e-10, 1000).unwrap();
                assert_eq!(r.status, Status::Converged);
                close(r.x[0], b / d);
                close(r.x[1], -0.5);
                assert!(r.residual_norm <= 1e-10);
            }
        }
    }
}
#[test]
fn budget_ties_and_zero() {
    let r = minimize_diagonal(&[2.], &[2.], &[0.], 2.0, 0).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.iterations, 0);
    let r = minimize_diagonal(&[2.], &[2.], &[0.], 1.0, 0).unwrap();
    assert_eq!(r.status, Status::BudgetExhausted);
    assert_eq!(r.x, vec![0.]);
    let r = minimize_diagonal(&[2.], &[2.], &[1.], 1e-20, 0).unwrap();
    assert_eq!(r.status, Status::Converged);
    let r = minimize_diagonal(&[1.; 256], &[1.; 256], &[0.; 256], 1e-12, 2).unwrap();
    assert_eq!(r.x, vec![1.; 256]);
}
#[test]
fn validation_precedes_work() {
    for d in [f64::NAN, f64::INFINITY, 0., -1., 1e-7, 1e7] {
        assert_eq!(
            minimize_diagonal(&[d], &[0.], &[0.], 1.0, 0),
            Err(Error::InvalidInput)
        );
    }
    for tol in [0., -1., f64::NAN, f64::INFINITY] {
        assert_eq!(
            minimize_diagonal(&[1.], &[0.], &[0.], tol, 0),
            Err(Error::InvalidInput)
        );
    }
    assert_eq!(
        minimize_diagonal(&[], &[], &[], 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.], &[], &[0.], 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.], &[0.], &[f64::NAN], 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.], &[1e7], &[0.], 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.; 257], &[0.; 257], &[0.; 257], 1., 0),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        minimize_diagonal(&[1.], &[0.], &[0.], 1., 100001),
        Err(Error::InvalidInput)
    );
    for d in [1e-6, 1e6] {
        assert!(minimize_diagonal(&[d], &[0.], &[0.], 1., 0).is_ok());
    }
}
#[test]
fn residual_is_not_solution_error() {
    let r = minimize_diagonal(&[1e-6], &[1e-6], &[0.], 1e-5, 0).unwrap();
    assert_eq!(r.status, Status::Converged);
    assert_eq!(r.x[0], 0.); // Exact minimizer is 1 despite a small gradient.
}
