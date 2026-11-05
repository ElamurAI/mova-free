//! Wave16 bounded native joins. Exact observations; no global correctness claim.
use mova_algorithms::{
    banded_levenshtein, bipartite_vertex_cover, bit_parallel_lcs, boruvka, bron_kerbosch,
    disjoint_set::DisjointSet,
    fibonacci, gcd, kruskal, levenshtein, li_chao_tree, linear_recurrence,
    longest_common_subsequence, mobius_sieve,
    offline_dynamic_connectivity::{self, Answer, Operation},
    prim, prufer, sieve, totient_sieve,
};
use std::collections::BTreeSet;

#[test]
fn signed_forest_shift_joins_boruvka_kruskal_prim() {
    let pairs = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    // Constant weight translation preserves optimum only at fixed forest cardinality.
    for mask in 0..64_usize {
        for phase in 0..3_i64 {
            let mut edges: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(i, &(u, v))| boruvka::Edge {
                    u,
                    v,
                    weight: ((i as i64 * 3 + phase) % 11) - 5,
                })
                .collect();
            edges.push(boruvka::Edge {
                u: 0,
                v: 0,
                weight: -7,
            });
            if mask & 1 != 0 {
                edges.push(boruvka::Edge {
                    u: 1,
                    v: 0,
                    weight: 2,
                });
            }
            let shifted: Vec<_> = edges
                .iter()
                .map(|e| kruskal::Edge {
                    u: e.u,
                    v: e.v,
                    weight: (e.weight + 7) as u64,
                })
                .collect();
            let b = boruvka::minimum_spanning_forest(5, &edges).unwrap();
            let k = kruskal::minimum_spanning_forest(5, &shifted).unwrap();
            let p = prim::minimum_spanning_forest(5, &shifted).unwrap();
            let mut full = DisjointSet::new(5);
            for e in &edges {
                full.union(e.u, e.v).unwrap();
            }
            assert_eq!(b.components, full.components());
            assert_eq!(b.edge_indices.len(), 5 - full.components());
            for f in [&k, &p] {
                assert_eq!(f.components, b.components);
                assert_eq!(f.edge_indices.len(), b.edge_indices.len());
                assert_eq!(
                    f.total_weight as i128 - 7 * b.edge_indices.len() as i128,
                    b.total_weight
                );
            }
            let mut witness = DisjointSet::new(5);
            let mut sum = 0_i128;
            for &i in &b.edge_indices {
                let e = edges[i];
                assert!(witness.union(e.u, e.v).unwrap());
                sum += i128::from(e.weight);
            }
            assert_eq!(sum, b.total_weight);
            for u in 0..5 {
                for v in 0..5 {
                    assert_eq!(
                        witness.find(u).unwrap() == witness.find(v).unwrap(),
                        full.find(u).unwrap() == full.find(v).unwrap()
                    );
                }
            }
        }
    }
    assert_eq!(
        boruvka::minimum_spanning_forest(0, &[])
            .unwrap()
            .total_weight,
        0
    );
}

#[test]
fn prufer_words_join_tree_degree_and_dsu_identity() {
    for n in 2_usize..=6 {
        for mut rank in 0..n.pow((n - 2) as u32) {
            let mut code = vec![0; n - 2];
            for value in &mut code {
                *value = rank % n;
                rank /= n;
            }
            let edges = prufer::decode(n, &code).unwrap();
            assert_eq!(edges.len(), n - 1);
            let mut degree = vec![0; n];
            let mut tree = DisjointSet::new(n);
            for &(u, v) in &edges {
                assert!(u < v && v < n);
                assert!(tree.union(u, v).unwrap());
                degree[u] += 1;
                degree[v] += 1;
            }
            assert_eq!(tree.components(), 1);
            for (v, &d) in degree.iter().enumerate() {
                assert_eq!(d, 1 + code.iter().filter(|&&x| x == v).count());
            }
            assert_eq!(prufer::encode(n, &edges).unwrap(), code);
        }
    }
    // Literal topology anchors supplement inverse-only evidence.
    assert_eq!(
        prufer::decode(4, &[1, 1]).unwrap(),
        vec![(0, 1), (1, 2), (1, 3)]
    );
    assert_eq!(
        prufer::encode(4, &[(0, 1), (1, 2), (2, 3)]).unwrap(),
        vec![1, 2]
    );
    for n in [0, 1] {
        assert!(prufer::decode(n, &[]).unwrap().is_empty());
        assert!(prufer::encode(n, &[]).unwrap().is_empty());
    }
}

