use super::*;
fn oracle(n: usize, edges: &[Edge], limit: u64) -> (u64, i128) {
    let total = edges
        .iter()
        .fold(1usize, |p, e| p * (e.capacity as usize + 1));
    let mut best = (0, 0);
    for mut code in 0..total {
        let mut balance = vec![0i128; n];
        let mut cost = 0;
        for e in edges {
            let f = (code % (e.capacity as usize + 1)) as i128;
            code /= e.capacity as usize + 1;
            balance[e.from] -= f;
            balance[e.to] += f;
            cost += f * i128::from(e.cost);
        }
        let value = balance[n - 1];
        if value < 0
            || value > i128::from(limit)
            || balance[0] != -value
            || balance[1..n - 1].iter().any(|&b| b != 0)
        {
            continue;
        }
        let value = value as u64;
        if value > best.0 || value == best.0 && cost < best.1 {
            best = (value, cost);
        }
    }
    best
}
#[test]
fn exhaustive_flow_assignments() {
    let pairs = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for mask in 0..729 {
        let mut code = mask;
        let edges: Vec<_> = pairs
            .iter()
            .enumerate()
            .map(|(i, &(from, to))| {
                let capacity = code % 3;
                code /= 3;
                Edge {
                    from,
                    to,
                    capacity,
                    cost: i as i64 - 3,
                }
            })
            .collect();
        for limit in 0..=3 {
            let f = minimum_cost_flow(4, &edges, 0, 3, limit).unwrap();
            assert_eq!((f.value, f.cost), oracle(4, &edges, limit), "flow optimum");
            let mut balances = [0i128; 4];
            let mut cost = 0;
            for (e, &x) in edges.iter().zip(&f.edge_flows) {
                assert!(x <= e.capacity);
                balances[e.from] -= i128::from(x);
                balances[e.to] += i128::from(x);
                cost += i128::from(x) * i128::from(e.cost);
            }
            assert_eq!(balances, [-i128::from(f.value), 0, 0, i128::from(f.value)]);
            assert_eq!(cost, f.cost);
        }
    }
}
#[test]
fn residual_rerouting_and_boundaries() {
    let edges =
        [(0, 1, 0), (0, 2, 0), (1, 2, -1), (1, 3, 0), (2, 3, 0)].map(|(from, to, cost)| Edge {
            from,
            to,
            cost,
            capacity: 1,
        });
    let f = minimum_cost_flow(4, &edges, 0, 3, 2).unwrap();
    assert_eq!((f.value, f.cost), (2, 0));
    assert_eq!(f.edge_flows, vec![1, 1, 0, 1, 1]);
    let edges = [Edge {
        from: 0,
        to: 1,
        capacity: u64::MAX,
        cost: i64::MIN,
    }];
    let f = minimum_cost_flow(2, &edges, 0, 1, 1024).unwrap();
    assert_eq!(f.cost, 1024 * i128::from(i64::MIN));
    let edges = [
        Edge {
            from: 0,
            to: 1,
            capacity: 1,
            cost: 2,
        },
        Edge {
            from: 1,
            to: 0,
            capacity: 1,
            cost: -3,
        },
    ];
    assert_eq!(
        minimum_cost_flow(2, &edges, 0, 1, 0),
        Err(Error::NegativeCycle)
    );
    assert_eq!(
        minimum_cost_flow(65, &[], 999, 999, 0),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        minimum_cost_flow(0, &[], 0, 0, 0),
        Err(Error::InvalidVertex)
    );
    assert_eq!(minimum_cost_flow(2, &[], 0, 0, 0), Err(Error::SameTerminal));
    assert_eq!(
        minimum_cost_flow(
            2,
            &[Edge {
                from: 2,
                to: 0,
                capacity: 0,
                cost: 0
            }],
            0,
            1,
            0
        ),
        Err(Error::InvalidEdge { index: 0 })
    );
    assert_eq!(
        minimum_cost_flow(2, &[], 0, 1, 1025),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn cyclic_potential_oracle() {
    let pairs = [(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)];
    for mask in 0..64 {
        let edges: Vec<_> = pairs
            .iter()
            .enumerate()
            .map(|(i, &(from, to))| Edge {
                from,
                to,
                capacity: (mask >> i) & 1,
                cost: 1 + [2, -2, 0][from] - [2, -2, 0][to],
            })
            .collect();
        for limit in 0..=3 {
            let f = minimum_cost_flow(3, &edges, 0, 2, limit).unwrap();
            assert_eq!((f.value, f.cost), oracle(3, &edges, limit));
        }
    }
    let cycle = [Edge {
        from: 2,
        to: 2,
        capacity: 1,
        cost: -1,
    }];
    assert_eq!(
        minimum_cost_flow(3, &cycle, 0, 1, 0),
        Err(Error::NegativeCycle)
    );
    let zero_cycle = [Edge {
        from: 2,
        to: 2,
        capacity: 0,
        cost: -1,
    }];
    assert_eq!(minimum_cost_flow(3, &zero_cycle, 0, 1, 0).unwrap().value, 0);
}
