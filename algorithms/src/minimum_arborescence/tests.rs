use super::*;
fn oracle(n: usize, root: usize, edges: &[Edge]) -> Option<i128> {
    let mut best = None;
    for mask in 0..1usize << edges.len() {
        if mask.count_ones() as usize != n - 1 {
            continue;
        }
        let mut incoming = vec![0; n];
        let mut sum = 0i128;
        for (i, e) in edges.iter().enumerate() {
            if mask & (1 << i) != 0 {
                incoming[e.to] += 1;
                sum += i128::from(e.weight);
            }
        }
        if incoming[root] != 0 || (0..n).any(|v| v != root && incoming[v] != 1) {
            continue;
        }
        let mut reach = vec![false; n];
        reach[root] = true;
        for _ in 0..n {
            for (i, e) in edges.iter().enumerate() {
                if mask & (1 << i) != 0 && reach[e.from] {
                    reach[e.to] = true;
                }
            }
        }
        if reach.iter().all(|&x| x) {
            best = Some(best.map_or(sum, |b: i128| b.min(sum)));
        }
    }
    best
}
#[test]
fn exhaustive_directed_graphs() {
    let n = 4;
    let pairs: Vec<_> = (0..n)
        .flat_map(|u| (0..n).filter(move |&v| u != v).map(move |v| (u, v)))
        .collect();
    for bits in 0..1usize << pairs.len() {
        let e: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter_map(|(i, &(from, to))| {
                (bits & (1 << i) != 0).then_some(Edge {
                    from,
                    to,
                    weight: (i % 5) as i64 - 2,
                })
            })
            .collect();
        for root in 0..n {
            assert_eq!(
                minimum_arborescence(n, root, &e).unwrap(),
                oracle(n, root, &e)
            );
        }
    }
}
#[test]
fn contractions_extremes_errors() {
    let e = [
        Edge {
            from: 0,
            to: 1,
            weight: 5,
        },
        Edge {
            from: 0,
            to: 2,
            weight: 7,
        },
        Edge {
            from: 1,
            to: 2,
            weight: -2,
        },
        Edge {
            from: 2,
            to: 1,
            weight: -3,
        },
    ];
    assert_eq!(minimum_arborescence(3, 0, &e), Ok(Some(3)));
    assert_eq!(minimum_arborescence(3, 0, &e[2..]), Ok(None));
    assert_eq!(minimum_arborescence(1, 0, &[]), Ok(Some(0)));
    assert_eq!(minimum_arborescence(0, 0, &[]), Err(Error::InvalidRoot));
    assert_eq!(
        minimum_arborescence(MAX_VERTICES + 1, 0, &[]),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        minimum_arborescence(1, 0, &e),
        Err(Error::InvalidEdge { index: 0 })
    );
    let e = [
        Edge {
            from: 0,
            to: 1,
            weight: i64::MIN,
        },
        Edge {
            from: 1,
            to: 2,
            weight: i64::MIN,
        },
    ];
    assert_eq!(
        minimum_arborescence(3, 0, &e),
        Ok(Some(2 * i128::from(i64::MIN)))
    );
}

#[test]
fn completed_traversals_are_not_rewalked() {
    for n in [1, 2, 8, 32, 128, 512] {
        let parent: Vec<_> = (0..n).map(|v: usize| v.saturating_sub(1)).collect();
        let (ids, count, visits) = cycle_components(&parent, 0);
        assert_eq!(visits, n - 1, "each nonroot vertex advances exactly once");
        assert_eq!(count, 0);
        assert!(ids.iter().all(|&id| id == usize::MAX));
    }
}

#[test]
fn exhaustive_cycle_scan_against_independent_walks() {
    for n in 1..=5usize {
        for mut code in 0..n.pow((n - 1) as u32) {
            let mut parent = vec![0; n];
            for p in parent.iter_mut().skip(1) {
                *p = code % n;
                code /= n;
            }
            let (ids, count, visits) = cycle_components(&parent, 0);
            assert_eq!(visits, n - 1);
            let mut cycles = std::collections::BTreeSet::new();
            for start in 1..n {
                let mut path = Vec::new();
                let mut v = start;
                while v != 0 && !path.contains(&v) {
                    path.push(v);
                    v = parent[v];
                }
                if v == 0 {
                    assert_eq!(ids[start], usize::MAX);
                    continue;
                }
                let begin = path.iter().position(|&x| x == v).unwrap();
                let mut cycle = path[begin..].to_vec();
                cycle.sort_unstable();
                cycles.insert(cycle.clone());
                if cycle.contains(&start) {
                    assert_ne!(ids[start], usize::MAX);
                    for &x in &cycle {
                        assert_eq!(ids[start], ids[x]);
                    }
                } else {
                    assert_eq!(ids[start], usize::MAX);
                }
            }
            assert_eq!(count, cycles.len());
            let labels: std::collections::BTreeSet<_> = cycles.iter().map(|c| ids[c[0]]).collect();
            assert_eq!(labels.len(), count);
        }
    }
}

fn nested_family(k: usize) -> Vec<Edge> {
    let mut edges = vec![Edge {
        from: 0,
        to: 1,
        weight: k as i64,
    }];
    for i in 2..=k {
        edges.push(Edge {
            from: 1,
            to: i,
            weight: 0,
        });
        edges.push(Edge {
            from: i,
            to: 1,
            weight: i as i64 - 2,
        });
    }
    edges.push(Edge {
        from: 0,
        to: k + 1,
        weight: 0,
    });
    for i in k + 2..2 * k + 1 {
        edges.push(Edge {
            from: i - 1,
            to: i,
            weight: 0,
        });
    }
    edges
}

#[test]
fn nested_contractions_and_root_reaching_tail() {
    // Every feasible tree needs the sole root-to-star edge of cost k.
    // That edge plus all zero-cost star/tail edges is an independent witness.
    for k in [2, 3, 4, 8, 16, 32, 64, 128] {
        let edges = nested_family(k);
        assert_eq!(
            minimum_arborescence(2 * k + 1, 0, &edges),
            Ok(Some(k as i128))
        );
        if k <= 4 {
            assert_eq!(oracle(2 * k + 1, 0, &edges), Some(k as i128));
        }
    }
}
