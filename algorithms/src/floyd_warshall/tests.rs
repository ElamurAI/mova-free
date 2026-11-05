use super::*;

// Enumerate only simple paths. Nonnegative cycles cannot improve a shortest path.
fn oracle(graph: &[Vec<(usize, u64)>]) -> Vec<Vec<Option<u128>>> {
    fn visit(
        graph: &[Vec<(usize, u64)>],
        u: usize,
        cost: u128,
        seen: &mut [bool],
        best: &mut [Option<u128>],
    ) {
        best[u] = Some(best[u].map_or(cost, |old| old.min(cost)));
        for &(v, w) in &graph[u] {
            if !seen[v] {
                seen[v] = true;
                visit(graph, v, cost + u128::from(w), seen, best);
                seen[v] = false;
            }
        }
    }
    (0..graph.len())
        .map(|source| {
            let mut best = vec![None; graph.len()];
            let mut seen = vec![false; graph.len()];
            seen[source] = true;
            visit(graph, source, 0, &mut seen, &mut best);
            best
        })
        .collect()
}

#[test]
fn boundaries_parallel_zero_and_wide_costs() {
    assert_eq!(floyd_warshall(&[]), Ok(vec![]));
    assert_eq!(
        floyd_warshall(&[vec![(0, 9), (0, 0)]]),
        Ok(vec![vec![Some(0)]])
    );
    assert_eq!(
        floyd_warshall(&[vec![], vec![(2, 0)]]),
        Err(Error::InvalidEdge {
            from: 1,
            to: 2,
            vertices: 2
        })
    );
    let graph = vec![
        vec![(1, u64::MAX), (1, 5), (1, 9)],
        vec![(2, u64::MAX)],
        vec![],
        vec![],
    ];
    let expected = vec![
        vec![Some(0), Some(5), Some(5 + u128::from(u64::MAX)), None],
        vec![None, Some(0), Some(u128::from(u64::MAX)), None],
        vec![None, None, Some(0), None],
        vec![None, None, None, Some(0)],
    ];
    assert_eq!(floyd_warshall(&graph), Ok(expected));
    let graph = vec![vec![(1, 0)], vec![(0, 0), (2, 1)], vec![]];
    assert_eq!(floyd_warshall(&graph), Ok(oracle(&graph)));
}

#[test]
fn exhaustive_ternary_graphs_match_simple_paths() {
    for n in 0usize..=3 {
        for mut code in 0usize..3usize.pow((n * n) as u32) {
            let original = code;
            let mut graph = vec![vec![]; n];
            for edges in &mut graph {
                for v in 0..n {
                    match code % 3 {
                        1 => edges.push((v, 0)),
                        2 => edges.push((v, 3)),
                        _ => {}
                    }
                    code /= 3;
                }
            }
            let before = graph.clone();
            assert_eq!(
                floyd_warshall(&graph),
                Ok(oracle(&graph)),
                "n={n},code={original}"
            );
            assert_eq!(graph, before);
        }
    }
}

#[test]
fn seeded_full_width_graphs_match_simple_paths() {
    const SEED: u64 = 0xf10d20261029;
    let mut state = SEED;
    for case in 0..48 {
        let n = 4 + case % 3;
        let mut graph = vec![vec![]; n];
        for edges in &mut graph {
            for v in 0..n {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                if state >> 60 < 4 {
                    edges.push((
                        v,
                        if state.is_multiple_of(4) {
                            u64::MAX
                        } else {
                            state
                        },
                    ));
                }
            }
        }
        assert_eq!(
            floyd_warshall(&graph),
            Ok(oracle(&graph)),
            "seed={SEED:#x},case={case}"
        );
    }
}

#[test]
fn transpose_scaling_and_duplicate_arcs_transport_distances() {
    let graph = vec![
        vec![(1, 2), (3, 9)],
        vec![(2, 0)],
        vec![(3, 3)],
        vec![(1, 4)],
        vec![],
    ];
    let expected = oracle(&graph);
    let mut transformed = vec![vec![]; graph.len()];
    for (u, edges) in graph.iter().enumerate() {
        for &(v, w) in edges {
            transformed[v].extend([(u, w * 7), (u, w * 7)]);
        }
    }
    for edges in &mut transformed {
        edges.reverse();
    }
    let actual = floyd_warshall(&transformed).unwrap();
    for (u, row) in actual.iter().enumerate() {
        for (v, &value) in row.iter().enumerate() {
            assert_eq!(value, expected[v][u].map(|d| d * 7), "u={u},v={v}");
        }
    }
}