#[test]
fn complement_clique_joins_bipartite_cover_and_independent_witness() {
    for mask in 0..512_usize {
        let mut graph = vec![vec![]; 3];
        let mut original = BTreeSet::new();
        for (u, neighbors) in graph.iter_mut().enumerate() {
            for v in 0..3 {
                if mask & (1 << (3 * u + v)) != 0 {
                    neighbors.push(v);
                    original.insert((u, v + 3));
                }
            }
        }
        let complement: Vec<_> = (0..6)
            .flat_map(|u| (u + 1..6).map(move |v| (u, v)))
            .filter(|e| !original.contains(e))
            .collect();
        let clique = bron_kerbosch::maximum_clique(6, &complement).unwrap();
        let cover = bipartite_vertex_cover::minimum_vertex_cover(&graph, 3).unwrap();
        assert_eq!(clique.len(), 6 - cover.left.len() - cover.right.len());
        for &(u, v) in &original {
            assert!(!clique.contains(&u) || !clique.contains(&v));
            assert!(cover.left.contains(&u) || cover.right.contains(&(v - 3)));
        }
        let independent: Vec<_> = (0..6)
            .filter(|&v| {
                if v < 3 {
                    !cover.left.contains(&v)
                } else {
                    !cover.right.contains(&(v - 3))
                }
            })
            .collect();
        for &(u, v) in &original {
            assert!(!independent.contains(&u) || !independent.contains(&v));
        }
        assert_eq!(independent.len(), clique.len());
        // Complete 64-subset oracle also anchors the lexicographic tie rule.
        let mut best = Vec::new();
        for subset in 0..64_usize {
            let candidate: Vec<_> = (0..6).filter(|v| subset & (1 << v) != 0).collect();
            if original
                .iter()
                .all(|&(u, v)| !candidate.contains(&u) || !candidate.contains(&v))
                && (candidate.len() > best.len()
                    || (candidate.len() == best.len() && candidate < best))
            {
                best = candidate;
            }
        }
        assert_eq!(clique, best);
    }
}

#[test]
fn bounded_edit_lcs_join_exact_lengths_and_metric_bounds() {
    let words: Vec<Vec<u8>> = (0..=4)
        .flat_map(|n| {
            (0..1_usize << n)
                .map(move |mask| (0..n).map(|i| b'a' + ((mask >> i) & 1) as u8).collect())
        })
        .collect();
    let check = |a: &[u8], b: &[u8]| {
        let distance = levenshtein::distance(a, b);
        let lcs = bit_parallel_lcs::length(a, b).unwrap();
        assert_eq!(lcs, longest_common_subsequence::length(a, b));
        assert!(distance >= a.len().max(b.len()) - lcs);
        assert!(distance <= a.len() + b.len() - 2 * lcs);
        assert!(distance >= a.len().abs_diff(b.len()));
        for cap in [0, 1, 2, 3, 4, 64, usize::MAX] {
            assert_eq!(
                banded_levenshtein::distance(a, b, cap).unwrap(),
                (distance <= cap).then_some(distance)
            );
            assert_eq!(
                banded_levenshtein::distance(b, a, cap).unwrap(),
                (distance <= cap).then_some(distance)
            );
        }
    };
    for a in &words {
        for b in &words {
            check(a, b);
        }
    }
    for n in [63, 64] {
        let a: Vec<_> = (0..n).map(|i| (i % 3) as u8).collect();
        let b: Vec<_> = (0..n + 3).map(|i| ((i + 1) % 3) as u8).collect();
        check(&a, &b);
    }
    assert!(bit_parallel_lcs::length(&[0; 65], &[0; 65]).is_err());
    let wide = vec![0; banded_levenshtein::MAX_BAND_CELLS];
    assert!(banded_levenshtein::distance(&wide, &wide, usize::MAX).is_err());
    assert_eq!(banded_levenshtein::distance(&wide, &[], 0).unwrap(), None);
}

