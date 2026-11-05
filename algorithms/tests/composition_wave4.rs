//! Exact bounded client joins; independent leaf oracles remain necessary.
use mova_algorithms::*;

#[test]
fn binary_weight_distances_embed_dijkstra() {
    for n in 1..=3 {
        for mask in 0..(1usize << (n * n)) {
            let graph: Vec<Vec<_>> = (0..n)
                .map(|u| {
                    (0..n)
                        .filter(|&v| mask & (1 << (u * n + v)) != 0)
                        .map(|v| (v, ((u + v) % 2) as u8))
                        .collect()
                })
                .collect();
            let widened: Vec<Vec<_>> = graph
                .iter()
                .map(|es| es.iter().map(|&(v, w)| (v, u64::from(w))).collect())
                .collect();
            for source in 0..n {
                let result: Vec<_> = zero_one_bfs::zero_one_bfs(&graph, source)
                    .unwrap()
                    .into_iter()
                    .map(|d| d.map(|v| v as u128))
                    .collect();
                assert_eq!(result, dijkstra::dijkstra(&widened, source).unwrap());
            }
        }
    }
}

#[test]
fn signed_dag_distances_agree_with_bellman_ford() {
    for n in 1..=4 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..(1usize << pairs.len()) {
            let mut graph = vec![vec![]; n];
            for (bit, &(u, v)) in pairs.iter().enumerate() {
                if mask & (1 << bit) != 0 {
                    graph[u].push((v, ((3 * u + v) % 7) as i64 - 3));
                }
            }
            for source in 0..n {
                assert_eq!(
                    dag_shortest_paths::shortest_paths(&graph, source).unwrap(),
                    bellman_ford::bellman_ford(&graph, source).unwrap()
                );
            }
        }
    }
    // The DAG API has a global cycle gate; Bellman-Ford ignores unreachable cycles.
    let graph = vec![vec![], vec![(1, 1)]];
    assert!(dag_shortest_paths::shortest_paths(&graph, 0).is_err());
    assert_eq!(
        bellman_ford::bellman_ford(&graph, 0).unwrap(),
        vec![Some(0), None]
    );
}

