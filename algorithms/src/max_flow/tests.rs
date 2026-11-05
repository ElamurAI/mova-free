use super::*;

fn cut_oracle(n: usize, edges: &[Edge], source: usize, sink: usize) -> u128 {
    (0usize..1 << n)
        .filter(|mask| mask & (1 << source) != 0 && mask & (1 << sink) == 0)
        .map(|mask| {
            edges
                .iter()
                .filter(|e| mask & (1 << e.from) != 0 && mask & (1 << e.to) == 0)
                .map(|e| u128::from(e.capacity))
                .sum()
        })
        .min()
        .unwrap()
}

fn certificate(n: usize, edges: &[Edge], source: usize, sink: usize, flow: &Flow) {
    assert_eq!(flow.edge_flows.len(), edges.len());
    assert_eq!(flow.source_side.len(), n);
    assert!(flow.source_side[source]);
    assert!(!flow.source_side[sink]);
    let mut incoming = vec![0u128; n];
    let mut outgoing = vec![0u128; n];
    let mut cut = 0;
    for (e, &f) in edges.iter().zip(&flow.edge_flows) {
        assert!(f <= e.capacity);
        incoming[e.to] += u128::from(f);
        outgoing[e.from] += u128::from(f);
        if flow.source_side[e.from] && !flow.source_side[e.to] {
            cut += u128::from(e.capacity);
            assert_eq!(f, e.capacity);
        }
        if !flow.source_side[e.from] && flow.source_side[e.to] {
            assert_eq!(f, 0);
        }
        if e.from == e.to {
            assert_eq!(f, 0);
        }
    }
    for v in 0..n {
        if v == source {
            assert_eq!(outgoing[v], incoming[v] + flow.value);
        } else if v == sink {
            assert_eq!(incoming[v], outgoing[v] + flow.value);
        } else {
            assert_eq!(incoming[v], outgoing[v]);
        }
    }
    assert_eq!(cut, flow.value);
}

#[test]
fn exhaustive_cuts_and_certificates() {
    let positions: Vec<_> = (0..4)
        .flat_map(|from| {
            (0..4)
                .filter(move |&to| from != to)
                .map(move |to| (from, to))
        })
        .collect();
    for mask in 0usize..1 << positions.len() {
        let edges: Vec<_> = positions
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &(from, to))| Edge {
                from,
                to,
                capacity: 1,
            })
            .collect();
        let flow = max_flow(4, &edges, 0, 3).unwrap();
        assert_eq!(flow.value, cut_oracle(4, &edges, 0, 3));
        certificate(4, &edges, 0, 3, &flow);
        let reversed: Vec<_> = edges
            .iter()
            .map(|e| Edge {
                from: e.to,
                to: e.from,
                capacity: e.capacity,
            })
            .collect();
        assert_eq!(max_flow(4, &reversed, 3, 0).unwrap().value, flow.value);
    }
}

#[test]
fn capacity_boundary_parallel_loops_and_scaling() {
    let edges = [
        Edge {
            from: 0,
            to: 0,
            capacity: u64::MAX,
        },
        Edge {
            from: 0,
            to: 1,
            capacity: u64::MAX,
        },
        Edge {
            from: 1,
            to: 0,
            capacity: u64::MAX,
        },
        Edge {
            from: 0,
            to: 1,
            capacity: u64::MAX,
        },
    ];
    let flow = max_flow(2, &edges, 0, 1).unwrap();
    assert_eq!(flow.value, 2 * u128::from(u64::MAX));
    assert_eq!(flow.edge_flows, [0, u64::MAX, 0, u64::MAX]);
    certificate(2, &edges, 0, 1, &flow);
    for a in 0..5 {
        for b in 0..5 {
            let edges = [
                Edge {
                    from: 0,
                    to: 1,
                    capacity: a,
                },
                Edge {
                    from: 1,
                    to: 2,
                    capacity: b,
                },
            ];
            let flow = max_flow(3, &edges, 0, 2).unwrap();
            assert_eq!(flow.value, u128::from(a.min(b)));
            let scaled: Vec<_> = edges
                .iter()
                .map(|e| Edge {
                    capacity: e.capacity * 3,
                    ..*e
                })
                .collect();
            assert_eq!(max_flow(3, &scaled, 0, 2).unwrap().value, flow.value * 3);
        }
    }
}

#[test]
fn residual_cancellation_and_preimage() {
    let edges: Vec<_> = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 3), (3, 5), (4, 5)]
        .into_iter()
        .map(|(from, to)| Edge {
            from,
            to,
            capacity: 1,
        })
        .collect();
    let flow = max_flow(6, &edges, 0, 5).unwrap();
    assert_eq!(flow.value, 2);
    assert_eq!(flow.edge_flows[2], 0);
    certificate(6, &edges, 0, 5, &flow);
    for capacity in 0..10 {
        let direct = [Edge {
            from: 0,
            to: 1,
            capacity,
        }];
        let output = max_flow(2, &direct, 0, 1).unwrap();
        assert_eq!(
            output,
            Flow {
                value: u128::from(capacity),
                edge_flows: vec![capacity],
                source_side: vec![true, false]
            }
        );
        // An isolated extra vertex preserves value/edge observations, not full cut shape.
        assert_eq!(
            max_flow(3, &direct, 0, 1).unwrap().edge_flows,
            output.edge_flows
        );
    }
}

#[test]
fn validation_and_empty_network() {
    assert_eq!(
        max_flow(0, &[], 0, 0),
        Err(Error::InvalidVertex {
            vertex: 0,
            vertices: 0
        })
    );
    assert_eq!(
        max_flow(2, &[], 0, 2),
        Err(Error::InvalidVertex {
            vertex: 2,
            vertices: 2
        })
    );
    assert_eq!(max_flow(2, &[], 1, 1), Err(Error::SameTerminal));
    assert_eq!(
        max_flow(
            2,
            &[Edge {
                from: 2,
                to: 0,
                capacity: 0
            }],
            0,
            1
        ),
        Err(Error::InvalidEdge { index: 0 })
    );
    assert_eq!(
        max_flow(2, &[], 0, 1).unwrap(),
        Flow {
            value: 0,
            edge_flows: vec![],
            source_side: vec![true, false]
        }
    );
}
