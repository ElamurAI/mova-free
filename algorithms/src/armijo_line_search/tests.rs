use super::*;
#[test]
fn backtracking_oracle() {
    for power in 0..8 {
        let start = 2.0_f64.powi(power);
        let r = search(|a| (1.0 - a).powi(2), 1.0, -2.0, start, 0.5, 0.25, 30).unwrap();
        let mut alpha = start;
        let mut k = 0;
        while (1.0 - alpha).powi(2) > 1.0 - 0.5 * alpha || (1.0 - alpha).powi(2) >= 1.0 {
            alpha *= 0.5;
            k += 1;
        }
        assert_eq!(r.step, alpha);
        assert_eq!(r.iterations, k);
        assert_eq!(r.status, Status::Converged);
        assert!(r.value <= 1.0 - 0.5 * r.step);
    }
}
#[test]
fn budget_and_counts() {
    let mut count = 0;
    let r = search(
        |_| {
            count += 1;
            2.0
        },
        1.0,
        -1.0,
        1.0,
        0.5,
        0.1,
        3,
    )
    .unwrap();
    assert_eq!(r.status, Status::BudgetExhausted);
    assert_eq!(count, 4);
    assert_eq!(count, r.evaluations);
}
#[test]
fn validation_and_nonfinite() {
    assert_eq!(
        search(|_| panic!(), 0.0, 1.0, 1.0, 0.5, 0.1, 10),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        search(|_| f64::INFINITY, 0.0, -1.0, 1.0, 0.5, 0.1, 10),
        Err(Error::NonFiniteEvaluation)
    );
    assert_eq!(
        search(|_| panic!(), 0.0, -f64::MAX, f64::MAX, 0.5, 0.5, 1),
        Err(Error::NonFiniteArithmetic)
    );
}
#[test]
fn rounded_non_decrease_stagnates() {
    let tiny = f64::from_bits(1);
    let r = search(|_| 1.0, 1.0, -1.0, tiny, 0.5, 0.5, 5).unwrap();
    assert_eq!(r.status, Status::Stagnated);
    assert_ne!(r.status, Status::Converged);
}
