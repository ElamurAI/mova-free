use super::*;
fn cubic(w: &[u64]) -> u128 {
    let n = w.len();
    let mut d = vec![vec![0; n + 1]; n + 1];
    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len;
            let sum = w[i..j].iter().map(|&v| v as u128).sum::<u128>();
            d[i][j] = (i..j).map(|r| d[i][r] + d[r + 1][j] + sum).min().unwrap();
        }
    }
    d[0][n]
}
#[test]
fn exhaustive() {
    for n in 0..=7 {
        for code in 0..3usize.pow(n as u32) {
            let mut x = code;
            let w = (0..n)
                .map(|_| {
                    let v = (x % 3) as u64;
                    x /= 3;
                    v
                })
                .collect::<Vec<_>>();
            assert_eq!(min_cost(&w, 64), Ok(cubic(&w)));
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(min_cost(&[], 0), Ok(0));
    assert_eq!(min_cost(&[9], 3), Err(Error::BudgetExceeded));
    assert_eq!(min_cost(&[9], 4), Ok(9));
    assert_eq!(min_cost(&[u64::MAX; 3], 16), Ok(5 * u64::MAX as u128));
    assert_eq!(min_cost(&[0; 12], 169), Ok(0));
    assert_eq!(min_cost(&[10, 1, 1], 16), Ok(15));
}
