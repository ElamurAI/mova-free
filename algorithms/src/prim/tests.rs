use super::*;
fn components(n: usize, edges: &[Edge], mask: usize) -> usize {
    let mut seen = vec![false; n];
    let mut count = 0;
    for root in 0..n {
        if seen[root] {
            continue;
        }
        count += 1;
        let mut pending = vec![root];
        while let Some(u) = pending.pop() {
            if seen[u] {
                continue;
            }
            seen[u] = true;
            for (i, e) in edges.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    if e.u == u {
                        pending.push(e.v);
                    }
                    if e.v == u {
                        pending.push(e.u);
                    }
                }
            }
        }
    }
    count
}
fn brute(n: usize, edges: &[Edge]) -> (u128, usize) {
    let c = components(n, edges, (1 << edges.len()) - 1);
    let mut best = u128::MAX;
    for mask in 0..1usize << edges.len() {
        if mask.count_ones() as usize != n - c || components(n, edges, mask) != c {
            continue;
        }
        let weight = edges
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, e)| u128::from(e.weight))
            .sum();
        best = best.min(weight);
    }
    (best, c)
}
#[test]
fn exhaustive_forests() {
    for n in 0..=4usize {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mut code in 0..3usize.pow(pairs.len() as u32) {
            let mut edges = vec![];
            for &(u, v) in &pairs {
                if code % 3 != 0 {
                    edges.push(Edge {
                        u,
                        v,
                        weight: if code % 3 == 1 { 0 } else { 3 },
                    });
                }
                code /= 3;
            }
            let got = minimum_spanning_forest(n, &edges).unwrap();
            assert_eq!((got.total_weight, got.components), brute(n, &edges));
            assert_eq!(got.edge_indices.len(), n - got.components);
            let selected: Vec<_> = got.edge_indices.iter().map(|&i| edges[i]).collect();
            assert_eq!(
                components(n, &selected, (1 << selected.len()) - 1),
                got.components
            );
            let other = crate::kruskal::minimum_spanning_forest(n, &edges).unwrap();
            assert_eq!(
                (got.total_weight, got.components),
                (other.total_weight, other.components)
            );
        }
    }
}
#[test]
fn ties_parallel_loops_and_wide_sum() {
    let e = vec![
        Edge {
            u: 0,
            v: 0,
            weight: 0,
        },
        Edge {
            u: 1,
            v: 0,
            weight: u64::MAX,
        },
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
    let f = minimum_spanning_forest(4, &e).unwrap();
    assert_eq!(f.edge_indices, vec![1, 3]);
    assert_eq!(f.total_weight, 2 * u128::from(u64::MAX));
    assert_eq!(f.components, 2);
}
#[test]
fn validation_and_empty() {
    assert_eq!(minimum_spanning_forest(0, &[]).unwrap().components, 0);
    assert_eq!(
        minimum_spanning_forest(
            2,
            &[Edge {
                u: 3,
                v: 4,
                weight: 0
            }]
        ),
        Err(Error::InvalidEndpoint {
            edge: 0,
            vertex: 3,
            vertices: 2
        })
    );
    assert_eq!(
        minimum_spanning_forest(1_000_001, &[]),
        Err(Error::ResourceLimit)
    );
    assert_eq!(
        minimum_spanning_forest(
            0,
            &vec![
                Edge {
                    u: 0,
                    v: 0,
                    weight: 0
                };
                1_000_001
            ]
        ),
        Err(Error::ResourceLimit)
    );
}
#[test]
fn orientation_scaling_and_redundant_loop() {
    let e = vec![
        Edge {
            u: 0,
            v: 1,
            weight: 4,
        },
        Edge {
            u: 0,
            v: 2,
            weight: 2,
        },
        Edge {
            u: 1,
            v: 2,
            weight: 1,
        },
    ];
    let a = minimum_spanning_forest(3, &e).unwrap();
    let changed: Vec<_> = e
        .iter()
        .map(|e| Edge {
            u: e.v,
            v: e.u,
            weight: e.weight * 7,
        })
        .collect();
    let b = minimum_spanning_forest(3, &changed).unwrap();
    assert_eq!(a.edge_indices, b.edge_indices);
    assert_eq!(b.total_weight, 7 * a.total_weight);
    let mut extra = e;
    extra.push(Edge {
        u: 2,
        v: 2,
        weight: 0,
    });
    assert_eq!(minimum_spanning_forest(3, &extra).unwrap(), a);
}
