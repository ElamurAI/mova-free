use super::*;
// Enumerate every bipartition of the symbol set: all labeled full binary trees.
fn optimal(w: &[u64], mask: usize, memo: &mut [Option<u128>]) -> u128 {
    if mask.count_ones() <= 1 {
        return 0;
    }
    if let Some(v) = memo[mask] {
        return v;
    }
    let total: u128 = w
        .iter()
        .enumerate()
        .filter(|(i, _)| mask & (1 << i) != 0)
        .map(|(_, v)| u128::from(*v))
        .sum();
    let anchor = mask.isolate_lowest_one();
    let mut best = u128::MAX;
    let mut part = (mask - 1) & mask;
    while part != 0 {
        if part & anchor != 0 {
            best = best.min(optimal(w, part, memo) + optimal(w, mask ^ part, memo) + total);
        }
        part = (part - 1) & mask;
    }
    memo[mask] = Some(best);
    best
}
fn check(w: &[u64]) {
    let c = codes(w);
    assert_eq!(c.len(), w.len());
    for (i, a) in c.iter().enumerate() {
        assert!(!a.is_empty());
        for (j, b) in c.iter().enumerate() {
            if i != j {
                assert!(!b.starts_with(a));
            }
        }
    }
    let actual: u128 = w
        .iter()
        .zip(&c)
        .map(|(v, c)| u128::from(*v) * c.len() as u128)
        .sum();
    let expected = if w.len() == 1 {
        u128::from(w[0])
    } else {
        optimal(w, (1 << w.len()) - 1, &mut vec![None; 1 << w.len()])
    };
    assert_eq!(actual, expected, "{w:?}");
    assert_eq!(c, codes(w));
}
#[test]
fn exhaustive_tree_optimality() {
    for n in 0..=6 {
        for mut word in 0..3usize.pow(n) {
            let w: Vec<_> = (0..n)
                .map(|_| {
                    let v = (word % 3) as u64;
                    word /= 3;
                    v
                })
                .collect();
            check(&w);
        }
    }
}
#[test]
fn boundaries_ties_and_scaling() {
    assert!(codes(&[]).is_empty());
    assert_eq!(codes(&[0]), vec![vec![false]]);
    assert_eq!(
        codes(&[1, 1, 1]),
        vec![vec![true, false], vec![true, true], vec![false]]
    );
    assert_eq!(
        codes(&[0, 0, 0]),
        vec![vec![true, false], vec![true, true], vec![false]]
    );
    for w in [
        vec![u64::MAX; 6],
        vec![0, 1, 0, 100, 1],
        vec![5, 9, 12, 13, 16, 45],
    ] {
        check(&w);
    }
    assert_eq!(codes(&[1, 2, 3, 4]), codes(&[7, 14, 21, 28]));
    assert_eq!(codes(&[1, 1]), codes(&[5, 9]));
}