#[test]
fn totient_mobius_join_divisor_convolutions_gcd_and_primes() {
    const LIMIT: usize = 256;
    let phi = totient_sieve::totient_sieve(LIMIT).unwrap();
    let mu = mobius_sieve::mobius_sieve(LIMIT).unwrap();
    let primes = sieve::sieve(LIMIT).unwrap();
    assert_eq!((phi[0], mu[0], phi[1], mu[1]), (0, 0, 1, 1));
    for n in 1..=LIMIT {
        let divisors: Vec<_> = (1..=n).filter(|d| n % d == 0).collect();
        assert_eq!(divisors.iter().map(|&d| phi[d]).sum::<usize>(), n);
        assert_eq!(
            divisors.iter().map(|&d| i64::from(mu[d])).sum::<i64>(),
            i64::from(n == 1)
        );
        assert_eq!(
            divisors
                .iter()
                .map(|&d| i64::from(mu[d]) * (n / d) as i64)
                .sum::<i64>(),
            phi[n] as i64
        );
        assert_eq!(
            phi[n],
            (1..=n)
                .filter(|&k| gcd::gcd(k as u64, n as u64) == 1)
                .count()
        );
        assert_eq!(primes.contains(&n), n >= 2 && phi[n] == n - 1);
        if primes.contains(&n) {
            assert_eq!(mu[n], -1);
        }
        if primes.iter().any(|&p| p <= n / p && n % (p * p) == 0) {
            assert_eq!(mu[n], 0);
        }
    }
}

#[test]
fn fibonacci_join_kitamasa_cassini_and_composite_zero() {
    for modulus in [1_u64, 2, 4, 6, 8, 10, 97, u64::MAX] {
        let mut pair = (0_u64, 1 % modulus);
        for n in 0..=80_u64 {
            let value = fibonacci::fibonacci_mod(n, modulus).unwrap();
            assert_eq!(value, pair.0);
            assert_eq!(
                value,
                linear_recurrence::nth_term(&[0, 1], &[1, 1], n, modulus).unwrap()
            );
            if n > 0 {
                let before = fibonacci::fibonacci_mod(n - 1, modulus).unwrap();
                let after = fibonacci::fibonacci_mod(n + 1, modulus).unwrap();
                let m = u128::from(modulus);
                let product = u128::from(before) * u128::from(after) % m;
                let square = u128::from(value) * u128::from(value) % m;
                let residual = (product + m - square) % m;
                assert_eq!(residual, if n % 2 == 0 { 1 % m } else { (m - 1) % m });
            }
            pair = (
                pair.1,
                ((u128::from(pair.0) + u128::from(pair.1)) % u128::from(modulus)) as u64,
            );
        }
        for n in [1_u64 << 32, (1_u64 << 63) + 7, u64::MAX] {
            assert_eq!(
                fibonacci::fibonacci_mod(n, modulus).unwrap(),
                linear_recurrence::nth_term(&[0, 1], &[1, 1], n, modulus).unwrap()
            );
        }
    }
    assert_eq!(fibonacci::fibonacci_mod(12, 8).unwrap(), 0);
    assert_eq!(fibonacci::fibonacci_mod(3, 2).unwrap(), 0);
    assert!(fibonacci::fibonacci_mod(0, 0).is_err());
    assert!(linear_recurrence::nth_term(&[0, 1], &[1, 1], 0, 0).is_err());
}

