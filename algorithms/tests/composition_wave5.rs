//! Finite public-API joins; no leaf implementation is substituted.
use mova_algorithms::{
    articulation_points, bfs, bipartite_matching, bridges, coin_change, eulerian_trail, huffman,
    kmp, longest_common_subsequence, longest_palindromic_subsequence, lowest_common_ancestor::Lca,
    manacher, matrix_chain, max_flow, suffix_array, suffix_automaton::SuffixAutomaton,
};
use std::collections::BTreeSet;

#[test]
fn flow_matches_bipartite_cardinality() {
    for left in 0..=3 {
        for right in 0..=3 {
            for mask in 0..1usize << (left * right) {
                let graph: Vec<Vec<usize>> = (0..left)
                    .map(|u| {
                        (0..right)
                            .filter(|&v| mask & (1 << (u * right + v)) != 0)
                            .collect()
                    })
                    .collect();
                let matching = bipartite_matching::bipartite_matching(&graph, right).unwrap();
                let source = left + right;
                let sink = source + 1;
                let mut edges = Vec::new();
                for (u, row) in graph.iter().enumerate() {
                    edges.push(max_flow::Edge {
                        from: source,
                        to: u,
                        capacity: 1,
                    });
                    for &v in row {
                        edges.push(max_flow::Edge {
                            from: u,
                            to: left + v,
                            capacity: 1,
                        });
                    }
                }
                for v in 0..right {
                    edges.push(max_flow::Edge {
                        from: left + v,
                        to: sink,
                        capacity: 1,
                    });
                }
                let flow = max_flow::max_flow(sink + 1, &edges, source, sink).unwrap();
                assert_eq!(flow.value, matching.iter().flatten().count() as u128);
                let cut: u128 = edges
                    .iter()
                    .filter(|e| flow.source_side[e.from] && !flow.source_side[e.to])
                    .map(|e| u128::from(e.capacity))
                    .sum();
                assert_eq!(cut, flow.value);
                let mut used_left = BTreeSet::new();
                let mut used_right = BTreeSet::new();
                for (e, &value) in edges.iter().zip(&flow.edge_flows) {
                    assert!(value <= 1);
                    if e.from < left && e.to >= left && e.to < source && value == 1 {
                        assert!(used_left.insert(e.from));
                        assert!(used_right.insert(e.to - left));
                        assert!(graph[e.from].contains(&(e.to - left)));
                    }
                }
                assert_eq!(used_left.len() as u128, flow.value);
            }
        }
    }
}

fn components(
    n: usize,
    edges: &[bridges::Edge],
    removed_edge: Option<usize>,
    removed_vertex: Option<usize>,
) -> usize {
    let mut graph = vec![vec![]; n];
    for (i, e) in edges.iter().enumerate() {
        if Some(i) != removed_edge && Some(e.u) != removed_vertex && Some(e.v) != removed_vertex {
            graph[e.u].push(e.v);
            graph[e.v].push(e.u);
        }
    }
    let mut seen = vec![false; n];
    let mut count = 0;
    for u in 0..n {
        if Some(u) != removed_vertex && !seen[u] {
            count += 1;
            for (v, d) in bfs::bfs(&graph, u).unwrap().into_iter().enumerate() {
                seen[v] |= d.is_some();
            }
        }
    }
    count
}

