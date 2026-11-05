use super::*;
#[test]
fn boundaries_and_large_costs() {
    assert_eq!(
        dijkstra(&[], 0),
        Err(DijkstraError::InvalidSource {
            source: 0,
            vertices: 0
        })
    );
    assert_eq!(
        dijkstra(&[vec![]], 1),
        Err(DijkstraError::InvalidSource {
            source: 1,
            vertices: 1
        })
    );
    assert_eq!(
        dijkstra(&[vec![], vec![(2, 0)]], 0),
        Err(DijkstraError::InvalidEdge {
            from: 1,
            to: 2,
            vertices: 2
        })
    );
    assert_eq!(
        dijkstra(&[vec![(1, u64::MAX)], vec![(2, u64::MAX)], vec![]], 0),
        Ok(vec![
            Some(0),
            Some(u128::from(u64::MAX)),
            Some(2 * u128::from(u64::MAX))
        ])
    );
    assert_eq!(
        dijkstra(&[vec![(0, 0), (1, 9), (1, 0)], vec![(0, 0)], vec![]], 0),
        Ok(vec![Some(0), Some(0), None])
    );
    assert_eq!(dijkstra(&[vec![]], 0), Ok(vec![Some(0)]));
    assert_eq!(
        dijkstra(
            &[vec![(1, 10), (2, 1)], vec![(3, 1)], vec![(1, 1)], vec![]],
            0
        ),
        Ok(vec![Some(0), Some(2), Some(1), Some(3)])
    );
}

// Synchronous Bellman-Ford uses copies and a bounded path-length recurrence,
// independently of heap selection and stale-entry handling.
fn oracle(graph: &[Vec<(usize, u64)>], source: usize) -> Vec<Option<u128>> {
    let mut previous = vec![None; graph.len()];
    previous[source] = Some(0u128);
    for _ in 1..graph.len() {
        let mut next = previous.clone();
        for (u, edges) in graph.iter().enumerate() {
            if let Some(d) = previous[u] {
                for &(v, w) in edges {
                    let candidate = d + u128::from(w);
                    next[v] = Some(next[v].map_or(candidate, |old| old.min(candidate)));
                }
            }
        }
        previous = next;
    }
    previous
}

#[test]
fn exhaustive_bellman_ford_oracle() {
    for n in 1usize..=3 {
        for code in 0usize..3usize.pow((n * n) as u32) {
            let mut digits = code;
            let mut graph = vec![vec![]; n];
            for edges in &mut graph {
                for v in 0..n {
                    match digits % 3 {
                        1 => edges.push((v, 0)),
                        2 => edges.push((v, 2)),
                        _ => {}
                    }
                    digits /= 3;
                }
            }
            let before = graph.clone();
            for source in 0..n {
                assert_eq!(
                    dijkstra(&graph, source),
                    Ok(oracle(&graph, source)),
                    "n={n}, code={code}, source={source}"
                );
            }
            assert_eq!(graph, before);
        }
    }
}

fn next_word(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

#[test]
fn wider_seeded_full_width_weights_match_bellman_ford() {
    const SEED: u64 = 0xd151_2026_1029;
    let mut state = SEED;
    for case in 0..36 {
        let n = 4 + case % 13;
        let mut graph = vec![vec![]; n];
        for edges in &mut graph {
            for v in 0..n {
                let word = next_word(&mut state);
                if word >> 60 < 4 {
                    let weight = match word % 5 {
                        0 => 0,
                        1 => u64::MAX,
                        _ => next_word(&mut state),
                    };
                    edges.push((v, weight));
                }
            }
        }
        let before = graph.clone();
        for source in 0..n {
            assert_eq!(
                dijkstra(&graph, source),
                Ok(oracle(&graph, source)),
                "seed={SEED:#x}, case={case}, source={source}"
            );
        }
        assert_eq!(graph, before, "seed={SEED:#x}, case={case}");
    }
}

#[test]
fn weight_scaling_and_edge_splitting_preserve_path_costs() {
    const SEED: u64 = 0xd152_2026_1029;
    let mut state = SEED;
    for case in 0..24 {
        let n = 4 + case % 6;
        let mut graph = vec![vec![]; n];
        for (u, edges) in graph.iter_mut().enumerate() {
            edges.push(((u + 1) % n, next_word(&mut state) % 1_000_000));
            edges.push((
                (next_word(&mut state) % n as u64) as usize,
                next_word(&mut state) % 1_000_000,
            ));
        }
        let scaled: Vec<Vec<(usize, u64)>> = graph
            .iter()
            .map(|edges| edges.iter().map(|&(v, w)| (v, w * 7)).collect())
            .collect();
        let mut split = vec![vec![]; n];
        for (u, edges) in graph.iter().enumerate() {
            for &(v, w) in edges {
                let middle = split.len();
                split.push(vec![(v, w - w / 2)]);
                split[u].push((middle, w / 2));
            }
        }
        for source in 0..n {
            let expected = oracle(&graph, source);
            assert_eq!(
                dijkstra(&graph, source),
                Ok(expected.clone()),
                "seed={SEED:#x}, case={case}, original source={source}"
            );
            let expected_scaled: Vec<_> = expected.iter().map(|d| d.map(|cost| cost * 7)).collect();
            assert_eq!(
                dijkstra(&scaled, source),
                Ok(expected_scaled),
                "seed={SEED:#x}, case={case}, scaled source={source}"
            );
            let actual_split = dijkstra(&split, source).unwrap();
            assert_eq!(
                &actual_split[..n],
                expected.as_slice(),
                "seed={SEED:#x}, case={case}, split source={source}"
            );
        }
    }
}

#[test]
fn zero_cycles_stale_entries_and_wide_sum_compete() {
    let graph = vec![
        vec![(1, u64::MAX), (2, 7), (4, u64::MAX)],
        vec![(3, 0)],
        vec![(1, 0)],
        vec![(1, 0), (4, 5)],
        vec![(5, u64::MAX)],
        vec![(6, u64::MAX)],
        vec![],
        vec![],
    ];
    assert_eq!(
        dijkstra(&graph, 0),
        Ok(vec![
            Some(0),
            Some(7),
            Some(7),
            Some(7),
            Some(12),
            Some(12 + u128::from(u64::MAX)),
            Some(12 + 2 * u128::from(u64::MAX)),
            None
        ])
    );
    assert_eq!(dijkstra(&graph, 0), Ok(oracle(&graph, 0)));
}
