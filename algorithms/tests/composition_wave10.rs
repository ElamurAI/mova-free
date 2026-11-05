//! Native joins retain graph, byte, item and coefficient identities.
use mova_algorithms::{
    bellman_ford, binomial_coefficient, bounded_knapsack, burrows_wheeler, hirschberg, johnson,
    kmp, knapsack, kruskal, lcp_array, levenshtein, linear_recurrence, longest_common_subsequence,
    number_theoretic_transform as ntt, prim, suffix_automaton::SuffixAutomaton, transitive_closure,
    wildcard_matching,
};

fn words(max_len: u32) -> Vec<Vec<u8>> {
    (0..=max_len)
        .flat_map(|n| {
            (0..1usize << n)
                .map(move |code| (0..n).map(|i| b'a' + ((code >> i) & 1) as u8).collect())
        })
        .collect()
}

#[test]
fn signed_distances_retain_reachability_and_potential_identity() {
    let arcs = [
        (0, 1),
        (1, 2),
        (2, 0),
        (2, 3),
        (3, 1),
        (0, 3),
        (1, 0),
        (3, 2),
    ];
    let potential = [-2i64, 0, 3, -1];
    for mask in 0..256 {
        let mut graph = vec![vec![]; 4];
        let mut plain = vec![vec![]; 4];
        for (i, &(u, v)) in arcs.iter().enumerate() {
            if mask & (1 << i) != 0 {
                graph[u].push((v, 1 + potential[v] - potential[u]));
                plain[u].push(v);
            }
        }
        let all = johnson::johnson(&graph).unwrap();
        let reach = transitive_closure::transitive_closure(&plain).unwrap();
        for source in 0..4 {
            let row = bellman_ford::bellman_ford(&graph, source).unwrap();
            assert_eq!(all[source], row);
            for (target, distance) in row.iter().enumerate() {
                assert_eq!(reach.reachable(source, target), Some(distance.is_some()));
            }
            assert_eq!(row[source], Some(0));
        }
    }
    // Global Johnson rejection differs from a single-source reachable-cycle gate.
    let bad = vec![vec![], vec![(2, -1)], vec![(1, 0)]];
    assert_eq!(
        bellman_ford::bellman_ford(&bad, 0).unwrap(),
        [Some(0), None, None]
    );
    assert!(matches!(
        johnson::johnson(&bad),
        Err(johnson::Error::NegativeCycle)
    ));
}

#[test]
fn spanning_forests_share_weight_and_original_connectivity() {
    let pairs = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for mask in 0..64 {
        let mut edges = vec![];
        for (i, &(u, v)) in pairs.iter().enumerate() {
            if mask & (1 << i) != 0 {
                edges.push(kruskal::Edge {
                    u,
                    v,
                    weight: (i % 3) as u64,
                });
            }
        }
        edges.push(kruskal::Edge {
            u: 2,
            v: 2,
            weight: 0,
        });
        let p = prim::minimum_spanning_forest(5, &edges).unwrap();
        let k = kruskal::minimum_spanning_forest(5, &edges).unwrap();
        assert_eq!(
            (p.total_weight, p.components),
            (k.total_weight, k.components)
        );
        assert_eq!(p.edge_indices.len(), 5 - p.components);
        let adjacency = |ids: &[usize]| {
            let mut g = vec![vec![]; 5];
            for &i in ids {
                let e = edges[i];
                g[e.u].push(e.v);
                g[e.v].push(e.u);
            }
            g
        };
        let whole = transitive_closure::transitive_closure(&adjacency(
            &(0..edges.len()).collect::<Vec<_>>(),
        ))
        .unwrap();
        let selected = transitive_closure::transitive_closure(&adjacency(&p.edge_indices)).unwrap();
        for u in 0..5 {
            for v in 0..5 {
                assert_eq!(whole.reachable(u, v), selected.reachable(u, v));
            }
        }
        assert_eq!(
            p.edge_indices
                .iter()
                .map(|&i| u128::from(edges[i].weight))
                .sum::<u128>(),
            p.total_weight
        );
    }
}

