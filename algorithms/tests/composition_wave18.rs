//! Wave18 bounded public API composition fixtures; no universal correctness claim.
use mova_algorithms::{
    biconnected_components, bridges, dag_path_count, floor_sum, levenshtein, needleman_wunsch,
    persistent_segment_tree::PersistentSegmentTree, pollard_rho, run_length_encoding,
    segmented_sieve, two_way_matching, wavelet_matrix::WaveletMatrix,
};

#[test]
fn block_forest_join_path_counts() {
    let edges: Vec<_> = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (3, 4), (4, 4)]
        .into_iter()
        .map(|(u, v)| bridges::Edge { u, v })
        .collect();
    let blocks = biconnected_components::biconnected_components(6, &edges).unwrap();
    assert_eq!(blocks, vec![vec![0, 1, 2], vec![3], vec![4, 5], vec![6]]);
    let mut ids: Vec<_> = blocks.iter().flatten().copied().collect();
    ids.sort();
    assert_eq!(ids, (0..edges.len()).collect::<Vec<_>>());
    let singletons: Vec<_> = blocks
        .iter()
        .filter(|b| b.len() == 1 && edges[b[0]].u != edges[b[0]].v)
        .map(|b| b[0])
        .collect();
    assert_eq!(singletons, bridges::bridges(6, &edges).unwrap());
    // Actual block membership builds the incidence tree; orient from vertex 0.
    let mut incidence = vec![vec![]; 6 + blocks.len()];
    for (i, block) in blocks.iter().enumerate() {
        if block.len() == 1 && edges[block[0]].u == edges[block[0]].v {
            continue;
        }
        let vertices: std::collections::BTreeSet<_> = block
            .iter()
            .flat_map(|&e| [edges[e].u, edges[e].v])
            .collect();
        for v in vertices {
            incidence[v].push(6 + i);
            incidence[6 + i].push(v);
        }
    }
    let mut dag = vec![vec![]; incidence.len()];
    let mut seen = vec![false; incidence.len()];
    seen[0] = true;
    let mut stack = vec![0];
    while let Some(u) = stack.pop() {
        for &v in &incidence[u] {
            if !seen[v] {
                seen[v] = true;
                dag[u].push(v);
                stack.push(v);
            }
        }
    }
    assert_eq!(
        dag_path_count::dag_path_count(&dag, 0).unwrap(),
        vec![1, 1, 1, 1, 1, 0, 1, 1, 1, 0]
    );
    // Parallel directed arcs are distinct path choices, even though reachability is unchanged.
    assert_eq!(
        dag_path_count::dag_path_count(&[vec![1, 1], vec![2], vec![]], 0).unwrap(),
        vec![1, 2, 2]
    );
}

#[test]
fn run_decode_join_byte_search_and_edit_alignment() {
    let words: &[&[u8]] = &[b"", b"a", b"b", b"aaabaaa", b"abababa", &[0, 255, 0, 0]];
    for &a in words {
        let runs = run_length_encoding::encode(a);
        let decoded = run_length_encoding::decode(&runs, a.len()).unwrap();
        assert_eq!(decoded, a);
        for &b in words {
            let expected: Vec<_> = if b.is_empty() {
                (0..=a.len()).collect()
            } else if b.len() > a.len() {
                vec![]
            } else {
                (0..=a.len() - b.len())
                    .filter(|&i| &a[i..i + b.len()] == b)
                    .collect()
            };
            assert_eq!(two_way_matching::find_all(&decoded, b), expected);
            let scoring = needleman_wunsch::Scoring {
                match_score: 0,
                mismatch_score: -1,
                gap_score: -1,
            };
            let alignment = needleman_wunsch::align(&decoded, b, scoring, 1024).unwrap();
            assert_eq!(
                alignment.score,
                -(levenshtein::distance(a, b) as i64),
                "edit_distance_join_alignment_score"
            );
            assert_eq!(
                alignment
                    .columns
                    .iter()
                    .filter_map(|c| c.0)
                    .collect::<Vec<_>>(),
                a
            );
            assert_eq!(
                alignment
                    .columns
                    .iter()
                    .filter_map(|c| c.1)
                    .collect::<Vec<_>>(),
                b
            );
            let score: i64 = alignment
                .columns
                .iter()
                .map(|&(x, y)| match (x, y) {
                    (Some(x), Some(y)) => {
                        if x == y {
                            0
                        } else {
                            -1
                        }
                    }
                    (None, None) => panic!("empty column"),
                    _ => -1,
                })
                .sum();
            assert_eq!(score, alignment.score);
        }
    }
}

#[test]
fn segmented_primes_join_factorization_and_literal_divisibility() {
    let primes = segmented_sieve::segmented_sieve(0, 160).unwrap();
    let literal: Vec<_> = (2..=160u64)
        .filter(|&n| (2..n).all(|d| n % d != 0))
        .collect();
    assert_eq!(primes, literal);
    for n in 1..=160u64 {
        let factors = pollard_rho::factorize(n, 10_000).unwrap();
        assert!(factors.windows(2).all(|w| w[0] <= w[1]));
        assert!(factors.iter().all(|p| primes.contains(p)));
        assert_eq!(factors.iter().product::<u64>(), n);
        assert_eq!(factors.len() == 1, primes.contains(&n));
    }
}

#[test]
fn floor_sum_join_wavelet_threshold_counts() {
    for n in 0..=12u64 {
        for m in 1..=7u64 {
            for a in 0..=4u64 {
                for b in 0..=4u64 {
                    let values: Vec<_> = (0..n).map(|i| a * i + b).collect();
                    let wave = WaveletMatrix::from_slice(&values);
                    // Sum floor(v/m) by counting horizontal threshold crossings, independent of Euclidean reduction.
                    let count: u128 = (1..=60 / m)
                        .map(|k| {
                            (n as usize - wave.count_less(0, n as usize, k * m).unwrap()) as u128
                        })
                        .sum();
                    assert_eq!(floor_sum::floor_sum(n, m, a, b).unwrap(), count);
                }
            }
        }
    }
}

#[test]
fn persistent_versions_join_wavelet_order_statistics() {
    let mut snapshots = vec![vec![3i64, 1, 3, 0, 5]];
    let mut tree = PersistentSegmentTree::from_slice(&snapshots[0]).unwrap();
    for (base, index, value) in [(0, 1, 7), (0, 4, 0), (1, 0, 2), (2, 2, 9)] {
        let mut next = snapshots[base].clone();
        next[index] = value;
        assert_eq!(tree.set(base, index, value).unwrap(), snapshots.len());
        snapshots.push(next);
    }
    for (version, values) in snapshots.iter().enumerate() {
        let unsigned: Vec<_> = values.iter().map(|&v| v as u64).collect();
        let wave = WaveletMatrix::from_slice(&unsigned);
        for start in 0..=values.len() {
            for end in start..=values.len() {
                let mut sorted = unsigned[start..end].to_vec();
                sorted.sort();
                let mut joined = 0i128;
                for (k, &want) in sorted.iter().enumerate() {
                    let got = wave.kth(start, end, k).unwrap();
                    assert_eq!(got, want);
                    joined += got as i128;
                }
                assert_eq!(tree.range_sum(version, start, end).unwrap(), joined);
                assert_eq!(wave.kth(start, end, sorted.len()), None);
                for value in 0..=10 {
                    assert_eq!(
                        wave.rank(start, end, value).unwrap(),
                        sorted.iter().filter(|&&v| v == value).count()
                    );
                }
            }
        }
    }
    assert_eq!(tree.version_count(), snapshots.len());
}
