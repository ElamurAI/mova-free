use super::*;
fn trial(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut d = 2;
    while d <= n / d {
        while n.is_multiple_of(d) {
            result.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        result.push(n);
    }
    result
}
#[test]
fn exhaustive_trial_oracle() {
    for n in 1..5000 {
        assert_eq!(factorize(n, 10000), Ok(trial(n)), "{n}");
    }
}
#[test]
fn boundaries_and_budget() {
    assert_eq!(factorize(0, MAX_STEPS + 1), Err(FactorError::Zero));
    assert_eq!(factorize(1, MAX_STEPS + 1), Err(FactorError::BudgetLimit));
    assert_eq!(factorize(1, 0), Ok(vec![]));
    assert_eq!(factorize(2, 0), Ok(vec![2]));
    assert_eq!(factorize(1 << 63, 0), Ok(vec![2; 63]));
    assert_eq!(factorize(9, 0), Err(FactorError::BudgetExhausted));
    assert_eq!(factorize(25, 1), Err(FactorError::BudgetExhausted));
    assert_eq!(factorize(9, 100), Ok(vec![3, 3]));
}
#[test]
fn full_width_and_powers() {
    for n in [u64::MAX, 3u64.pow(40), 1_000_000_007 * 1_000_000_009] {
        let fs = factorize(n, 1_000_000).unwrap();
        assert!(fs.iter().all(|&p| crate::miller_rabin::is_prime(p)));
        assert_eq!(
            fs.iter().map(|&p| u128::from(p)).product::<u128>(),
            u128::from(n)
        );
    }
    assert_eq!(
        factorize(u64::MAX, 1_000_000),
        Ok(vec![3, 5, 17, 257, 641, 65537, 6700417])
    );
    assert_eq!(
        factorize(18_446_744_073_709_551_557, 0),
        Ok(vec![18_446_744_073_709_551_557])
    );
}
#[test]
fn deterministic_budget_extension() {
    for n in [9, 25, 49, 91, 8051, 10403, 99991 * 97] {
        let expected = factorize(n, 10000).unwrap();
        assert_eq!(expected, trial(n));
        assert_eq!(factorize(n, 20000), Ok(expected.clone()));
        assert_eq!(factorize(n, 10000), Ok(expected));
    }
}
#[test]
fn independent_modular_step() {
    for n in 3..30 {
        for x in 0..n {
            for c in 1..10 {
                let mut product = 0;
                for _ in 0..x {
                    product = (product + x) % n;
                }
                assert_eq!(step(x, c, n), (product + c) % n);
            }
        }
    }
    assert_eq!(step(u64::MAX - 1, 1, u64::MAX), 2);
}