#[test]
fn lcp_and_automaton_count_the_same_distinct_substrings() {
    for text in words(7) {
        let (sa, lcp) = lcp_array::lcp_array(&text);
        assert_eq!(sa.len(), text.len());
        assert_eq!(lcp.len(), text.len());
        let n = text.len() as u128;
        let distinct = n * (n + 1) / 2 - lcp.iter().map(|&x| x as u128).sum::<u128>();
        let automaton = SuffixAutomaton::new(&text);
        assert_eq!(distinct, automaton.distinct_substrings());
        for (rank, &start) in sa.iter().enumerate() {
            let shared = lcp[rank];
            assert!(shared <= text.len() - start);
            for len in shared + 1..=text.len() - start {
                assert!(automaton.contains(&text[start..start + len]));
            }
        }
    }
}

#[test]
fn transformed_bytes_restore_literal_wildcard_search_identity() {
    let needles = words(3);
    for text in words(6) {
        let (last, primary) = burrows_wheeler::transform(&text);
        let restored = burrows_wheeler::inverse(&last, primary).unwrap();
        assert_eq!(restored, text);
        for needle in &needles {
            let mut pattern = vec![b'*'];
            pattern.extend(needle);
            pattern.push(b'*');
            assert_eq!(
                wildcard_matching::wildcard_match(&restored, &pattern),
                !kmp::find_all(&text, needle).is_empty()
            );
        }
    }
}

#[test]
fn multiplicity_expansion_preserves_knapsack_objective() {
    for code in 0..64usize {
        let items = (0..3)
            .map(|i| bounded_knapsack::Item {
                weight: i,
                value: (i + 2) as u128,
                count: (code >> (2 * i)) & 3,
            })
            .collect::<Vec<_>>();
        let flat = items
            .iter()
            .flat_map(|item| {
                std::iter::repeat_n(
                    knapsack::Item {
                        weight: item.weight,
                        value: item.value as u64,
                    },
                    item.count,
                )
            })
            .collect::<Vec<_>>();
        for capacity in 0..=10 {
            assert_eq!(
                bounded_knapsack::bounded_knapsack(&items, capacity).unwrap(),
                knapsack::max_value(&flat, capacity).unwrap()
            );
        }
    }
}

#[test]
fn hirschberg_pairs_preserve_lcs_and_edit_bounds() {
    let inputs = words(4);
    for a in &inputs {
        for b in &inputs {
            let pairs = hirschberg::hirschberg(a, b).unwrap();
            assert_eq!(pairs.len(), longest_common_subsequence::length(a, b));
            assert!(pairs.windows(2).all(|p| p[0].0 < p[1].0 && p[0].1 < p[1].1));
            for &(i, j) in &pairs {
                assert_eq!(a[i], b[j]);
            }
            let edit = levenshtein::distance(a, b);
            assert!(a.len().max(b.len()) - pairs.len() <= edit);
            assert!(edit <= a.len() + b.len() - 2 * pairs.len());
        }
    }
}

#[test]
fn ntt_binomial_products_preserve_coefficient_modulus() {
    let mut polynomial = vec![1];
    for degree in 0..=40 {
        for (k, &value) in polynomial.iter().enumerate() {
            let exact = binomial_coefficient::binomial_coefficient(degree, k as u64).unwrap();
            assert_eq!(value, (exact % u128::from(ntt::MODULUS)) as u64);
        }
        polynomial = ntt::convolution(&polynomial, &[1, 1]).unwrap();
    }
}

#[test]
fn recurrence_denominator_annihilates_the_generated_tail() {
    const P: u64 = ntt::MODULUS;
    for coefficients in [vec![2, 3], vec![P - 1, 0, 1], vec![0], vec![1]] {
        let initial = (1..=coefficients.len() as u64).collect::<Vec<_>>();
        let sequence = (0..32)
            .map(|n| linear_recurrence::nth_term(&initial, &coefficients, n, P).unwrap())
            .collect::<Vec<_>>();
        let mut q = vec![1];
        q.extend(coefficients.iter().map(|c| (P - c % P) % P));
        let product = ntt::convolution(&sequence, &q).unwrap();
        assert!(
            product[coefficients.len()..sequence.len()]
                .iter()
                .all(|&x| x == 0),
            "kernel {q:?} yielded {product:?}"
        );
    }
}