#[test]
fn deletion_connectivity_and_euler_identity() {
    let pairs: Vec<_> = (0..3)
        .flat_map(|u| (u..3).map(move |v| bridges::Edge { u, v }))
        .collect();
    for len in 0..=4u32 {
        for mut code in 0..pairs.len().pow(len) {
            let edges: Vec<_> = (0..len)
                .map(|_| {
                    let e = pairs[code % pairs.len()];
                    code /= pairs.len();
                    e
                })
                .collect();
            let base = components(3, &edges, None, None);
            let expected_edges: Vec<_> = (0..edges.len())
                .filter(|&i| components(3, &edges, Some(i), None) > base)
                .collect();
            let expected_vertices: Vec<_> = (0..3)
                .filter(|&v| components(3, &edges, None, Some(v)) > base)
                .collect();
            assert_eq!(bridges::bridges(3, &edges).unwrap(), expected_edges);
            assert_eq!(
                articulation_points::articulation_points(3, &edges).unwrap(),
                expected_vertices
            );
            let mut degree = [0usize; 3];
            for e in &edges {
                degree[e.u] += 1;
                degree[e.v] += 1;
            }
            let isolates = degree.iter().filter(|&&d| d == 0).count();
            let odd = degree.iter().filter(|&&d| d % 2 != 0).count();
            let exists = edges.is_empty() || (base - isolates == 1 && (odd == 0 || odd == 2));
            let trail = eulerian_trail::eulerian_trail(3, &edges).unwrap();
            assert_eq!(trail.is_some(), exists);
            if let Some(trail) = trail {
                assert_eq!(trail.edges.len(), edges.len());
                assert_eq!(trail.vertices.len(), edges.len() + 1);
                assert_eq!(
                    trail.edges.iter().copied().collect::<BTreeSet<_>>(),
                    (0..edges.len()).collect()
                );
                for (step, &id) in trail.edges.iter().enumerate() {
                    let e = edges[id];
                    let (a, b) = (trail.vertices[step], trail.vertices[step + 1]);
                    assert!((e.u == a && e.v == b) || (e.u == b && e.v == a));
                }
            }
        }
    }
}

