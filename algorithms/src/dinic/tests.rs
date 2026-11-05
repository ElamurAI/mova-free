use super::*;

// Independent cut enumeration, without residual search.
fn optimum(n: usize, edges: &[Edge], s: usize, t: usize) -> u128 {
    (0..1usize << n)
        .filter(|&set| set >> s & 1 == 1 && set >> t & 1 == 0)
        .map(|set| {
            edges
                .iter()
                .filter(|e| set >> e.from & 1 == 1 && set >> e.to & 1 == 0)
                .map(|e| u128::from(e.capacity))
                .sum()
        })
        .min()
        .unwrap()
}
fn verify(n: usize, edges: &[Edge], s: usize, t: usize, flow: &Flow) {
    assert_eq!(flow.edge_flows.len(), edges.len());
    assert_eq!(flow.source_side.len(), n);
    let mut balance = vec![0i128; n];
    let mut reachable = vec![false; n];
    reachable[s] = true;
    for (edge, &amount) in edges.iter().zip(&flow.edge_flows) {
        assert!(amount <= edge.capacity);
        if edge.from == edge.to {
            assert_eq!(amount, 0);
        }
        balance[edge.from] -= i128::from(amount);
        balance[edge.to] += i128::from(amount);
    }
    for _ in 0..n {
        for (edge, &amount) in edges.iter().zip(&flow.edge_flows) {
            if reachable[edge.from] && amount < edge.capacity {
                reachable[edge.to] = true;
            }
            if reachable[edge.to] && amount > 0 {
                reachable[edge.from] = true;
            }
        }
    }
    assert_eq!(reachable, flow.source_side);
    assert!(!reachable[t]);
    for (v, &net) in balance.iter().enumerate() {
        assert_eq!(
            net,
            if v == s {
                -(flow.value as i128)
            } else if v == t {
                flow.value as i128
            } else {
                0
            }
        );
    }
    let cut: u128 = edges
        .iter()
        .filter(|e| reachable[e.from] && !reachable[e.to])
        .map(|e| u128::from(e.capacity))
        .sum();
    assert_eq!(cut, flow.value);
}

#[test]
fn exhaustive_weighted_cuts() {
    let slots = [(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)];
    for mut code in 0..3usize.pow(6) {
        let edges: Vec<_> = slots
            .iter()
            .map(|&(from, to)| {
                let capacity = (code % 3) as u64;
                code /= 3;
                Edge { from, to, capacity }
            })
            .collect();
        for (s, t) in [(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)] {
            let output = max_flow(3, &edges, s, t).unwrap();
            assert_eq!(output.value, optimum(3, &edges, s, t));
            verify(3, &edges, s, t, &output);
            assert_eq!(
                output.value,
                crate::max_flow::max_flow(3, &edges, s, t).unwrap().value
            );
        }
    }
}
#[test]
fn exhaustive_four_vertex_topologies() {
    let slots: Vec<_> = (0..4)
        .flat_map(|u| (0..4).filter(move |&v| u != v).map(move |v| (u, v)))
        .collect();
    for mask in 0..1usize << slots.len() {
        let edges: Vec<_> = slots
            .iter()
            .enumerate()
            .filter(|(i, _)| mask >> i & 1 == 1)
            .map(|(_, &(from, to))| Edge {
                from,
                to,
                capacity: 1,
            })
            .collect();
        let flow = max_flow(4, &edges, 0, 3).unwrap();
        assert_eq!(flow.value, optimum(4, &edges, 0, 3));
        verify(4, &edges, 0, 3, &flow);
    }
}
#[test]
fn cancellation_wide_arithmetic_and_scaling() {
    let edges: Vec<_> = [
        (0, 1),
        (0, 2),
        (1, 3),
        (1, 4),
        (2, 3),
        (3, 5),
        (4, 5),
        (1, 1),
        (3, 1),
    ]
    .into_iter()
    .map(|(from, to)| Edge {
        from,
        to,
        capacity: 1,
    })
    .collect();
    let output = max_flow(6, &edges, 0, 5).unwrap();
    assert_eq!(output.value, 2);
    assert_eq!(output.edge_flows[2], 0);
    verify(6, &edges, 0, 5, &output);
    assert_eq!(max_flow(6, &edges, 0, 5).unwrap(), output);
    let wide: Vec<_> = edges
        .iter()
        .map(|e| Edge {
            capacity: u64::MAX,
            ..*e
        })
        .collect();
    let scaled = max_flow(6, &wide, 0, 5).unwrap();
    assert_eq!(scaled.value, 2 * u128::from(u64::MAX));
    verify(6, &wide, 0, 5, &scaled);
    let parallel = [Edge {
        from: 0,
        to: 1,
        capacity: u64::MAX,
    }; 3];
    assert_eq!(
        max_flow(2, &parallel, 0, 1).unwrap().value,
        3 * u128::from(u64::MAX)
    );
}
#[test]
fn validation_precedence_and_limits() {
    assert_eq!(
        max_flow(MAX_VERTICES + 1, &[], usize::MAX, 0),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        max_flow(0, &[], 0, 0),
        Err(Error::InvalidVertex {
            vertex: 0,
            vertices: 0
        })
    );
    assert_eq!(
        max_flow(2, &[], 1, 2),
        Err(Error::InvalidVertex {
            vertex: 2,
            vertices: 2
        })
    );
    let bad = [Edge {
        from: 9,
        to: 9,
        capacity: 0,
    }];
    assert_eq!(max_flow(2, &bad, 0, 0), Err(Error::SameTerminal));
    assert_eq!(
        max_flow(2, &bad, 0, 1),
        Err(Error::InvalidEdge { index: 0 })
    );
    assert_eq!(
        max_flow(2, &vec![bad[0]; MAX_EDGES + 1], 0, 0),
        Err(Error::LimitExceeded)
    );
    let empty = max_flow(2, &[], 0, 1).unwrap();
    assert_eq!(
        empty,
        Flow {
            value: 0,
            edge_flows: vec![],
            source_side: vec![true, false]
        }
    );
}
#[test]
fn long_path_and_ambiguous_value_fiber() {
    let n = 20_000;
    let edges: Vec<_> = (1..n)
        .map(|to| Edge {
            from: to - 1,
            to,
            capacity: 7,
        })
        .collect();
    let flow = max_flow(n, &edges, 0, n - 1).unwrap();
    assert_eq!(flow.value, 7);
    assert!(flow.edge_flows.iter().all(|&f| f == 7));
    for spare in 1..8 {
        let a = [
            Edge {
                from: 0,
                to: 1,
                capacity: 1,
            },
            Edge {
                from: 1,
                to: 2,
                capacity: spare,
            },
        ];
        assert_eq!(max_flow(3, &a, 0, 2).unwrap().value, 1);
    }
}
