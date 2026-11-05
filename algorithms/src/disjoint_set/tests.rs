use super::*;

// Independent oracle: a label per member; merging relabels the whole class.
fn merge(labels: &mut [usize], a: usize, b: usize) -> bool {
    let from = labels[b];
    let to = labels[a];
    if from == to {
        return false;
    }
    for label in labels {
        if *label == from {
            *label = to;
        }
    }
    true
}

fn agrees(set: &DisjointSet, labels: &[usize]) -> bool {
    if set.len() != labels.len() || set.size.len() != labels.len() {
        return false;
    }
    let mut roots = Vec::new();
    for i in 0..set.len() {
        let mut r = i;
        let mut steps = 0;
        while r < set.len() && set.parent[r] != r && steps <= set.len() {
            r = set.parent[r];
            steps += 1;
        }
        if r >= set.len() || steps > set.len() {
            return false;
        }
        roots.push(r);
    }
    let distinct = (0..labels.len())
        .filter(|&i| !labels[..i].contains(&labels[i]))
        .count();
    if set.components != distinct {
        return false;
    }
    for i in 0..set.len() {
        if roots[i] == i && set.size[i] != roots.iter().filter(|&&r| r == i).count() {
            return false;
        }
        for j in 0..set.len() {
            if (roots[i] == roots[j]) != (labels[i] == labels[j]) {
                return false;
            }
        }
    }
    true
}

fn sequences(set: DisjointSet, labels: Vec<usize>, depth: usize) {
    assert!(agrees(&set, &labels));
    if depth == 0 {
        return;
    }
    // Include every valid index and one out-of-range index in every operation.
    for a in 0..=set.len() {
        let mut next = set.clone();
        match next.find(a) {
            Ok(root) => assert_eq!(labels[a], labels[root]),
            Err(e) => {
                assert_eq!(
                    e,
                    IndexError {
                        index: a,
                        len: set.len()
                    }
                );
                assert_eq!(next, set);
            }
        }
        sequences(next, labels.clone(), depth - 1);
        for b in 0..=set.len() {
            let mut next = set.clone();
            let mut expected = labels.clone();
            let result = next.union(a, b);
            if a < set.len() && b < set.len() {
                assert_eq!(result, Ok(merge(&mut expected, a, b)));
            } else {
                assert_eq!(
                    result,
                    Err(IndexError {
                        index: if a >= set.len() { a } else { b },
                        len: set.len()
                    })
                );
                assert_eq!(next, set);
            }
            sequences(next, expected, depth - 1);
            let mut next = set.clone();
            let result = next.connected(a, b);
            if a < set.len() && b < set.len() {
                assert_eq!(result, Ok(labels[a] == labels[b]));
            } else {
                assert!(result.is_err());
                assert_eq!(next, set);
            }
            sequences(next, labels.clone(), depth - 1);
        }
    }
}

#[test]
fn all_small_operation_sequences_match_partition_oracle() {
    for n in 0..=3 {
        sequences(DisjointSet::new(n), (0..n).collect(), 3);
    }
}

#[test]
fn empty_and_repeated_unions() {
    let mut empty = DisjointSet::new(0);
    assert!(empty.is_empty());
    assert_eq!(empty.components(), 0);
    assert!(empty.find(usize::MAX).is_err());
    let mut set = DisjointSet::new(2);
    assert_eq!(set.union(1, 0), Ok(true));
    assert_eq!(set.union(0, 1), Ok(false));
    assert_eq!(set.union(1, 1), Ok(false));
    assert_eq!(set.components(), 1);
    assert_eq!(set.find(1), Ok(0));
}

fn depth_two() -> DisjointSet {
    let mut set = DisjointSet::new(5);
    set.union(0, 1).unwrap();
    set.union(2, 3).unwrap();
    set.union(0, 2).unwrap();
    assert_eq!(set.parent[3], 2);
    set
}

#[test]
fn invalid_second_argument_does_not_even_compress_first_path() {
    let set = depth_two();
    for (a, b) in [(3, 5), (5, 3), (usize::MAX, usize::MAX)] {
        let mut actual = set.clone();
        assert!(actual.union(a, b).is_err());
        assert_eq!(actual, set);
        assert!(actual.connected(a, b).is_err());
        assert_eq!(actual, set);
    }
}

#[test]
fn compression_preserves_partition_and_unrelated_storage() {
    let mut set = depth_two();
    let before = set.clone();
    assert_eq!(set.find(3), Ok(0));
    assert_eq!(set.parent[3], 0);
    assert_eq!(set.parent[4], before.parent[4]);
    assert_eq!(set.size, before.size);
    assert_eq!(set.components, before.components);
    assert!(agrees(&set, &[0, 0, 0, 0, 4]));
}

#[test]
fn equal_size_ties_and_unequal_size_priority() {
    let mut set = DisjointSet::new(4);
    set.union(3, 2).unwrap();
    assert_eq!(set.find(3), Ok(2));
    set.union(0, 2).unwrap();
    assert_eq!(set.find(0), Ok(2));
    assert_eq!(set.size[2], 3);
}

#[test]
fn reverse_partition_observation_retains_forest_ambiguity() {
    let a = depth_two();
    let mut b = a.clone();
    b.find(3).unwrap();
    assert_ne!(a.parent, b.parent);
    assert!(agrees(&a, &[0, 0, 0, 0, 4]));
    assert!(agrees(&b, &[0, 0, 0, 0, 4]));
}

