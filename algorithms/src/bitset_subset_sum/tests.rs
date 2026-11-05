use super::*;
#[test]
fn exhaustive() {
    for code in 0..625 {
        let mut x = code;
        let mut a = [0; 4];
        for v in &mut a {
            *v = x % 5;
            x /= 5;
        }
        for t in 0..20 {
            let oracle = (0..16).any(|mask| {
                (0..4)
                    .filter(|i| mask & (1 << i) != 0)
                    .map(|i| a[i])
                    .sum::<usize>()
                    == t
            });
            assert_eq!(reachable(&a, t, 1), Ok(oracle));
        }
    }
}
#[test]
fn word_edges_and_single_use() {
    assert_eq!(reachable(&[63, 1, 64, 65], 193, 4), Ok(true));
    assert_eq!(reachable(&[64], 128, 3), Ok(false));
    assert_eq!(reachable(&[1], 2, 1), Ok(false));
    assert_eq!(reachable(&[usize::MAX], 0, 1), Ok(true));
    assert_eq!(reachable(&[], 0, 0), Err(Error::BudgetExceeded));
    for t in 0..200 {
        let a = [63, 64, 65];
        let oracle = (0..8).any(|m| {
            (0..3)
                .filter(|i| m & (1 << i) != 0)
                .map(|i| a[i])
                .sum::<usize>()
                == t
        });
        assert_eq!(reachable(&a, t, 4), Ok(oracle));
    }
}
