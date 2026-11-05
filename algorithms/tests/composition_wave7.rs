//! Bounded public-API joins: certificates, units and adapters are checked together.
use mova_algorithms::{
    a_star, bfs, binomial_coefficient, boyer_moore_horspool, dijkstra, fenwick, interval_union,
    kmp, knapsack, levenshtein, longest_common_subsequence, longest_common_substring, max_flow,
    optimal_string_alignment, rabin_karp, stoer_wagner, subset_sum, suffix_automaton, two_sat,
};

fn words(max: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|len| {
            (0..1usize << len).map(move |bits| (0..len).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}

#[test]
fn astar_dijkstra_certified_paths() {
    // Every directed three-vertex topology, including loops; positive weights
    // make h(v)=1 except h(goal)=0 globally consistent.
    for mask in 0..512usize {
        let mut graph = vec![Vec::new(); 3];
        for (from, row) in graph.iter_mut().enumerate() {
            for to in 0..3 {
                if mask & (1 << (from * 3 + to)) != 0 {
                    row.push((to, 1 + ((from + to + mask) % 2) as u64));
                }
            }
        }
        for start in 0..3 {
            let exact = dijkstra::dijkstra(&graph, start).unwrap();
            for (goal, expected) in exact.iter().enumerate() {
                for base in [0, 1] {
                    let mut h = vec![base; 3];
                    h[goal] = 0;
                    let found = a_star::shortest_path(&graph, start, goal, &h).unwrap();
                    assert_eq!(found.as_ref().map(|p| p.distance), *expected);
                    if let Some(path) = found {
                        assert_eq!(path.vertices.first(), Some(&start));
                        assert_eq!(path.vertices.last(), Some(&goal));
                        assert!(path.vertices.len() <= graph.len());
                        let cost: u128 = path
                            .vertices
                            .windows(2)
                            .map(|pair| {
                                let edge = graph[pair[0]].iter().find(|e| e.0 == pair[1]).unwrap();
                                u128::from(edge.1)
                            })
                            .sum();
                        assert_eq!(cost, path.distance);
                    }
                }
            }
        }
    }
}

#[test]
fn two_sat_edge_inequality_matches_bfs_parity() {
    let pairs = [(0, 0), (0, 1), (0, 2), (1, 1), (1, 2), (2, 2)];
    for mask in 0..64usize {
        let mut graph = vec![Vec::new(); 3];
        let mut clauses = Vec::new();
        for (bit, &(u, v)) in pairs.iter().enumerate() {
            if mask & (1 << bit) != 0 {
                graph[u].push(v);
                graph[v].push(u);
                for positive in [true, false] {
                    clauses.push((
                        two_sat::Literal {
                            variable: u,
                            positive,
                        },
                        two_sat::Literal {
                            variable: v,
                            positive,
                        },
                    ));
                }
            }
        }
        let mut color = [None; 3];
        for root in 0..3 {
            if color[root].is_none() {
                for (v, d) in bfs::bfs(&graph, root).unwrap().iter().enumerate() {
                    if let Some(d) = d {
                        color[v] = Some(d % 2);
                    }
                }
            }
        }
        let bipartite = graph
            .iter()
            .enumerate()
            .all(|(u, row)| row.iter().all(|&v| color[u] != color[v]));
        let assignment = two_sat::solve(3, &clauses).unwrap();
        assert_eq!(assignment.is_some(), bipartite);
        if let Some(a) = assignment {
            for (left, right) in &clauses {
                assert!(
                    (a[left.variable] == left.positive) || (a[right.variable] == right.positive)
                );
            }
            for (u, row) in graph.iter().enumerate() {
                for &v in row {
                    assert_ne!(a[u], a[v]);
                }
            }
        }
    }
    assert_eq!(two_sat::solve(0, &[]).unwrap(), Some(vec![]));
}

#[test]
fn global_cut_matches_terminal_flows_and_certificate() {
    for code in 0..729usize {
        let mut matrix = vec![vec![0u64; 4]; 4];
        let mut digits = code;
        for (i, j) in [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)] {
            let weight = (digits % 3) as u64;
            digits /= 3;
            matrix[i][j] = weight;
            matrix[j][i] = weight;
        }
        let mut edges = Vec::new();
        for (from, row) in matrix.iter().enumerate() {
            for (to, &capacity) in row.iter().enumerate() {
                if from != to && capacity > 0 {
                    edges.push(max_flow::Edge { from, to, capacity });
                }
            }
        }
        let best = (1..4)
            .map(|sink| max_flow::max_flow(4, &edges, 0, sink).unwrap().value)
            .min()
            .unwrap();
        let cut = stoer_wagner::minimum_cut(&matrix).unwrap().unwrap();
        assert_eq!(cut.weight, best);
        assert!(cut.side.contains(&0) && cut.side.len() < 4);
        assert!(cut.side.windows(2).all(|p| p[0] < p[1]));
        let mut crossing = 0u128;
        for &u in &cut.side {
            for (v, &w) in matrix[u].iter().enumerate() {
                if !cut.side.contains(&v) {
                    crossing += u128::from(w);
                }
            }
        }
        assert_eq!(crossing, cut.weight);
    }
}

#[test]
fn verified_search_adapters_match_kmp() {
    for text in words(6) {
        for pattern in words(3) {
            let expected = kmp::find_all(&text, &pattern);
            assert_eq!(rabin_karp::find_all(&text, &pattern), expected);
            assert_eq!(boyer_moore_horspool::find_all(&text, &pattern), expected);
        }
    }
    let collision = [249, 27, 201, 69, 201, 27, 249, 1];
    assert_eq!(
        rabin_karp::find_all(&collision, &[0; 8]),
        kmp::find_all(&collision, &[0; 8])
    );
}

#[test]
fn substring_witnesses_join_automata_and_subsequence_bound() {
    let all = words(4);
    for a in &all {
        let sa = suffix_automaton::SuffixAutomaton::new(a);
        for b in &all {
            let sb = suffix_automaton::SuffixAutomaton::new(b);
            let w = longest_common_substring::longest_common_substring(a, b);
            let witness = &a[w.a_start..w.a_start + w.len];
            assert_eq!(witness, &b[w.b_start..w.b_start + w.len]);
            assert!(sa.contains(witness) && sb.contains(witness));
            assert!(w.len <= longest_common_subsequence::length(a, b));
            // Membership is only feasibility; exclude any longer common slice too.
            for i in 0..a.len() {
                for j in i + w.len + 1..=a.len() {
                    assert!(!sb.contains(&a[i..j]));
                }
            }
        }
    }
}

#[test]
fn restricted_alignment_bounds_join_levenshtein() {
    let all = words(4);
    for a in &all {
        for b in &all {
            let restricted = optimal_string_alignment::distance(a, b);
            assert!(restricted <= levenshtein::distance(a, b));
            assert!(restricted >= a.len().abs_diff(b.len()));
        }
    }
    assert_eq!(optimal_string_alignment::distance(b"CA", b"ABC"), 3);
    assert_eq!(optimal_string_alignment::distance(b"CA", b"AC"), 1);
    assert_eq!(levenshtein::distance(b"CA", b"AC"), 2);
}

#[test]
fn subset_knapsack_and_unit_cardinality_binomial() {
    for values in words(6) {
        let values: Vec<usize> = values.iter().map(|&v| usize::from(v) * 2).collect();
        let items: Vec<_> = values
            .iter()
            .map(|&v| knapsack::Item {
                weight: v,
                value: v as u64,
            })
            .collect();
        for target in 0..=13 {
            let witness = subset_sum::subset_sum(&values, target).unwrap();
            assert_eq!(
                witness.is_some(),
                knapsack::max_value(&items, target).unwrap() == target as u128
            );
            if let Some(indices) = witness {
                assert!(indices.windows(2).all(|p| p[0] < p[1]));
                assert_eq!(indices.iter().map(|&i| values[i]).sum::<usize>(), target);
            }
        }
    }
    for n in 0..=9usize {
        let mut histogram = vec![0u128; n + 1];
        for mask in 0..1usize << n {
            histogram[mask.count_ones() as usize] += 1;
        }
        for (k, &count) in histogram.iter().enumerate() {
            assert_eq!(
                count,
                binomial_coefficient::binomial_coefficient(n as u64, k as u64).unwrap()
            );
            let witness = subset_sum::subset_sum(&vec![1; n], k).unwrap().unwrap();
            assert_eq!(witness.len(), k);
        }
        assert!(
            subset_sum::subset_sum(&vec![1; n], n + 1)
                .unwrap()
                .is_none()
        );
    }
}

#[test]
fn interval_union_joins_grid_coverage_and_fenwick() {
    let mut options = Vec::new();
    for a in -2..=2i64 {
        for b in a..=2 {
            options.push((a, b));
        }
    }
    for &first in &options {
        for &second in &options {
            for &third in &options {
                let intervals = [first, second, third];
                let union = interval_union::interval_union(&intervals).unwrap();
                let points: Vec<i64> = (-2..2)
                    .map(|x| i64::from(intervals.iter().any(|&(a, b)| a <= x && x < b)))
                    .collect();
                let tree = fenwick::Fenwick::from_slice(&points);
                assert_eq!(union.covered_length, tree.prefix_sum(4).unwrap() as u128);
                assert!(union.intervals.windows(2).all(|p| p[0].1 < p[1].0));
                for x in -2..2 {
                    assert_eq!(
                        union.intervals.iter().any(|&(a, b)| a <= x && x < b),
                        points[(x + 2) as usize] == 1
                    );
                }
                for start in 0..=4 {
                    for end in start..=4 {
                        let lo = start as i64 - 2;
                        let hi = end as i64 - 2;
                        let clipped: u128 = union
                            .intervals
                            .iter()
                            .map(|&(a, b)| (b.min(hi) - a.max(lo)).max(0) as u128)
                            .sum();
                        assert_eq!(clipped, tree.range_sum(start, end).unwrap() as u128);
                    }
                }
            }
        }
    }
}
