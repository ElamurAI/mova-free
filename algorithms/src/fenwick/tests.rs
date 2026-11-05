use super::*;
fn direct(values: &[i64], start: usize, end: usize) -> i128 {
    values[start..end].iter().map(|&x| i128::from(x)).sum()
}
fn check_all(tree: &Fenwick, values: &[i64]) {
    assert_eq!(tree.len(), values.len());
    for end in 0..=values.len() {
        assert_eq!(
            tree.prefix_sum(end),
            Ok(direct(values, 0, end)),
            "prefix end={end}"
        );
        for start in 0..=end {
            assert_eq!(
                tree.range_sum(start, end),
                Ok(direct(values, start, end)),
                "range={start}..{end}"
            );
        }
    }
}
#[test]
fn every_interval_matches_direct_slice() {
    for n in 0..=33 {
        let values: Vec<_> = (0..n).map(|i| (i as i64 * 17) % 23 - 11).collect();
        check_all(&Fenwick::from_slice(&values), &values);
    }
}
#[test]
fn deterministic_updates_preserve_exact_sums() {
    for n in [1, 7, 32, 65] {
        let mut values = vec![0; n];
        let mut tree = Fenwick::from_slice(&values);
        let mut seed = 0x5eed_u64;
        for step in 0..500 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let index = (seed >> 32) as usize % n;
            let delta = (seed % 201) as i64 - 100;
            values[index] += delta;
            assert_eq!(tree.add(index, delta), Ok(()), "n={n} step={step}");
            assert_eq!(tree.value(index), Some(values[index]), "n={n} step={step}");
            let end = (seed >> 16) as usize % (n + 1);
            let start = end / 2;
            assert_eq!(
                tree.prefix_sum(end),
                Ok(direct(&values, 0, end)),
                "n={n} step={step}"
            );
            assert_eq!(
                tree.range_sum(start, end),
                Ok(direct(&values, start, end)),
                "n={n} step={step}"
            );
        }
        check_all(&tree, &values);
    }
}
#[test]
fn full_width_points_and_overflow_are_atomic() {
    let mut tree = Fenwick::from_slice(&[i64::MAX, i64::MAX, i64::MIN, i64::MIN, 0]);
    assert_eq!(tree.prefix_sum(2), Ok(2 * i128::from(i64::MAX)));
    assert_eq!(tree.range_sum(2, 4), Ok(2 * i128::from(i64::MIN)));
    for (index, delta) in [(0, 1), (2, -1)] {
        let before = tree.clone();
        assert_eq!(
            tree.add(index, delta),
            Err(Error::PointOverflow {
                index,
                value: before.values[index],
                delta
            })
        );
        assert_eq!(tree, before);
    }
    tree.add(4, i64::MAX).unwrap();
    tree.add(4, -i64::MAX).unwrap();
    tree.add(4, i64::MIN).unwrap();
    check_all(&tree, &[i64::MAX, i64::MAX, i64::MIN, i64::MIN, i64::MIN]);
}
#[test]
fn empty_invalid_and_reversed_bounds() {
    let mut empty = Fenwick::from_slice(&[]);
    assert!(empty.is_empty());
    assert_eq!(empty.prefix_sum(0), Ok(0));
    assert_eq!(empty.range_sum(0, 0), Ok(0));
    assert_eq!(
        empty.add(0, 1),
        Err(Error::InvalidIndex { index: 0, len: 0 })
    );
    let mut tree = Fenwick::from_slice(&[3, -8, 11]);
    for index in [3, usize::MAX] {
        let before = tree.clone();
        assert_eq!(
            tree.add(index, i64::MAX),
            Err(Error::InvalidIndex { index, len: 3 })
        );
        assert_eq!(tree, before);
        assert_eq!(tree.value(index), None);
    }
    for (start, end) in [(2, 1), (0, 4), (usize::MAX, usize::MAX)] {
        assert_eq!(
            tree.range_sum(start, end),
            Err(Error::InvalidRange { start, end, len: 3 })
        );
    }
    assert_eq!(
        tree.prefix_sum(usize::MAX),
        Err(Error::InvalidRange {
            start: 0,
            end: usize::MAX,
            len: 3
        })
    );
}
