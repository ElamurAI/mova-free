//! Two hundred exact, optimization and approximation algorithms with explicit domains and adjacent mathematical groupants.
//!
//! Implementations and fixtures are authored for Mova. See the crate README for
//! contracts, costs, provenance and the distinction between models and proofs.
//!
//! ```
//! use mova_algorithms::{binary_search, gcd, merge_sort};
//! let mut values = [7, 2, 7, 1];
//! merge_sort::merge_sort(&mut values);
//! assert_eq!(values, [1, 2, 7, 7]);
//! assert_eq!(binary_search::binary_search_first(&values, &7), Some(2));
//! assert_eq!(gcd::gcd(54, 24), 6);
//! ```
#![forbid(unsafe_code)]

pub mod a_star;
pub mod adaptive_simpson;
pub mod adaptive_trapezoid;
pub mod aho_corasick;
pub mod alias_method;
pub mod armijo_line_search;
pub mod articulation_points;
pub mod baby_step_giant_step;
pub mod banded_levenshtein;
pub mod barycentric_lagrange;
pub mod barzilai_borwein;
pub mod bellman_ford;
pub mod berlekamp_massey;
pub mod bernstein_de_casteljau;
pub mod bfs;
pub mod biconnected_components;
pub mod binary_search;
pub mod binomial_coefficient;
pub mod bipartite_matching;
pub mod bipartite_vertex_cover;
pub mod bisection_root;
pub mod bit_parallel_lcs;
pub mod bitset_subset_sum;
pub mod bk_tree;
pub mod blossom_matching;
pub mod bluestein_fft;
pub mod boruvka;
pub mod bounded_knapsack;
pub mod boyer_moore_horspool;
pub mod brent_minimize;
pub mod bridges;
pub mod bron_kerbosch;
pub mod burrows_wheeler;
pub mod byte_pair_encoding;
pub mod cartesian_tree;
pub mod chebyshev_clenshaw;
pub mod chinese_remainder;
pub mod cholesky;
pub mod closest_pair;
pub mod coin_change;
pub mod compensated_sum;
pub mod conjugate_gradient;
pub mod convex_hull;
pub mod coordinate_descent_lasso;
pub mod counting_sort;
pub mod dag_path_count;
pub mod dag_shortest_paths;
pub mod damerau_levenshtein;
pub mod dijkstra;
pub mod dinic;
pub mod discrete_cosine_transform;
pub mod disjoint_set;
pub mod divide_conquer_dp;
pub mod dominators;
pub mod eulerian_trail;
pub mod exact_cover;
pub mod extended_gcd;
pub mod fast_fourier_transform;
pub mod fast_walsh_hadamard;
pub mod fenwick;
pub mod fibonacci;
pub mod fista_lasso;
pub mod floor_sum;
pub mod floyd_warshall;
pub mod forward_backward;
pub mod frank_wolfe;
pub mod functional_graph_decomposition;
pub mod gauss_legendre;
pub mod gauss_seidel;
pub mod gaussian_elimination_modular;
pub mod gcd;
pub mod goertzel;
pub mod golden_section_search;
pub mod gradient_descent_quadratic;
pub mod haar_wavelet;
pub mod heap_sort;
pub mod heavy_ball_quadratic;
pub mod held_karp;
pub mod hirschberg;
pub mod histogram_quantiles;
pub mod hopcroft_karp;
pub mod horner;
pub mod householder_qr;
pub mod huffman;
pub mod hungarian;
pub mod integer_sqrt;
pub mod interval_tree;
pub mod interval_union;
pub mod inversion_count;
pub mod isotonic_regression;
pub mod jacobi_eigen;
pub mod jacobi_iteration;
pub mod johnson;
pub mod k_core;
pub mod karatsuba_polynomial;
pub mod kd_tree;
pub mod kmp;
pub mod knapsack;
pub mod knapsack_fptas;
pub mod knuth_optimal_bst;
pub mod kruskal;
pub mod l1_ball_projection;
pub mod lanczos;
pub mod lcp_array;
pub mod levenshtein;
pub mod li_chao_tree;
pub mod linear_recurrence;
pub mod longest_common_subsequence;
pub mod longest_common_substring;
pub mod longest_increasing_subsequence;
pub mod longest_palindromic_subsequence;
pub mod lowest_common_ancestor;
pub mod lu_decomposition;
pub mod lyndon_factorization;
pub mod lz77;
pub mod lzw;
pub mod manacher;
pub mod matrix_chain;
pub mod max_flow;
pub mod maximum_cardinality_search;
pub mod maximum_weight_closure;
pub mod meet_in_middle_knapsack;
pub mod merge_sort;
pub mod miller_rabin;
pub mod minimal_rotation;
pub mod minimum_arborescence;
pub mod minimum_cost_flow;
pub mod misra_gries;
pub mod mobius_sieve;
pub mod modified_gram_schmidt;
pub mod modular_power;
pub mod monotone_cubic;
pub mod myers_bitvector;
pub mod natural_cubic_spline;
pub mod needleman_wunsch;
pub mod nesterov_quadratic;
pub mod neville_interpolation;
pub mod newton_divided_differences;
pub mod number_theoretic_transform;
pub mod offline_dynamic_connectivity;
pub mod optimal_string_alignment;
pub mod palindromic_tree;
pub mod patience_diff;
pub mod persistent_segment_tree;
pub mod piecewise_linear;
pub mod pollard_rho;
pub mod power_iteration;
pub mod prim;
pub mod prime_factorization;
pub mod projected_gradient_box;
pub mod proximal_gradient_lasso;
pub mod prufer;
pub mod quantized_dot;
pub mod quickselect;
pub mod rabin_karp;
pub mod radix_sort;
pub mod romberg_integration;
pub mod run_length_encoding;
pub mod savitzky_golay;
pub mod segment_tree;
pub mod segmented_sieve;
pub mod shift_and;
pub mod sieve;
pub mod simplex_projection;
pub mod sinkhorn_scaling;
pub mod sliding_window_minimum;
pub mod smawk;
pub mod soft_threshold;
pub mod space_saving;
pub mod sparse_table;
pub mod splay_tree;
pub mod stable_marriage;
pub mod stoer_wagner;
pub mod streaming_moments;
pub mod strongly_connected_components;
pub mod subset_convolution;
pub mod subset_mobius;
pub mod subset_sum;
pub mod subset_sum_fptas;
pub mod subset_zeta;
pub mod suffix_array;
pub mod suffix_automaton;
pub mod tanh_sinh_quadrature;
pub mod tarjan_scc;
pub mod tonelli_shanks;
pub mod topological_sort;
pub mod totient_sieve;
pub mod transitive_closure;
pub mod transitive_reduction_dag;
pub mod treap;
pub mod triangular_solve;
pub mod two_sat;
pub mod two_way_matching;
pub mod viterbi;
pub mod wavelet_matrix;
pub mod weighted_covariance;
pub mod weighted_interval_scheduling;
pub mod wildcard_matching;
pub mod z_algorithm;
pub mod zero_one_bfs;

