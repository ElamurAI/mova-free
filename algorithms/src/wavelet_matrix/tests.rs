use super::*;
fn check(values: &[u64]) {
    let matrix = WaveletMatrix::from_slice(values);
    assert_eq!(matrix.len(), values.len());
    assert_eq!(matrix.is_empty(), values.is_empty());
    for a in 0..=values.len() {
        for b in a..=values.len() {
            let mut sorted = values[a..b].to_vec();
            sorted.sort_unstable();
            for k in 0..=sorted.len() {
                assert_eq!(matrix.kth(a, b, k), sorted.get(k).copied());
            }
            for x in [0, 1, 2, 3, 1 << 63, u64::MAX] {
                assert_eq!(
                    matrix.rank(a, b, x),
                    Some(sorted.iter().filter(|&&v| v == x).count())
                );
                assert_eq!(
                    matrix.count_less(a, b, x),
                    Some(sorted.iter().filter(|&&v| v < x).count())
                );
                for y in [0, 1, 3, u64::MAX] {
                    assert_eq!(
                        matrix.range_count(a, b, x, y),
                        if x > y {
                            None
                        } else {
                            Some(sorted.iter().filter(|&&v| x <= v && v < y).count())
                        }
                    );
                }
            }
        }
    }
    for prefix in &matrix.levels {
        assert_eq!(prefix.len(), values.len() + 1);
        assert_eq!(prefix[0], 0);
        assert!(prefix.windows(2).all(|w| w[1] >= w[0] && w[1] - w[0] <= 1));
    }
}
#[test]
fn exhaustive_small_unsigned_oracle() {
    for n in 0..=5 {
        for mut code in 0..3usize.pow(n) {
            let mut values = Vec::new();
            for _ in 0..n {
                values.push((code % 3) as u64);
                code /= 3;
            }
            check(&values);
        }
    }
}
#[test]
fn extremes_duplicates_and_snapshot() {
    let mut values = vec![u64::MAX, 0, 1 << 63, u64::MAX, 1, 0];
    check(&values);
    let matrix = WaveletMatrix::from_slice(&values);
    values.fill(42);
    assert_eq!(matrix.kth(0, 6, 5), Some(u64::MAX));
    assert_eq!(matrix.rank(0, 6, 0), Some(2));
    check(&[7; 17]);
}
#[test]
fn invalid_ranges_and_k() {
    let matrix = WaveletMatrix::from_slice(&[2, 1]);
    for (a, b) in [(1, 0), (0, 3), (usize::MAX, usize::MAX)] {
        assert_eq!(matrix.rank(a, b, 1), None);
        assert_eq!(matrix.count_less(a, b, 1), None);
        assert_eq!(matrix.range_count(a, b, 0, 2), None);
        assert_eq!(matrix.kth(a, b, 0), None);
    }
    assert_eq!(matrix.kth(0, 2, usize::MAX), None);
    assert_eq!(matrix.rank(2, 2, 1), Some(0));
}
#[test]
fn kth_zero_branch_control() {
    assert_eq!(WaveletMatrix::from_slice(&[0, 1]).kth(0, 2, 1), Some(1));
}
