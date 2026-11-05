use super::*;

fn reachability(n: usize, edges: &[Edge], indices: &[usize]) -> Vec<Vec<bool>> {
    let mut reach = vec![vec![false; n]; n];
    for (v, row) in reach.iter_mut().enumerate() {
        row[v] = true;
    }
    for &i in indices {
        let e = edges[i];
        reach[e.u][e.v] = true;
        reach[e.v][e.u] = true;
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                reach[u][v] |= reach[u][k] && reach[k][v];
            }
        }
    }
    reach
}

fn verify_with_subsets(n: usize, edges: &[Edge]) {
    let full = reachability(n, edges, &(0..edges.len()).collect::<Vec<_>>());
    let components = (0..n).filter(|&v| (0..v).all(|u| !full[u][v])).count();
    let rank = n - components;
    let mut minimum = None;
    for mask in 0usize..1 << edges.len() {
        if mask.count_ones() as usize != rank {
            continue;
        }
        let indices: Vec<_> = (0..edges.len()).filter(|&i| mask & (1 << i) != 0).collect();
        if reachability(n, edges, &indices) == full {
            let cost: u128 = indices.iter().map(|&i| u128::from(edges[i].weight)).sum();
            minimum = Some(minimum.map_or(cost, |old: u128| old.min(cost)));
        }
    }
    let before = edges.to_vec();
    let actual = minimum_spanning_forest(n, edges).unwrap();
    assert_eq!(actual.components, components);
    assert_eq!(actual.total_weight, minimum.unwrap());
    assert_eq!(actual.edge_indices.len(), rank);
    let unique: std::collections::BTreeSet<_> = actual.edge_indices.iter().copied().collect();
    assert_eq!(unique.len(), rank);
    assert!(actual.edge_indices.iter().all(|&i| i < edges.len()));
    assert_eq!(reachability(n, edges, &actual.edge_indices), full);
    assert_eq!(
        actual.total_weight,
        actual
            .edge_indices
            .iter()
            .map(|&i| u128::from(edges[i].weight))
            .sum()
    );
    assert_eq!(edges, before);
}

#[test]
fn boundaries_ties_parallel_edges_and_wide_costs() {
    assert_eq!(
        minimum_spanning_forest(0, &[]),
        Ok(Forest {
            edge_indices: vec![],
            total_weight: 0,
            components: 0
        })
    );
    assert_eq!(
        minimum_spanning_forest(3, &[]),
        Ok(Forest {
            edge_indices: vec![],
            total_weight: 0,
            components: 3
        })
    );
    assert_eq!(
        minimum_spanning_forest(
            2,
            &[
                Edge {
                    u: 0,
                    v: 1,
                    weight: 0
                },
                Edge {
                    u: 2,
                    v: 9,
                    weight: 0
                }
            ]
        ),
        Err(Error::InvalidEndpoint {
            edge: 1,
            vertex: 2,
            vertices: 2
        })
    );
    let edges = [
        Edge {
            u: 2,
            v: 1,
            weight: 5,
        },
        Edge {
            u: 1,
            v: 0,
            weight: 5,
        },
        Edge {
            u: 0,
            v: 1,
            weight: 5,
        },
        Edge {
            u: 0,
            v: 2,
            weight: 5,
        },
        Edge {
            u: 0,
            v: 0,
            weight: 0,
        },
    ];
    assert_eq!(
        minimum_spanning_forest(4, &edges),
        Ok(Forest {
            edge_indices: vec![1, 3],
            total_weight: 10,
            components: 2
        })
    );
    let wide = [
        Edge {
            u: 0,
            v: 1,
            weight: u64::MAX,
        },
        Edge {
            u: 1,
            v: 2,
            weight: u64::MAX,
        },
    ];
    assert_eq!(
        minimum_spanning_forest(3, &wide).unwrap().total_weight,
        2 * u128::from(u64::MAX)
    );
}

#[test]
fn all_small_weighted_graphs_match_subset_oracle() {
    for n in 0..=4 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for code in 0usize..3usize.pow(pairs.len() as u32) {
            let mut digits = code;
            let mut edges = vec![];
            for &(u, v) in &pairs {
                match digits % 3 {
                    1 => edges.push(Edge { u, v, weight: 0 }),
                    2 => edges.push(Edge { u, v, weight: 3 }),
                    _ => {}
                }
                digits /= 3;
            }
            // Context is emitted before a failing helper assertion.
            let result = std::panic::catch_unwind(|| verify_with_subsets(n, &edges));
            assert!(result.is_ok(), "n={n},code={code}");
        }
    }
}

#[test]
fn seeded_multigraphs_match_subset_oracle() {
    const SEED: u64 = 0x6b7520261029;
    let mut state = SEED;
    for case in 0..40 {
        let n = 2 + case % 5;
        let mut edges = vec![];
        for _ in 0..8 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let u = (state % n as u64) as usize;
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let v = (state % n as u64) as usize;
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            edges.push(Edge {
                u,
                v,
                weight: if state.is_multiple_of(4) {
                    u64::MAX
                } else {
                    state
                },
            });
        }
        let result = std::panic::catch_unwind(|| verify_with_subsets(n, &edges));
        assert!(result.is_ok(), "seed={SEED:#x},case={case}");
    }
}

#[test]
fn endpoint_reversal_and_positive_scaling_preserve_choices() {
    let edges = [
        Edge {
            u: 0,
            v: 1,
            weight: 7,
        },
        Edge {
            u: 1,
            v: 2,
            weight: 2,
        },
        Edge {
            u: 0,
            v: 2,
            weight: 4,
        },
        Edge {
            u: 3,
            v: 4,
            weight: 1,
        },
    ];
    let original = minimum_spanning_forest(6, &edges).unwrap();
    let transformed: Vec<_> = edges
        .iter()
        .map(|e| Edge {
            u: e.v,
            v: e.u,
            weight: e.weight * 11,
        })
        .collect();
    let altered = minimum_spanning_forest(6, &transformed).unwrap();
    assert_eq!(altered.edge_indices, original.edge_indices);
    assert_eq!(altered.total_weight, original.total_weight * 11);
    assert_eq!(altered.components, original.components);
    verify_with_subsets(6, &transformed);
}