#[test]
fn lca_reconstructs_bfs_path_lengths() {
    for n in 0..=6usize {
        let total: usize = (1..=n).product();
        for mut code in 0..total {
            let parents: Vec<_> = (0..n)
                .map(|v| {
                    let p = code % (v + 1);
                    code /= v + 1;
                    (p != v).then_some(p)
                })
                .collect();
            let lca = Lca::new(&parents).unwrap();
            let mut graph = vec![vec![]; n];
            let mut depth = vec![0; n];
            for (v, p) in parents.iter().enumerate() {
                if let Some(p) = *p {
                    graph[v].push(p);
                    graph[p].push(v);
                    depth[v] = depth[p] + 1;
                }
            }
            for u in 0..n {
                for (v, distance) in bfs::bfs(&graph, u).unwrap().into_iter().enumerate() {
                    let ancestor = lca.query(u, v).unwrap();
                    assert_eq!(
                        ancestor.map(|a| depth[u] + depth[v] - 2 * depth[a]),
                        distance
                    );
                    if let Some(a) = ancestor {
                        for mut node in [u, v] {
                            while node != a {
                                node = parents[node].expect("ancestor must be on parent chain");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn binary_words(max: u32) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}

#[test]
fn automaton_suffix_order_and_kmp_share_substrings() {
    let queries = binary_words(3);
    for text in binary_words(7) {
        let automaton = SuffixAutomaton::new(&text);
        let order = suffix_array::suffix_array(&text);
        let lcp_sum: usize = order
            .windows(2)
            .map(|pair| {
                text[pair[0]..]
                    .iter()
                    .zip(&text[pair[1]..])
                    .take_while(|(a, b)| a == b)
                    .count()
            })
            .sum();
        let count = text.len() * (text.len() + 1) / 2 - lcp_sum;
        assert_eq!(automaton.distinct_substrings(), count as u128);
        let literal: BTreeSet<_> = (0..text.len())
            .flat_map(|i| (i + 1..=text.len()).map(move |j| (i, j)))
            .map(|(i, j)| text[i..j].to_vec())
            .collect();
        assert_eq!(count, literal.len());
        for query in &queries {
            assert_eq!(
                automaton.contains(query),
                !kmp::find_all(&text, query).is_empty()
            );
            assert_eq!(
                automaton.contains(query),
                query.is_empty() || literal.contains(query)
            );
        }
    }
}

fn optimal_tree_cost(weights: &[u64], mask: usize, memo: &mut [Option<u128>]) -> u128 {
    if mask.count_ones() <= 1 {
        return 0;
    }
    if let Some(value) = memo[mask] {
        return value;
    }
    let sum: u128 = weights
        .iter()
        .enumerate()
        .filter(|(i, _)| mask & (1 << i) != 0)
        .map(|(_, &w)| u128::from(w))
        .sum();
    let fixed = 1usize << mask.trailing_zeros();
    let mut best = u128::MAX;
    let mut subset = (mask - 1) & mask;
    while subset != 0 {
        if subset & fixed != 0 {
            best = best.min(
                optimal_tree_cost(weights, subset, memo)
                    + optimal_tree_cost(weights, mask ^ subset, memo)
                    + sum,
            );
        }
        subset = (subset - 1) & mask;
    }
    memo[mask] = Some(best);
    best
}

#[test]
fn huffman_prefix_trie_has_optimal_weighted_paths() {
    for n in 0..=5u32 {
        for mut code in 0..3usize.pow(n) {
            let weights: Vec<_> = (0..n)
                .map(|_| {
                    let w = (code % 3) as u64;
                    code /= 3;
                    w
                })
                .collect();
            let codes = huffman::codes(&weights);
            assert_eq!(codes.len(), weights.len());
            for (i, a) in codes.iter().enumerate() {
                for (j, b) in codes.iter().enumerate() {
                    if i != j {
                        assert!(!b.starts_with(a));
                    }
                }
            }
            let weighted: u128 = weights
                .iter()
                .zip(&codes)
                .map(|(&w, c)| u128::from(w) * c.len() as u128)
                .sum();
            let expected = if weights.len() == 1 {
                u128::from(weights[0])
            } else {
                optimal_tree_cost(
                    &weights,
                    (1 << weights.len()) - 1,
                    &mut vec![None; 1 << weights.len()],
                )
            };
            assert_eq!(weighted, expected);
            let prefixes: BTreeSet<Vec<bool>> = codes
                .iter()
                .flat_map(|c| (1..=c.len()).map(|end| c[..end].to_vec()))
                .collect();
            let edge_cost: u128 = prefixes
                .iter()
                .map(|prefix| {
                    weights
                        .iter()
                        .zip(&codes)
                        .filter(|(_, c)| c.starts_with(prefix))
                        .map(|(&w, _)| u128::from(w))
                        .sum::<u128>()
                })
                .sum();
            assert_eq!(edge_cost, weighted);
        }
    }
}

#[test]
fn palindrome_subsequence_lcs_and_contiguous_bound() {
    for text in binary_words(8) {
        let reverse: Vec<_> = text.iter().rev().copied().collect();
        let length = longest_palindromic_subsequence::length(&text);
        assert_eq!(length, longest_common_subsequence::length(&text, &reverse));
        let radii = manacher::manacher(&text);
        let contiguous = radii
            .odd
            .iter()
            .map(|&r| 2 * r - 1)
            .chain(radii.even.iter().map(|&r| 2 * r))
            .max()
            .unwrap_or(0);
        assert!(contiguous <= length && length <= text.len());
        // This equality concerns length, not a palindromic LCS witness.
    }
}

#[test]
fn unit_work_paths_price_coin_and_square_chain_restrictions() {
    for mask in 0..32usize {
        let coins: Vec<_> = (1..=5).filter(|&c| mask & (1 << (c - 1)) != 0).collect();
        for amount in 0..=24usize {
            let graph: Vec<Vec<_>> = (0..=amount)
                .map(|a| {
                    coins
                        .iter()
                        .filter_map(|&c| (a + c <= amount).then_some(a + c))
                        .collect()
                })
                .collect();
            assert_eq!(
                coin_change::minimum_coins(&coins, amount).unwrap(),
                bfs::bfs(&graph, 0).unwrap()[amount]
            );
        }
    }
    // Equal-square chains form a separate restricted unit-work embedding:
    // all parenthesizations make m-1 merges, each costing side^3.
    for matrices in 1..=10usize {
        for side in 1..=5u64 {
            let steps = coin_change::minimum_coins(&[1], matrices - 1)
                .unwrap()
                .unwrap();
            assert_eq!(
                matrix_chain::minimum_cost(&vec![side; matrices + 1]).unwrap(),
                steps as u128 * u128::from(side).pow(3)
            );
        }
    }
}
