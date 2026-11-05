//! Exact bounded joins preserve graph, byte, index, preference and residue identities.
use mova_algorithms::{
    bipartite_vertex_cover, burrows_wheeler, cartesian_tree, dinic, hopcroft_karp, inversion_count,
    kmp, lowest_common_ancestor::Lca, max_flow, merge_sort, miller_rabin, minimal_rotation,
    modular_power, shift_and, sieve, sparse_table::SparseTable, stable_marriage, subset_mobius,
    subset_zeta, tonelli_shanks,
};
fn words(max: u32, alphabet: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..alphabet.pow(n)).map(move |mut x| {
                (0..n)
                    .map(|_| {
                        let b = (x % alphabet) as u8;
                        x /= alphabet;
                        b
                    })
                    .collect()
            })
        })
        .collect()
}
#[test]
fn cover_and_matching_share_the_same_flow_network() {
    for mask in 0..512 {
        let g: Vec<Vec<usize>> = (0..3)
            .map(|u| (0..3).filter(|v| mask & (1 << (3 * u + v)) != 0).collect())
            .collect();
        let c = bipartite_vertex_cover::minimum_vertex_cover(&g, 3).unwrap();
        assert!(g.iter().enumerate().all(|(u, vs)| {
            vs.iter()
                .all(|v| c.left.contains(&u) || c.right.contains(v))
        }));
        let matched = hopcroft_karp::maximum_matching(&g, 3)
            .unwrap()
            .iter()
            .flatten()
            .count();
        let mut edges = vec![];
        for (u, vs) in g.iter().enumerate() {
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
            for &v in vs {
                edges.push(max_flow::Edge {
                    from: u,
                    to: 3 + v,
                    capacity: 1,
                });
            }
        }
        let f = dinic::max_flow(8, &edges, 6, 7).unwrap();
        assert_eq!(c.left.len() + c.right.len(), matched);
        assert_eq!(f.value, matched as u128);
        let cut = edges
            .iter()
            .filter(|e| f.source_side[e.from] && !f.source_side[e.to])
            .map(|e| u128::from(e.capacity))
            .sum::<u128>();
        assert_eq!(cut, f.value);
    }
}
fn permutations(v: &mut [usize], at: usize, out: &mut Vec<Vec<usize>>) {
    if at == v.len() {
        out.push(v.to_vec());
        return;
    }
    for i in at..v.len() {
        v.swap(at, i);
        permutations(v, at + 1, out);
        v.swap(at, i);
    }
}
#[test]
fn common_preferences_transport_priority_to_inverse_permutation() {
    for n in 0..=5 {
        let mut ps = vec![];
        permutations(&mut (0..n).collect::<Vec<_>>(), 0, &mut ps);
        for priority in ps {
            let p = vec![(0..n).collect::<Vec<_>>(); n];
            let r = vec![priority.clone(); n];
            let matched = stable_marriage::stable_matching(&p, &r).unwrap();
            for (receiver, &proposer) in priority.iter().enumerate() {
                assert_eq!(matched[proposer], receiver);
            }
            assert_eq!(
                inversion_count::count_inversions(&matched).unwrap(),
                inversion_count::count_inversions(&priority).unwrap()
            );
        }
    }
}
#[test]
fn prime_gate_and_canonical_root_preserve_modular_squares() {
    let primes = sieve::sieve(100).unwrap();
    for p in 0..=100u64 {
        assert_eq!(miller_rabin::is_prime(p), primes.contains(&(p as usize)));
        if !miller_rabin::is_prime(p) {
            assert!(tonelli_shanks::sqrt_mod(1, p).is_err());
            continue;
        }
        for a in 0..p {
            let got = tonelli_shanks::sqrt_mod(a, p).unwrap();
            let want =
                (0..p).find(|r| (u128::from(*r) * u128::from(*r)) % u128::from(p) == u128::from(a));
            assert_eq!(got, want);
            if let Some(r) = got {
                assert_eq!(modular_power::modular_power(r, 2, p).unwrap(), a);
                assert!(r <= p - r);
            }
        }
    }
}
#[test]
fn bit_parallel_matches_keep_kmp_byte_boundaries() {
    for text in words(6, 2) {
        for pattern in words(4, 2) {
            let got = shift_and::find_all(&text, &pattern).unwrap();
            assert_eq!(got, kmp::find_all(&text, &pattern));
            let expected: Vec<_> = (0..=text.len())
                .filter(|&i| text[i..].starts_with(&pattern))
                .collect();
            assert_eq!(got, expected);
        }
    }
    let text = vec![255; 66];
    let pattern = vec![255; 64];
    assert_eq!(shift_and::find_all(&text, &pattern).unwrap(), vec![0, 1, 2]);
    assert!(shift_and::find_all(&text, &[255; 65]).is_err());
}
#[test]
fn minimal_rotation_preserves_bwt_column_and_earliest_tie() {
    for text in words(6, 3) {
        let k = minimal_rotation::minimal_rotation(&text);
        let chosen: Vec<_> = text[k..].iter().chain(&text[..k]).copied().collect();
        let (last, _) = burrows_wheeler::transform(&text);
        let first = burrows_wheeler::inverse(&last, 0).unwrap();
        assert_eq!(chosen, first);
        let first_equal = if text.is_empty() {
            0
        } else {
            (0..text.len())
                .find(|&i| {
                    text[i..]
                        .iter()
                        .chain(&text[..i])
                        .copied()
                        .eq(chosen.iter().copied())
                })
                .unwrap()
        };
        assert_eq!(k, first_equal);
        assert_eq!(burrows_wheeler::transform(&chosen).0, last);
    }
}
#[test]
fn cartesian_parent_lca_recovers_leftmost_range_minima() {
    for values in words(6, 3) {
        let tree = cartesian_tree::min_cartesian_tree(&values).unwrap();
        let lca = Lca::new(&tree.parent).unwrap();
        let sparse = SparseTable::from_slice(&values);
        for l in 0..values.len() {
            for r in l + 1..=values.len() {
                let i = lca.query(l, r - 1).unwrap().unwrap();
                let expected = (l..r).min_by_key(|&j| (values[j], j)).unwrap();
                assert_eq!(i, expected);
                assert_eq!(sparse.query(l, r).unwrap(), &values[i]);
            }
        }
    }
}
#[test]
fn inversion_observations_preserve_sort_and_duplicate_reversal() {
    for values in words(6, 3) {
        let inv = inversion_count::count_inversions(&values).unwrap();
        let reversed = values.iter().rev().copied().collect::<Vec<_>>();
        let opposite = inversion_count::count_inversions(&reversed).unwrap();
        let unequal = (0..values.len())
            .map(|i| {
                (i + 1..values.len())
                    .filter(|&j| values[i] != values[j])
                    .count() as u128
            })
            .sum::<u128>();
        assert_eq!(inv + opposite, unequal);
        let mut ordered = values.clone();
        merge_sort::merge_sort(&mut ordered);
        assert_eq!(inversion_count::count_inversions(&ordered).unwrap(), 0);
    }
}
#[test]
fn subset_inverse_preserves_normalized_residues() {
    for modulus in [1, 2, 6, 97, u64::MAX] {
        for bits in 0..=5 {
            let original: Vec<u64> = (0..1usize << bits)
                .map(|i| u64::MAX.wrapping_sub(i as u64 * 17))
                .collect();
            let mut v = original.clone();
            subset_zeta::transform(&mut v, modulus).unwrap();
            for (mask, &got) in v.iter().enumerate() {
                let mut want = 0u128;
                for (s, &x) in original.iter().enumerate() {
                    if s & mask == s {
                        want = (want + u128::from(x)) % u128::from(modulus);
                    }
                }
                assert_eq!(got, want as u64);
            }
            subset_mobius::transform(&mut v, modulus).unwrap();
            assert_eq!(v, original.iter().map(|x| x % modulus).collect::<Vec<_>>());
        }
    }
}
#[test]
fn zeta_product_inverts_to_or_convolution() {
    for modulus in [1, 6, 97, u64::MAX] {
        for seed in 0..16u64 {
            let a: Vec<_> = (0..8).map(|i| (seed + 3 * i) % 11).collect();
            let b: Vec<_> = (0..8).map(|i| (2 * seed + 5 * i) % 13).collect();
            let (mut za, mut zb) = (a.clone(), b.clone());
            subset_zeta::transform(&mut za, modulus).unwrap();
            subset_zeta::transform(&mut zb, modulus).unwrap();
            for (x, y) in za.iter_mut().zip(zb) {
                *x = ((u128::from(*x) * u128::from(y)) % u128::from(modulus)) as u64;
            }
            subset_mobius::transform(&mut za, modulus).unwrap();
            let mut expected = vec![0u64; 8];
            for (i, &x) in a.iter().enumerate() {
                for (j, &y) in b.iter().enumerate() {
                    let k = i | j;
                    expected[k] = ((u128::from(expected[k]) + u128::from(x) * u128::from(y))
                        % u128::from(modulus)) as u64;
                }
            }
            assert_eq!(za, expected);
        }
    }
}