#[cfg(test)]
mod composition_tests {
    use super::*;

    #[test]
    fn sorting_then_search_matches_original_rank() {
        for len in 0..=5u32 {
            for mut code in 0..3usize.pow(len) {
                let input: Vec<i32> = (0..len)
                    .map(|_| {
                        let value = (code % 3) as i32 - 1;
                        code /= 3;
                        value
                    })
                    .collect();
                let mut sorted = input.clone();
                merge_sort::merge_sort(&mut sorted);
                for needle in -2..=2 {
                    let rank = input.iter().filter(|&&x| x < needle).count();
                    assert_eq!(binary_search::lower_bound(&sorted, &needle), rank);
                    let found = input.contains(&needle).then_some(rank);
                    assert_eq!(binary_search::binary_search_first(&sorted, &needle), found);
                }
            }
        }
    }

    #[test]
    fn unit_weight_graphs_share_distances() {
        for n in 1..=3usize {
            for mask in 0..(1usize << (n * n)) {
                let mut graph = vec![vec![]; n];
                for (u, edges) in graph.iter_mut().enumerate() {
                    for v in 0..n {
                        if mask & (1 << (u * n + v)) != 0 {
                            edges.push(v);
                        }
                    }
                }
                let weighted: Vec<Vec<_>> = graph
                    .iter()
                    .map(|edges| edges.iter().map(|&v| (v, 1u64)).collect())
                    .collect();
                for source in 0..n {
                    let unweighted = bfs::bfs(&graph, source).unwrap();
                    let expected: Vec<_> = unweighted
                        .into_iter()
                        .map(|d| d.map(|x| x as u128))
                        .collect();
                    assert_eq!(dijkstra::dijkstra(&weighted, source).unwrap(), expected);
                }
            }
        }
        assert!(bfs::bfs(&[vec![], vec![2]], 0).is_err());
        assert!(dijkstra::dijkstra(&[vec![], vec![(2, 1)]], 0).is_err());
    }

