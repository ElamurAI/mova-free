use super::*;
fn check(tree: &PersistentSegmentTree, snapshots: &[Vec<i64>]) {
    assert_eq!(tree.version_count(), snapshots.len());
    for (version, values) in snapshots.iter().enumerate() {
        for a in 0..=values.len() {
            for b in a..=values.len() {
                let sum: i128 = values[a..b].iter().map(|&v| i128::from(v)).sum();
                assert_eq!(tree.range_sum(version, a, b), Ok(sum));
            }
        }
    }
}
#[test]
fn branching_versions_against_vec_snapshots() {
    for n in 1..=17 {
        let original: Vec<i64> = (0..n).map(|i| i as i64 - 8).collect();
        let mut tree = PersistentSegmentTree::from_slice(&original).unwrap();
        let mut snapshots = vec![original];
        assert_eq!(tree.node_count(), 2 * n - 1);
        for step in 0..24 {
            let parent = (step * 7) % snapshots.len();
            let index = (step * 11) % n;
            let value = step as i64 - 12;
            let before = tree.nodes.clone();
            let mut next = snapshots[parent].clone();
            next[index] = value;
            let version = tree.set(parent, index, value).unwrap();
            snapshots.push(next);
            assert_eq!(version, snapshots.len() - 1);
            assert_eq!(&tree.nodes[..before.len()], before.as_slice());
            let height = usize::BITS - (n - 1).leading_zeros();
            assert!(tree.node_count() - before.len() <= height as usize + 1);
            check(&tree, &snapshots);
        }
    }
}
#[test]
fn invalid_operations_preserve_exact_state() {
    let mut tree = PersistentSegmentTree::from_slice(&[4, 5, 6]).unwrap();
    let nodes = tree.nodes.clone();
    let roots = tree.roots.clone();
    assert_eq!(tree.set(9, usize::MAX, 0), Err(Error::InvalidVersion));
    assert_eq!(tree.set(0, 3, 0), Err(Error::InvalidIndex));
    assert_eq!(tree.range_sum(9, 3, 0), Err(Error::InvalidVersion));
    assert_eq!(tree.range_sum(0, 2, 1), Err(Error::InvalidRange));
    assert_eq!(tree.range_sum(0, 0, usize::MAX), Err(Error::InvalidRange));
    assert_eq!(tree.nodes, nodes);
    assert_eq!(tree.roots, roots);
}
#[test]
fn wide_sums_cancellation_and_empty() {
    let values = vec![i64::MAX, i64::MAX, i64::MIN, i64::MIN, 1];
    let mut tree = PersistentSegmentTree::from_slice(&values).unwrap();
    check(&tree, std::slice::from_ref(&values));
    assert_eq!(tree.range_sum(0, 0, 2), Ok(2 * i128::from(i64::MAX)));
    let v = tree.set(0, 4, i64::MAX).unwrap();
    assert_eq!(tree.range_sum(v, 0, 5), Ok(i128::from(i64::MAX) - 2));
    assert_eq!(tree.range_sum(0, 0, 5), Ok(-1));
    let mut empty = PersistentSegmentTree::from_slice(&[]).unwrap();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.node_count(), 0);
    assert_eq!(empty.range_sum(0, 0, 0), Ok(0));
    assert_eq!(empty.set(0, 0, 1), Err(Error::InvalidIndex));
    assert_eq!(empty.version_count(), 1);
}
#[test]
fn replacement_control() {
    let mut tree = PersistentSegmentTree::from_slice(&[1, 2]).unwrap();
    let v = tree.set(0, 0, 9).unwrap();
    assert_eq!(tree.range_sum(v, 0, 2), Ok(11));
    assert_eq!(tree.range_sum(0, 0, 2), Ok(3));
}

#[test]
fn defensive_overflow_rolls_back_appended_nodes() {
    // Deliberately outside public-state invariant to reach the defensive guard.
    let mut tree = PersistentSegmentTree::from_slice(&[0, 0]).unwrap();
    let root = tree.roots[0].unwrap();
    let left = tree.nodes[root].left;
    tree.nodes[left].sum = i128::MAX;
    let nodes = tree.nodes.clone();
    let roots = tree.roots.clone();
    assert_eq!(tree.set(0, 1, 1), Err(Error::Overflow));
    assert_eq!(tree.nodes, nodes);
    assert_eq!(tree.roots, roots);
}
