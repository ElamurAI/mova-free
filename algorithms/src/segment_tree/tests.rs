use super::*;
fn oracle(values: &[i32], start: usize, end: usize) -> Option<usize> {
    if start >= end || end > values.len() {
        return None;
    }
    (start..end).min_by_key(|&i| (values[i], i))
}
fn check(tree: &SegmentTree<i32>, values: &[i32]) {
    assert_eq!(tree.len(), values.len());
    assert_eq!(tree.is_empty(), values.is_empty());
    let before = tree.clone();
    for start in 0..=values.len() + 1 {
        for end in 0..=values.len() + 1 {
            let expected = oracle(values, start, end);
            assert_eq!(
                tree.query_index(start, end),
                expected,
                "{values:?} {start}..{end}"
            );
            assert_eq!(tree.query(start, end), expected.map(|i| &values[i]));
        }
    }
    assert_eq!(tree.query_index(usize::MAX, usize::MAX), None);
    assert_eq!(tree.query_index(0, usize::MAX), None);
    assert_eq!(*tree, before);
}
#[test]
fn exhaustive_point_updates_and_ranges() {
    for n in 0..=6u32 {
        for mut code in 0..3usize.pow(n) {
            let values: Vec<i32> = (0..n)
                .map(|_| {
                    let x = (code % 3) as i32 - 1;
                    code /= 3;
                    x
                })
                .collect();
            let original = values.clone();
            let tree = SegmentTree::from_slice(&values);
            check(&tree, &values);
            for index in 0..values.len() {
                for value in -2..=2 {
                    let mut updated = tree.clone();
                    let mut expected = values.clone();
                    updated.update(index, value).unwrap();
                    expected[index] = value;
                    check(&updated, &expected);
                    updated.update(index, values[index]).unwrap();
                    assert_eq!(updated, tree);
                }
            }
            assert_eq!(values, original);
        }
    }
}
#[test]
fn invalid_updates_are_atomic() {
    for values in [vec![], vec![3], vec![5, 2, 2, 9, 1]] {
        let mut tree = SegmentTree::from_slice(&values);
        for index in [values.len(), values.len() + 1, usize::MAX] {
            let before = tree.clone();
            assert_eq!(
                tree.update(index, -99),
                Err(InvalidIndex {
                    index,
                    len: values.len()
                })
            );
            assert_eq!(tree, before);
        }
    }
}
#[test]
fn repeated_updates_move_and_restore_leftmost_ties() {
    let mut values = vec![4; 9];
    let mut tree = SegmentTree::from_slice(&values);
    for round in 0..7 {
        for i in 0..values.len() {
            values[i] = ((i + round) % 5) as i32 - 2;
            tree.update(i, values[i]).unwrap();
            check(&tree, &values);
        }
    }
    tree.update(4, i32::MIN).unwrap();
    assert_eq!(tree.query_index(0, 9), Some(4));
    tree.update(4, i32::MAX).unwrap();
    values[4] = i32::MAX;
    check(&tree, &values);
    assert_eq!(tree.value(4), Some(&i32::MAX));
    assert_eq!(tree.value(9), None);
}
#[test]
fn negative_controls_reject_wrong_boundary_and_stale_parent() {
    let mut tree = SegmentTree::from_slice(&[0, 9, 4, 2, 7]);
    assert_ne!(tree.query_index(1, 4), Some(0));
    tree.update(0, 10).unwrap();
    assert_eq!(tree.query_index(0, 5), Some(3));
    assert_ne!(tree.query_index(0, 5), Some(0));
}

#[test]
fn semilattice_decomposition_and_singleton_reconstruction() {
    for n in 1..=6u32 {
        for mut code in 0..3usize.pow(n) {
            let values: Vec<i32> = (0..n)
                .map(|_| {
                    let value = (code % 3) as i32 - 1;
                    code /= 3;
                    value
                })
                .collect();
            let tree = SegmentTree::from_slice(&values);
            let recovered: Vec<_> = (0..values.len())
                .map(|i| *tree.query(i, i + 1).unwrap())
                .collect();
            assert_eq!(recovered, values);
            for s in 0..values.len() {
                for e in s + 1..=values.len() {
                    for m in s..=e {
                        let combined = [tree.query_index(s, m), tree.query_index(m, e)]
                            .into_iter()
                            .flatten()
                            .min_by_key(|&i| (values[i], i));
                        assert_eq!(tree.query_index(s, e), combined);
                    }
                }
            }
        }
    }
    // Indices alone lose all singleton values.
    assert_eq!(
        SegmentTree::from_slice(&[9]).query_index(0, 1),
        SegmentTree::from_slice(&[-9]).query_index(0, 1)
    );
}

#[test]
fn disjoint_updates_commute_and_preserve_snapshot_ranges() {
    let values = [3, 1, 1, 8, -2, 4, 0];
    let snapshot = SegmentTree::from_slice(&values);
    for i in 0..values.len() {
        for j in 0..values.len() {
            let mut left = snapshot.clone();
            left.update(i, -5).unwrap();
            for s in 0..values.len() {
                for e in s + 1..=values.len() {
                    if !(s..e).contains(&i) {
                        assert_eq!(left.query_index(s, e), snapshot.query_index(s, e));
                        assert_eq!(left.query(s, e), snapshot.query(s, e));
                    }
                }
            }
            left.update(j, 6).unwrap();
            let mut right = snapshot.clone();
            right.update(j, 6).unwrap();
            right.update(i, -5).unwrap();
            if i != j {
                assert_eq!(left, right);
            } else {
                assert_ne!(left, right);
            }
            let before = left.clone();
            assert!(left.update(usize::MAX, 0).is_err());
            assert_eq!(left, before);
            assert_eq!(snapshot, SegmentTree::from_slice(&values));
        }
    }
}

#[test]
fn strict_order_embedding_preserves_argmin_but_collapse_does_not() {
    let values = [3, -2, -2, 4, 0, 1, -1];
    let mapped: Vec<_> = values.iter().map(|&x| 3 * x + 7).collect();
    let original = SegmentTree::from_slice(&values);
    let transformed = SegmentTree::from_slice(&mapped);
    for s in 0..values.len() {
        for e in s + 1..=values.len() {
            assert_eq!(original.query_index(s, e), transformed.query_index(s, e));
        }
    }
    assert_ne!(
        SegmentTree::from_slice(&[1, 0]).query_index(0, 2),
        SegmentTree::from_slice(&[0, 0]).query_index(0, 2)
    );
}