    #[test]
    fn disjoint_sets_match_undirected_reachability() {
        for n in 1..=4usize {
            let pairs: Vec<_> = (0..n)
                .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
                .collect();
            for mask in 0..(1usize << pairs.len()) {
                let mut graph = vec![vec![]; n];
                let mut sets = disjoint_set::DisjointSet::new(n);
                for (bit, &(u, v)) in pairs.iter().enumerate() {
                    if mask & (1 << bit) != 0 {
                        graph[u].push(v);
                        graph[v].push(u);
                        sets.union(u, v).unwrap();
                    }
                }
                let rows: Vec<_> = (0..n).map(|u| bfs::bfs(&graph, u).unwrap()).collect();
                for (u, row) in rows.iter().enumerate() {
                    for (v, distance) in row.iter().enumerate() {
                        assert_eq!(sets.connected(u, v).unwrap(), distance.is_some());
                    }
                }
                let components = (0..n)
                    .filter(|&i| (0..i).all(|j| rows[j][i].is_none()))
                    .count();
                assert_eq!(sets.components(), components);
            }
        }
    }

    #[test]
    fn ordering_algorithms_share_ranks_and_strict_length() {
        for len in 0..=5u32 {
            for mut code in 0..3usize.pow(len) {
                let input: Vec<i32> = (0..len)
                    .map(|_| {
                        let v = (code % 3) as i32 - 1;
                        code /= 3;
                        v
                    })
                    .collect();
                let mut heap = input.clone();
                heap_sort::heap_sort(&mut heap);
                let mut merged = input.clone();
                merge_sort::merge_sort(&mut merged);
                assert_eq!(heap, merged, "input={input:?}");
                for (k, &expected) in heap.iter().enumerate() {
                    let mut selected = input.clone();
                    assert_eq!(
                        quickselect::select_nth(&mut selected, k).copied(),
                        Some(expected),
                        "input={input:?}, k={k}"
                    );
                    assert!(selected[..k].iter().all(|&x| x <= expected));
                    assert!(selected[k + 1..].iter().all(|&x| x >= expected));
                }
                let ids = longest_increasing_subsequence::indices(&heap);
                let distinct = heap
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| i == 0 || x != &heap[i - 1])
                    .count();
                assert_eq!(ids.len(), distinct);
                assert!(ids.iter().all(|&i| i < heap.len()));
                assert!(
                    ids.windows(2)
                        .all(|p| p[0] < p[1] && heap[p[0]] < heap[p[1]])
                );
            }
        }
    }

    #[test]
    fn all_pairs_distances_identify_strong_components() {
        for n in 0..=3usize {
            for mask in 0..(1usize << (n * n)) {
                let mut graph = vec![vec![]; n];
                for (u, edges) in graph.iter_mut().enumerate() {
                    for v in 0..n {
                        if mask & (1 << (u * n + v)) != 0 {
                            edges.push(v);
                        }
                    }
                }
                let weighted: Vec<Vec<_>> = graph
                    .iter()
                    .enumerate()
                    .map(|(u, es)| es.iter().map(|&v| (v, ((u + v) % 3) as u64)).collect())
                    .collect();
                let distances = floyd_warshall::floyd_warshall(&weighted).unwrap();
                let components = strongly_connected_components::components(&graph).unwrap();
                let mut labels = vec![usize::MAX; n];
                for (label, component) in components.iter().enumerate() {
                    for &v in component {
                        assert_eq!(labels[v], usize::MAX);
                        labels[v] = label;
                    }
                }
                assert!(labels.iter().all(|&label| label != usize::MAX));
                for (source, row) in distances.iter().enumerate() {
                    assert_eq!(&dijkstra::dijkstra(&weighted, source).unwrap(), row);
                    for (v, distance) in row.iter().enumerate() {
                        assert_eq!(
                            labels[source] == labels[v],
                            distance.is_some() && distances[v][source].is_some(),
                            "n={n}, mask={mask}, source={source}, v={v}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn spanning_forest_preserves_original_connectivity() {
        for n in 0..=4usize {
            let pairs: Vec<_> = (0..n)
                .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
                .collect();
            for mask in 0..(1usize << pairs.len()) {
                let mut edges: Vec<_> = (0..n)
                    .map(|u| kruskal::Edge { u, v: u, weight: 0 })
                    .collect();
                let mut graph = vec![vec![]; n];
                for (bit, &(u, v)) in pairs.iter().enumerate() {
                    if mask & (1 << bit) != 0 {
                        edges.push(kruskal::Edge {
                            u,
                            v,
                            weight: ((u + 2 * v) % 5) as u64,
                        });
                        graph[u].push(v);
                        graph[v].push(u);
                    }
                }
                let forest = kruskal::minimum_spanning_forest(n, &edges).unwrap();
                let mut selected = vec![vec![]; n];
                let mut sets = disjoint_set::DisjointSet::new(n);
                let mut sum = 0u128;
                for &index in &forest.edge_indices {
                    let edge = edges[index];
                    assert!(
                        sets.union(edge.u, edge.v).unwrap(),
                        "selected edges must be acyclic"
                    );
                    selected[edge.u].push(edge.v);
                    selected[edge.v].push(edge.u);
                    sum += u128::from(edge.weight);
                }
                assert_eq!(sum, forest.total_weight);
                assert_eq!(sets.components(), forest.components);
                assert_eq!(forest.edge_indices.len(), n - forest.components);
                for source in 0..n {
                    let original = bfs::bfs(&graph, source).unwrap();
                    let spanning = bfs::bfs(&selected, source).unwrap();
                    assert!(
                        original
                            .iter()
                            .zip(&spanning)
                            .all(|(a, b)| a.is_some() == b.is_some()),
                        "n={n}, mask={mask}, source={source}"
                    );
                }
            }
        }
    }

    #[test]
    fn prefix_differences_rebuild_range_minimum_snapshots() {
        for n in 0..=12usize {
            let mut values: Vec<i64> = (0..n).map(|i| (i % 9) as i64 - 4).collect();
            let mut sums = fenwick::Fenwick::from_slice(&values);
            for round in 0..3usize {
                for (index, value) in values.iter_mut().enumerate() {
                    let delta = ((index + round) % 5) as i64 - 2;
                    sums.add(index, delta).unwrap();
                    *value += delta;
                }
                let recovered: Vec<i64> = (0..n)
                    .map(|index| {
                        let after = sums.prefix_sum(index + 1).unwrap();
                        let before = sums.prefix_sum(index).unwrap();
                        i64::try_from(after - before).unwrap()
                    })
                    .collect();
                assert_eq!(recovered, values, "n={n}, round={round}");
                let minima = sparse_table::SparseTable::from_slice(&recovered);
                for start in 0..=n {
                    for end in start..=n {
                        assert_eq!(minima.query(start, end), values[start..end].iter().min());
                        let expected: i128 =
                            values[start..end].iter().map(|&x| i128::from(x)).sum();
                        assert_eq!(sums.range_sum(start, end).unwrap(), expected);
                    }
                }
            }
        }
    }
    #[test]
    fn unreduced_modular_squares_have_exact_integer_roots() {
        let bases = (0..=256u64).chain([65_535, 1u64 << 31, u64::from(u32::MAX)]);
        for base in bases {
            let square = base.checked_mul(base).unwrap();
            let modulus = square.checked_add(1).unwrap();
            let reduced = modular_power::modular_power(base, 2, modulus).unwrap();
            assert_eq!(reduced, square, "base={base}, modulus={modulus}");
            assert_eq!(integer_sqrt::integer_sqrt(reduced), base);
        }
        let reduced = modular_power::modular_power(10, 2, 7).unwrap();
        assert_eq!(reduced, 2);
        assert_ne!(integer_sqrt::integer_sqrt(reduced), 10);
    }

    #[test]
    fn signed_shortest_paths_embed_nonnegative_distances() {
        for n in 1..=3 {
            for mask in 0..(1usize << (n * n)) {
                let mut signed = vec![vec![]; n];
                let mut unsigned = vec![vec![]; n];
                for u in 0..n {
                    for v in 0..n {
                        if mask & (1 << (u * n + v)) != 0 {
                            let weight = ((u + 2 * v) % 4) as u64;
                            unsigned[u].push((v, weight));
                            signed[u].push((v, weight as i64));
                            unsigned[u].push((v, weight + 3));
                            signed[u].push((v, weight as i64 + 3));
                        }
                    }
                }
                for source in 0..n {
                    let expected: Vec<_> = dijkstra::dijkstra(&unsigned, source)
                        .unwrap()
                        .into_iter()
                        .map(|d| d.map(|v| v as i128))
                        .collect();
                    assert_eq!(
                        bellman_ford::bellman_ford(&signed, source).unwrap(),
                        expected
                    );
                }
            }
        }
    }

    #[test]
    fn maximum_matching_has_no_augmenting_bfs_path() {
        for left in 0..=3 {
            for right in 0..=3 {
                for mask in 0..(1usize << (left * right)) {
                    let graph: Vec<Vec<_>> = (0..left)
                        .map(|u| {
                            (0..right)
                                .filter(|&v| mask & (1 << (u * right + v)) != 0)
                                .collect()
                        })
                        .collect();
                    let matching = bipartite_matching::bipartite_matching(&graph, right).unwrap();
                    let mut owner = vec![None; right];
                    for (u, matched) in matching.iter().enumerate() {
                        if let Some(v) = *matched {
                            assert!(graph[u].contains(&v));
                            assert!(owner[v].replace(u).is_none());
                        }
                    }
                    let source = left + right;
                    let mut residual = vec![vec![]; source + 1];
                    for u in 0..left {
                        if matching[u].is_none() {
                            residual[source].push(u);
                        }
                        for &v in &graph[u] {
                            if matching[u] == Some(v) {
                                residual[left + v].push(u);
                            } else {
                                residual[u].push(left + v);
                            }
                        }
                    }
                    let reachable = bfs::bfs(&residual, source).unwrap();
                    for v in 0..right {
                        if owner[v].is_none() {
                            assert!(reachable[left + v].is_none());
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn z_prefix_matches_agree_with_kmp() {
        for length in 0..=5u32 {
            for mut code in 0..3usize.pow(length) {
                let text: Vec<u8> = (0..length)
                    .map(|_| {
                        let b = (code % 3) as u8;
                        code /= 3;
                        b
                    })
                    .collect();
                for pattern in [
                    vec![0],
                    vec![1, 1],
                    vec![0, 1],
                    vec![2, 0, 2],
                    vec![0, 0, 0, 0, 0, 0],
                ] {
                    let mut joined = pattern.clone();
                    joined.push(3); // Delimiter is outside both byte alphabets.
                    joined.extend_from_slice(&text);
                    let z = z_algorithm::z_algorithm(&joined);
                    let found: Vec<_> = (0..text.len())
                        .filter(|&i| z[pattern.len() + 1 + i] >= pattern.len())
                        .collect();
                    assert_eq!(found, kmp::find_all(&text, &pattern));
                }
            }
        }
    }

    #[test]
    fn palindrome_spans_have_zero_reverse_edit_distance() {
        for length in 0..=6u32 {
            for mut code in 0..2usize.pow(length) {
                let text: Vec<u8> = (0..length)
                    .map(|_| {
                        let b = (code % 2) as u8;
                        code /= 2;
                        b
                    })
                    .collect();
                let radii = manacher::manacher(&text);
                assert_eq!(radii.odd.len(), text.len());
                assert_eq!(radii.even.len(), text.len());
                for center in 0..text.len() {
                    for (start, end) in [
                        (center + 1 - radii.odd[center], center + radii.odd[center]),
                        (center - radii.even[center], center + radii.even[center]),
                    ] {
                        let span = &text[start..end];
                        let reversed: Vec<_> = span.iter().rev().copied().collect();
                        assert_eq!(levenshtein::distance(span, &reversed), 0);
                        if start > 0 && end < text.len() {
                            let extended = &text[start - 1..end + 1];
                            let reverse_extended: Vec<_> = extended.iter().rev().copied().collect();
                            assert_ne!(levenshtein::distance(extended, &reverse_extended), 0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn dynamic_static_and_window_minima_share_snapshots() {
        for n in 0..=12 {
            let mut values: Vec<i64> = (0..n).map(|i| (i % 4) as i64 - 2).collect();
            let mut tree = segment_tree::SegmentTree::from_slice(&values);
            for round in 0..=3 {
                if n > 0 && round > 0 {
                    let index = (round * 5) % n;
                    let value = -(round as i64);
                    tree.update(index, value).unwrap();
                    values[index] = value;
                }
                let table = sparse_table::SparseTable::from_slice(&values);
                for width in 1..=n {
                    let windows = sliding_window_minimum::minimum_indices(&values, width);
                    assert_eq!(windows.len(), n - width + 1);
                    for (start, &index) in windows.iter().enumerate() {
                        let end = start + width;
                        let expected = (start..end).min_by_key(|&i| (values[i], i)).unwrap();
                        assert_eq!(index, expected);
                        assert_eq!(tree.query_index(start, end), Some(expected));
                        assert_eq!(tree.query(start, end), table.query(start, end));
                        assert_eq!(tree.query(start, end), Some(&values[expected]));
                    }
                }
                assert_eq!(tree.query(0, 0), None);
                assert!(sliding_window_minimum::minimum_indices(&values, 0).is_empty());
                assert!(sliding_window_minimum::minimum_indices(&values, n + 1).is_empty());
            }
        }
    }

    #[test]
    fn bounded_integer_sorts_preserve_generic_order_and_rank() {
        for length in 0..=6u32 {
            for mut code in 0..4usize.pow(length) {
                let input: Vec<usize> = (0..length)
                    .map(|_| {
                        let v = code % 4;
                        code /= 4;
                        v
                    })
                    .collect();
                let mut counted = input.clone();
                counting_sort::counting_sort(&mut counted, 4).unwrap();
                let mut merged = input.clone();
                merge_sort::merge_sort(&mut merged);
                let mut radix: Vec<u64> = input.iter().map(|&v| v as u64).collect();
                radix_sort::radix_sort(&mut radix).unwrap();
                assert_eq!(counted, merged);
                assert_eq!(radix, merged.iter().map(|&v| v as u64).collect::<Vec<_>>());
                for needle in 0..=4 {
                    assert_eq!(
                        binary_search::lower_bound(&counted, &needle),
                        input.iter().filter(|&&v| v < needle).count()
                    );
                }
            }
        }
    }

    #[test]
    fn prime_bezout_inverses_agree_with_modular_power() {
        let primes = sieve::sieve(127).unwrap();
        for p in primes {
            let modulus = p as u64;
            for a in 1..modulus {
                let bezout = extended_gcd::extended_gcd(a, modulus);
                assert_eq!(bezout.gcd, gcd::gcd(a, modulus));
                assert_eq!(bezout.gcd, 1);
                assert_eq!(i128::from(a) * bezout.x + i128::from(modulus) * bezout.y, 1);
                let inverse = bezout.x.rem_euclid(i128::from(modulus)) as u64;
                assert_eq!(
                    modular_power::modular_power(a, modulus - 2, modulus).unwrap(),
                    inverse
                );
                assert_eq!(u128::from(a) * u128::from(inverse) % u128::from(modulus), 1);
            }
            assert_eq!(extended_gcd::extended_gcd(modulus, modulus).gcd, modulus);
        }
    }
}
