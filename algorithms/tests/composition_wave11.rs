//! Supplied-input joins retain graph, byte, recurrence and objective identities.
use mova_algorithms::{
    berlekamp_massey, bipartite_matching, burrows_wheeler, damerau_levenshtein, dinic,
    fast_walsh_hadamard as fwht, held_karp, hopcroft_karp, hungarian, levenshtein,
    linear_recurrence, lyndon_factorization, manacher, max_flow, number_theoretic_transform as ntt,
    optimal_string_alignment, palindromic_tree, strongly_connected_components, tarjan_scc,
    transitive_closure,
};
fn words(max: u32) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n)
                .map(move |code| (0..n).map(|i| b'a' + ((code >> i) & 1) as u8).collect())
        })
        .collect()
}
#[test]
fn bipartite_matching_embeds_in_the_same_unit_capacity_network() {
    for mask in 0..512 {
        let mut graph = vec![vec![]; 3];
        for (u, row) in graph.iter_mut().enumerate() {
            for v in 0..3 {
                if mask & (1 << (3 * u + v)) != 0 {
                    row.push(v);
                }
            }
        }
        if mask & 1 != 0 {
            graph[0].push(0);
        }
        let matched = hopcroft_karp::maximum_matching(&graph, 3).unwrap();
        let old = bipartite_matching::bipartite_matching(&graph, 3).unwrap();
        let count = matched.iter().flatten().count();
        assert_eq!(count, old.iter().flatten().count());
        let rights = matched
            .iter()
            .flatten()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(rights.len(), count);
        for (u, mate) in matched.iter().enumerate() {
            if let Some(v) = mate {
                assert!(graph[u].contains(v));
            }
        }
        let mut edges = vec![];
        for (u, row) in graph.iter().enumerate() {
            edges.push(max_flow::Edge {
                from: 6,
                to: u,
                capacity: 1,
            });
            edges.push(max_flow::Edge {
                from: 3 + u,
                to: 7,
                capacity: 1,
            });
            for &v in row {
                edges.push(max_flow::Edge {
                    from: u,
                    to: 3 + v,
                    capacity: 1,
                });
            }
        }
        let fast = dinic::max_flow(8, &edges, 6, 7).unwrap();
        let reference = max_flow::max_flow(8, &edges, 6, 7).unwrap();
        assert_eq!(fast.value, count as u128);
        assert_eq!(fast.value, reference.value);
        for flow in [fast, reference] {
            assert_eq!(flow.edge_flows.len(), edges.len());
            let mut balance = [0i128; 8];
            for (e, &f) in edges.iter().zip(&flow.edge_flows) {
                assert!(f <= e.capacity);
                balance[e.from] -= i128::from(f);
                balance[e.to] += i128::from(f);
            }
            assert_eq!(&balance[..6], &[0; 6]);
            assert_eq!(balance[6], -(count as i128));
            assert_eq!(balance[7], count as i128);
            let cut = edges
                .iter()
                .filter(|e| flow.source_side[e.from] && !flow.source_side[e.to])
                .map(|e| u128::from(e.capacity))
                .sum::<u128>();
            assert_eq!(cut, flow.value);
        }
    }
}
#[test]
fn lowlink_partition_preserves_mutual_reachability() {
    for mask in 0..512 {
        let graph = (0..3)
            .map(|u| {
                (0..3)
                    .filter(|v| mask & (1 << (3 * u + v)) != 0)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let parts = tarjan_scc::components(&graph).unwrap();
        assert_eq!(
            parts,
            strongly_connected_components::components(&graph).unwrap()
        );
        let reach = transitive_closure::transitive_closure(&graph).unwrap();
        let mut owner = [usize::MAX; 3];
        for (id, part) in parts.iter().enumerate() {
            for &v in part {
                assert_eq!(owner[v], usize::MAX);
                owner[v] = id;
            }
        }
        for u in 0..3 {
            for v in 0..3 {
                assert_eq!(
                    owner[u] == owner[v],
                    reach.reachable(u, v).unwrap() && reach.reachable(v, u).unwrap()
                );
            }
        }
    }
}
#[test]
fn unrestricted_edits_refine_restricted_alignment() {
    let inputs = words(3);
    for a in &inputs {
        for b in &inputs {
            let full = damerau_levenshtein::distance(a, b).unwrap();
            let restricted = optimal_string_alignment::distance(a, b);
            let ordinary = levenshtein::distance(a, b);
            assert!(full <= restricted && restricted <= ordinary);
            assert_eq!(full, damerau_levenshtein::distance(b, a).unwrap());
        }
    }
    assert_eq!(damerau_levenshtein::distance(b"CA", b"ABC").unwrap(), 2);
    assert_eq!(optimal_string_alignment::distance(b"CA", b"ABC"), 3);
}
#[test]
fn palindrome_tree_counts_match_center_radii() {
    for text in words(7) {
        let tree = palindromic_tree::build(&text).unwrap();
        let radii = manacher::manacher(&text);
        let occurrences = radii.odd.iter().chain(&radii.even).sum::<usize>();
        assert_eq!(
            tree.palindromes()
                .iter()
                .map(|p| p.occurrences)
                .sum::<usize>(),
            occurrences
        );
        let longest = radii
            .odd
            .iter()
            .map(|r| 2 * r - 1)
            .chain(radii.even.iter().map(|r| 2 * r))
            .max()
            .unwrap_or(0);
        assert_eq!(tree.longest().map(|r| r.len()).unwrap_or(0), longest);
        assert_eq!(tree.distinct_count(), tree.palindromes().len());
        for p in tree.palindromes() {
            let bytes = &text[p.range.clone()];
            assert!(bytes.iter().eq(bytes.iter().rev()));
        }
    }
}
#[test]
fn doubled_lyndon_factors_select_the_bwt_minimum_rotation() {
    for text in words(7) {
        let factors = lyndon_factorization::factorize(&text);
        let rebuilt = factors
            .iter()
            .flat_map(|r| text[r.clone()].iter().copied())
            .collect::<Vec<_>>();
        assert_eq!(rebuilt, text);
        assert!(
            factors
                .windows(2)
                .all(|f| text[f[0].clone()] >= text[f[1].clone()])
        );
        let (last, _) = burrows_wheeler::transform(&text);
        let minimum = burrows_wheeler::inverse(&last, 0).unwrap();
        if text.is_empty() {
            assert!(minimum.is_empty());
            continue;
        }
        let twice = text.iter().chain(&text).copied().collect::<Vec<_>>();
        let doubled = lyndon_factorization::factorize(&twice);
        let start = doubled
            .iter()
            .rfind(|r| r.start < text.len())
            .unwrap()
            .start;
        let rotation = text[start..]
            .iter()
            .chain(&text[..start])
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(rotation, minimum);
    }
}
#[test]
fn learned_finite_recurrences_predict_the_same_generated_sequence() {
    const P: u64 = berlekamp_massey::MODULUS;
    for (coefficients, initial) in [
        (vec![1, 1], vec![0, 1]),
        (vec![2, 3], vec![1, 2]),
        (vec![1, 1, 1], vec![1, 2, 4]),
        (vec![0], vec![0]),
    ] {
        let coefficients: Vec<u64> = coefficients;
        let mut values = initial;
        while values.len() < 32 {
            let t = values.len();
            let next = coefficients.iter().enumerate().fold(0u128, |s, (j, &c)| {
                (s + u128::from(c) * u128::from(values[t - 1 - j])) % u128::from(P)
            });
            values.push(next as u64);
        }
        let inferred = berlekamp_massey::minimal_recurrence(&values[..12]).unwrap();
        if inferred.is_empty() {
            assert!(values.iter().all(|&x| x == 0));
            continue;
        }
        let seed = &values[..inferred.len()];
        for (n, &expected) in values.iter().enumerate() {
            assert_eq!(
                linear_recurrence::nth_term(seed, &inferred, n as u64, P).unwrap(),
                expected
            );
        }
    }
}
#[test]
fn no_carry_support_embeds_xor_in_polynomial_convolution() {
    assert_eq!(fwht::MODULUS, ntt::MODULUS);
    for seed in 0..32u64 {
        let mut a = vec![0; 16];
        let mut b = vec![0; 16];
        for i in 0..4 {
            a[i] = (seed + 3 * i as u64) % 7;
            b[4 * i] = (2 * seed + i as u64) % 11;
        }
        let xor = fwht::xor_convolution(&a, &b).unwrap();
        let ordinary = ntt::convolution(&a, &b).unwrap();
        assert_eq!(xor, ordinary[..16]);
        assert!(ordinary[16..].iter().all(|&x| x == 0));
    }
    let xor = fwht::xor_convolution(&[0, 1], &[0, 1]).unwrap();
    let ordinary = ntt::convolution(&[0, 1], &[0, 1]).unwrap();
    assert_eq!(xor, [1, 0]);
    assert_eq!(ordinary, [0, 0, 1]);
}
#[test]
fn assignment_relaxation_bounds_the_same_tour_objective() {
    for n in 2..=5 {
        for seed in 0..16 {
            let diagonal = (9 * n + 1) as i64;
            let costs = (0..n)
                .map(|u| {
                    (0..n)
                        .map(|v| {
                            if u == v {
                                diagonal
                            } else {
                                ((17 * u + 7 * v + 3 * seed + u * v) % 9) as i64
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let arcs = costs
                .iter()
                .enumerate()
                .map(|(u, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(v, &x)| (u != v).then_some(x as u64))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let assignment = hungarian::hungarian(&costs).unwrap();
            let tour = held_karp::held_karp(&arcs).unwrap().unwrap();
            assert_eq!(assignment.columns.len(), n);
            assert_eq!(
                assignment
                    .columns
                    .iter()
                    .copied()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                n
            );
            assert_eq!(
                assignment.cost,
                assignment
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(u, &v)| i128::from(costs[u][v]))
                    .sum::<i128>()
            );
            assert!(assignment.cost >= 0 && assignment.cost as u128 <= tour.cost);
            for (u, row) in costs.iter().enumerate() {
                for (v, &c) in row.iter().enumerate() {
                    assert!(
                        assignment.row_potentials[u] + assignment.column_potentials[v]
                            <= i128::from(c)
                    );
                }
            }
            assert_eq!(
                assignment
                    .row_potentials
                    .iter()
                    .chain(&assignment.column_potentials)
                    .sum::<i128>(),
                assignment.cost
            );
            assert_eq!(tour.vertices.len(), n + 1);
            assert_eq!(tour.vertices.first(), Some(&0));
            assert_eq!(tour.vertices.last(), Some(&0));
            assert_eq!(
                tour.vertices[..n]
                    .iter()
                    .copied()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                n
            );
            assert_eq!(
                tour.vertices
                    .windows(2)
                    .map(|e| u128::from(arcs[e[0]][e[1]].unwrap()))
                    .sum::<u128>(),
                tour.cost
            );
            let mut seen = vec![false; n];
            let mut v = 0;
            for _ in 0..n {
                if seen[v] {
                    break;
                }
                seen[v] = true;
                v = assignment.columns[v];
            }
            if seen.iter().all(|x| *x) && v == 0 {
                assert_eq!(assignment.cost as u128, tour.cost);
            }
        }
    }
}