#[test]
fn li_chao_join_squared_cost_dp_and_earliest_predecessor() {
    use li_chao_tree::{LiChaoTree, Line};
    let fixtures: &[&[i64]] = &[
        &[0],
        &[0, 0, 0, 0],
        &[3, -2, 3, 1, -2],
        &[-4, -2, 0, 2, 4],
        &[4, 2, 0, -2, -4],
        &[0, 3, 1, 3, 0, 2],
    ];
    for &x in fixtures {
        for penalty in [-3_i64, 0, 2] {
            let mut tree = LiChaoTree::new(x).unwrap();
            let mut dp = vec![0_i128; x.len()];
            assert_eq!(
                tree.insert(Line {
                    slope: -2 * x[0],
                    intercept: x[0] * x[0]
                })
                .unwrap(),
                0
            );
            for i in 1..x.len() {
                let (want, j) = (0..i)
                    .map(|j| {
                        let delta = i128::from(x[i]) - i128::from(x[j]);
                        (dp[j] + delta * delta + i128::from(penalty), j)
                    })
                    .min()
                    .unwrap();
                let minimum = tree.query(x[i]).unwrap();
                dp[i] = minimum.value + i128::from(x[i]) * i128::from(x[i]) + i128::from(penalty);
                assert_eq!((dp[i], minimum.line_id), (want, j));
                let intercept = i64::try_from(dp[i] + i128::from(x[i]) * i128::from(x[i])).unwrap();
                assert_eq!(
                    tree.insert(Line {
                        slope: -2 * x[i],
                        intercept
                    })
                    .unwrap(),
                    i
                );
            }
        }
    }
}

#[test]
fn offline_connectivity_join_rebuilt_dsu_and_literal_timeline() {
    let literal = [
        Operation::Components,
        Operation::Add(0, 1),
        Operation::Add(1, 2),
        Operation::Connected(0, 2),
        Operation::Remove(1, 0),
        Operation::Connected(0, 2),
        Operation::Components,
        Operation::Add(0, 0),
        Operation::Components,
        Operation::Remove(0, 0),
        Operation::Add(0, 1),
        Operation::Connected(0, 2),
    ];
    assert_eq!(
        offline_dynamic_connectivity::solve(4, &literal).unwrap(),
        vec![
            Answer::Components(4),
            Answer::Connected(true),
            Answer::Connected(false),
            Answer::Components(3),
            Answer::Components(3),
            Answer::Connected(true)
        ]
    );
    for n in 1..=6 {
        for phase in 0..8 {
            let mut operations = Vec::new();
            let mut expected = Vec::new();
            let mut active = BTreeSet::new();
            for step in 0..48 {
                let a = (step * 5 + phase) % n;
                let b = (step * step + phase * 3 + 1) % n;
                let edge = (a.min(b), a.max(b));
                if active.remove(&edge) {
                    operations.push(Operation::Remove(b, a));
                } else {
                    active.insert(edge);
                    operations.push(Operation::Add(a, b));
                }
                let mut dsu = DisjointSet::new(n);
                for &(u, v) in &active {
                    dsu.union(u, v).unwrap();
                }
                operations.push(Operation::Components);
                expected.push(Answer::Components(dsu.components()));
                for u in 0..n {
                    for v in 0..n {
                        operations.push(Operation::Connected(u, v));
                        expected.push(Answer::Connected(
                            dsu.find(u).unwrap() == dsu.find(v).unwrap(),
                        ));
                    }
                }
            }
            assert_eq!(
                offline_dynamic_connectivity::solve(n, &operations).unwrap(),
                expected
            );
        }
    }
    assert_eq!(
        offline_dynamic_connectivity::solve(0, &[Operation::Components]).unwrap(),
        vec![Answer::Components(0)]
    );
    assert!(
        offline_dynamic_connectivity::solve(2, &[Operation::Add(0, 1), Operation::Add(1, 0)])
            .is_err()
    );
    assert!(offline_dynamic_connectivity::solve(2, &[Operation::Remove(0, 1)]).is_err());
}
