use super::*;
fn oracle(n: usize, g: usize, cost: impl Fn(usize, usize) -> u128) -> Option<u128> {
    let mut d = vec![vec![None; n + 1]; g + 1];
    d[0][0] = Some(0);
    for layer in 1..=g {
        for j in 1..=n {
            d[layer][j] = (0..j)
                .filter_map(|k| d[layer - 1][k].map(|v| v + cost(k, j)))
                .min();
        }
    }
    d[g][n]
}
#[test]
fn exhaustive_monge_costs() {
    for code in 0..729 {
        let mut x = code;
        let mut prefix = vec![0u128];
        for _ in 0..6 {
            prefix.push(prefix.last().unwrap() + x % 3);
            x /= 3;
        }
        for layers in 0..=7 {
            let cost = |i: usize, j: usize| (prefix[j] - prefix[i]).pow(2);
            assert_eq!(
                partition(6, layers, 10000, cost),
                Ok(oracle(6, layers, cost))
            );
        }
    }
}
#[test]
fn budgets_overflow_and_empty() {
    assert_eq!(partition(0, 0, 0, |_, _| panic!()), Ok(Some(0)));
    assert_eq!(partition(0, 1, 0, |_, _| panic!()), Ok(None));
    assert_eq!(partition(3, 2, 0, |_, _| 0), Err(Error::BudgetExceeded));
    assert_eq!(
        partition(2, 2, 10, |_, _| u128::MAX),
        Err(Error::ArithmeticOverflow)
    );
    assert_eq!(
        partition(8, 3, 1000, |i, j| ((j - i) * (j - i)) as u128),
        Ok(Some(22))
    );
}