fn words(max_len: u32) -> Vec<Vec<u8>> {
    (0..=max_len)
        .flat_map(|n| {
            (0..(1usize << n)).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}

#[test]
fn multi_pattern_and_suffix_search_preserve_byte_occurrences() {
    let patterns: [&[u8]; 6] = [&[], &[0], &[0, 1], &[1, 1], &[0], &[1, 0, 1]];
    let automaton = aho_corasick::AhoCorasick::new(&patterns);
    for text in words(6) {
        let suffixes = suffix_array::suffix_array(&text);
        let mut expected: Vec<_> = patterns
            .iter()
            .enumerate()
            .flat_map(|(pattern, needle)| {
                kmp::find_all(&text, needle)
                    .into_iter()
                    .map(move |start| (start + needle.len(), pattern, start))
            })
            .collect();
        expected.sort_unstable();
        let got: Vec<_> = automaton
            .find_all(&text)
            .into_iter()
            .map(|m| (m.end, m.pattern, m.start))
            .collect();
        assert_eq!(got, expected);
        for needle in patterns.iter().filter(|p| !p.is_empty()) {
            let mut starts: Vec<_> = suffixes
                .iter()
                .copied()
                .filter(|&i| text[i..].starts_with(needle))
                .collect();
            starts.sort_unstable();
            assert_eq!(starts, kmp::find_all(&text, needle));
        }
        assert_eq!(kmp::find_all(&text, &[]).len(), suffixes.len() + 1);
    }
}

#[test]
fn common_subsequence_bounds_edit_cost_and_sorted_intersection() {
    let inputs = words(4);
    for a in &inputs {
        for b in &inputs {
            let l = longest_common_subsequence::length(a, b);
            let d = levenshtein::distance(a, b);
            assert_eq!(l, longest_common_subsequence::length(b, a));
            assert!(a.len().max(b.len()) - l <= d);
            assert!(d <= a.len() + b.len() - 2 * l);
            let mut sa = a.clone();
            let mut sb = b.clone();
            merge_sort::merge_sort(&mut sa);
            merge_sort::merge_sort(&mut sb);
            sa.dedup();
            sb.dedup();
            let intersection = sa.iter().filter(|x| sb.contains(x)).count();
            assert_eq!(longest_common_subsequence::length(&sa, &sb), intersection);
        }
    }
    assert_eq!(longest_common_subsequence::length(b"a", b"b"), 0);
    assert_eq!(levenshtein::distance(b"a", b"b"), 1); // Strictly below delete+insert.
}

#[test]
fn scheduling_restricted_families_match_knapsack_capacities() {
    for n in 0..=6u32 {
        for mut code in 0..3usize.pow(n) {
            let weights: Vec<u64> = (0..n)
                .map(|_| {
                    let w = (code % 3) as u64;
                    code /= 3;
                    w
                })
                .collect();
            let sack: Vec<_> = weights
                .iter()
                .map(|&value| knapsack::Item { weight: 1, value })
                .collect();
            for disjoint in [false, true] {
                let intervals: Vec<_> = weights
                    .iter()
                    .enumerate()
                    .map(|(i, &weight)| {
                        let start = if disjoint { i as i64 } else { 0 };
                        weighted_interval_scheduling::Item {
                            start,
                            end: start + 1,
                            weight,
                        }
                    })
                    .collect();
                let selected = weighted_interval_scheduling::select(&intervals).unwrap();
                let capacity = if disjoint { weights.len() } else { 1 };
                assert_eq!(
                    selected.total_weight,
                    knapsack::max_value(&sack, capacity).unwrap()
                );
                let mut seen = std::collections::BTreeSet::new();
                let mut total = 0u128;
                for &i in &selected.indices {
                    assert!(i < intervals.len() && seen.insert(i));
                    total += u128::from(intervals[i].weight);
                }
                assert_eq!(total, selected.total_weight);
                assert!(
                    selected
                        .indices
                        .windows(2)
                        .all(|p| intervals[p[0]].end <= intervals[p[1]].start)
                );
            }
        }
    }
}

#[test]
fn hull_projection_loses_nearest_pair_but_translation_preserves_both() {
    use convex_hull::Point;
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 10, y: 0 },
        Point { x: 10, y: 10 },
        Point { x: 0, y: 10 },
        Point { x: 5, y: 5 },
        Point { x: 5, y: 6 },
    ];
    let hull = convex_hull::convex_hull(&points);
    let full = closest_pair::closest_pair(&points).unwrap();
    let boundary = closest_pair::closest_pair(&hull).unwrap();
    assert_eq!(full.indices, (4, 5));
    assert_eq!(full.distance_squared, 1);
    assert_eq!(boundary.distance_squared, 100);
    assert_ne!(full.distance_squared, boundary.distance_squared);
    for dx in -3..=3 {
        for dy in -3..=3 {
            let moved: Vec<_> = points
                .iter()
                .map(|p| Point {
                    x: p.x + dx,
                    y: p.y + dy,
                })
                .collect();
            let shifted_hull: Vec<_> = hull
                .iter()
                .map(|p| Point {
                    x: p.x + dx,
                    y: p.y + dy,
                })
                .collect();
            assert_eq!(convex_hull::convex_hull(&moved), shifted_hull);
            let nearest = closest_pair::closest_pair(&moved).unwrap();
            assert_eq!(nearest.indices, full.indices);
            assert_eq!(nearest.distance_squared, full.distance_squared);
        }
    }
}

#[test]
fn prime_power_residues_reconstruct_exact_integer_classes() {
    let primes = sieve::sieve(256).unwrap();
    for n in 1..=256u64 {
        let factors = prime_factorization::factorize(n).unwrap();
        let powers: Vec<_> = factors
            .iter()
            .map(|&(p, e)| {
                assert!(primes.contains(&(p as usize)));
                p.checked_pow(e).unwrap()
            })
            .collect();
        assert_eq!(powers.iter().product::<u64>(), n);
        for (i, &q) in powers.iter().enumerate() {
            for &r in &powers[i + 1..] {
                assert_eq!(gcd::gcd(q, r), 1);
            }
        }
        for x in 0..=n {
            let input: Vec<_> = powers
                .iter()
                .map(|&modulus| chinese_remainder::Congruence {
                    residue: x % modulus,
                    modulus,
                })
                .collect();
            let restored = chinese_remainder::solve(&input).unwrap().unwrap();
            assert_eq!(restored.modulus, n);
            assert_eq!(restored.residue, x % n);
        }
    }
    let input = [
        chinese_remainder::Congruence {
            residue: 35 % 6,
            modulus: 6,
        },
        chinese_remainder::Congruence {
            residue: 35 % 10,
            modulus: 10,
        },
    ];
    let restored = chinese_remainder::solve(&input).unwrap().unwrap();
    assert_eq!((restored.residue, restored.modulus), (5, 30));
}