#[test]
fn negative_control_rejects_wrong_merge_and_component_count() {
    let mut wrong_merge = DisjointSet::new(3);
    wrong_merge.union(0, 2).unwrap();
    assert!(!agrees(&wrong_merge, &[0, 0, 2]));
    let mut wrong_count = DisjointSet::new(3);
    wrong_count.union(0, 1).unwrap();
    wrong_count.components += 1;
    assert!(!agrees(&wrong_count, &[0, 0, 2]));
}

fn next_word(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

#[test]
fn long_seeded_mixed_traces_match_independent_partition() {
    for n in [4, 17, 64] {
        for seed in [1, 0x51a7, 0xdead_beef] {
            let mut rng = seed;
            let mut set = DisjointSet::new(n);
            let mut labels: Vec<_> = (0..n).collect();
            for step in 0..600 {
                let op = next_word(&mut rng) >> 32;
                let a = ((next_word(&mut rng) >> 32) as usize) % (n + 2);
                let b = ((next_word(&mut rng) >> 32) as usize) % (n + 2);
                let before = set.clone();
                let context = format!("n={n} seed={seed} step={step} op={} a={a} b={b}", op % 3);
                match op % 3 {
                    0 => {
                        let actual = set.union(a, b);
                        if a < n && b < n {
                            assert_eq!(actual, Ok(merge(&mut labels, a, b)), "{context}");
                        } else {
                            assert_eq!(
                                actual,
                                Err(IndexError {
                                    index: if a >= n { a } else { b },
                                    len: n
                                }),
                                "{context}"
                            );
                            assert_eq!(set, before, "{context}");
                        }
                    }
                    1 => {
                        let actual = set.connected(a, b);
                        if a < n && b < n {
                            assert_eq!(actual, Ok(labels[a] == labels[b]), "{context}");
                        } else {
                            assert_eq!(
                                actual,
                                Err(IndexError {
                                    index: if a >= n { a } else { b },
                                    len: n
                                }),
                                "{context}"
                            );
                            assert_eq!(set, before, "{context}");
                        }
                    }
                    _ => {
                        let actual = set.find(a);
                        if a < n {
                            let root = actual.unwrap();
                            assert!(root < n, "{context}: root out of range");
                            assert_eq!(labels[a], labels[root], "{context}");
                            assert_eq!(set.parent[root], root, "{context}: not a representative");
                        } else {
                            assert_eq!(actual, Err(IndexError { index: a, len: n }), "{context}");
                            assert_eq!(set, before, "{context}");
                        }
                    }
                }
                assert!(
                    agrees(&set, &labels),
                    "{context}: state disagrees with oracle"
                );
            }
        }
    }
}

#[test]
fn balanced_tree_compression_and_errors_preserve_full_state() {
    let n = 64;
    let mut set = DisjointSet::new(n);
    let mut labels: Vec<_> = (0..n).collect();
    for power in 0..6 {
        let stride = 1 << power;
        for a in (0..n).step_by(stride * 2) {
            set.union(a, a + stride).unwrap();
            merge(&mut labels, a, a + stride);
        }
    }
    assert!(agrees(&set, &labels));
    let mut path = Vec::new();
    let mut at = n - 1;
    while set.parent[at] != at {
        path.push(at);
        at = set.parent[at];
    }
    assert_eq!(path.len(), 6);
    let before = set.clone();
    assert_eq!(set.find(n - 1), Ok(0));
    for &node in &path {
        assert_eq!(set.parent[node], 0, "uncompressed node {node}");
    }
    assert_eq!(set.size, before.size);
    assert_eq!(set.components, before.components);
    assert!(agrees(&set, &labels));
    // Include uncompressed branches after compressing the deepest leaf.
    for valid in [31, 47, 63] {
        for bad in [n, usize::MAX] {
            for (a, b) in [(valid, bad), (bad, valid), (bad, n)] {
                let snapshot = set.clone();
                let expected = Err(IndexError {
                    index: if a >= n { a } else { b },
                    len: n,
                });
                assert_eq!(set.union(a, b), expected, "union({a},{b})");
                assert_eq!(set, snapshot, "union({a},{b}) changed forest");
                assert_eq!(set.connected(a, b), expected, "connected({a},{b})");
                assert_eq!(set, snapshot, "connected({a},{b}) changed forest");
                assert_eq!(set.find(bad), Err(IndexError { index: bad, len: n }));
                assert_eq!(set, snapshot, "find({bad}) changed forest");
            }
        }
    }
}

#[test]
fn union_order_and_argument_orientation_preserve_partition() {
    let n = 32;
    let edges: Vec<_> = (0..4)
        .flat_map(|group| (1..8).map(move |i| (group * 8 + i - 1, group * 8 + i)))
        .collect();
    let mut labels: Vec<_> = (0..n).collect();
    for &(a, b) in &edges {
        merge(&mut labels, a, b);
    }
    for seed in [9, 101, 12345] {
        let mut rng = seed;
        let mut permuted = edges.clone();
        for i in (1..permuted.len()).rev() {
            let j = (next_word(&mut rng) >> 32) as usize % (i + 1);
            permuted.swap(i, j);
        }
        let mut set = DisjointSet::new(n);
        for (step, &(a, b)) in permuted.iter().enumerate() {
            let (a, b) = if step % 2 == 0 { (b, a) } else { (a, b) };
            assert_eq!(set.union(a, b), Ok(true), "seed={seed} edge={step}");
        }
        assert!(agrees(&set, &labels), "seed={seed}");
        assert_eq!(set.components(), 4);
        for &(a, b) in &permuted {
            assert_eq!(set.union(b, a), Ok(false), "seed={seed} repeated ({b},{a})");
        }
        assert!(agrees(&set, &labels), "seed={seed} after repeated edges");
    }
}
