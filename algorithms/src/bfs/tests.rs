use super::*;

#[test]
fn boundaries_and_global_validation() {
    assert_eq!(
        bfs(&[], 0),
        Err(BfsError::InvalidSource {
            source: 0,
            vertices: 0
        })
    );
    assert_eq!(
        bfs(&[vec![]], 1),
        Err(BfsError::InvalidSource {
            source: 1,
            vertices: 1
        })
    );
    assert_eq!(
        bfs(&[vec![0, 1, 1], vec![], vec![]], 0),
        Ok(vec![Some(0), Some(1), None])
    );
    assert_eq!(
        bfs(&[vec![], vec![2]], 0),
        Err(BfsError::InvalidEdge {
            from: 1,
            to: 2,
            vertices: 2
        })
    );
    assert_eq!(bfs(&[vec![]], 0), Ok(vec![Some(0)]));
}

#[test]
fn exhaustive_floyd_warshall_oracle() {
    for n in 1..=3 {
        for mask in 0usize..(1 << (n * n)) {
            let graph: Vec<Vec<usize>> = (0..n)
                .map(|u| (0..n).filter(|&v| mask & (1 << (u * n + v)) != 0).collect())
                .collect();
            let before = graph.clone();
            let mut oracle = vec![vec![n + 1; n]; n];
            for u in 0..n {
                oracle[u][u] = 0;
                for &v in &graph[u] {
                    oracle[u][v] = oracle[u][v].min(1);
                }
            }
            for k in 0..n {
                for u in 0..n {
                    for v in 0..n {
                        oracle[u][v] = oracle[u][v].min(oracle[u][k] + oracle[k][v]);
                    }
                }
            }
            for (source, row) in oracle.iter().enumerate() {
                let expected = row
                    .iter()
                    .map(|&d| if d <= n { Some(d) } else { None })
                    .collect();
                assert_eq!(
                    bfs(&graph, source),
                    Ok(expected),
                    "n={n}, mask={mask}, source={source}"
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

fn floyd(graph: &[Vec<usize>]) -> Vec<Vec<Option<usize>>> {
    let n = graph.len();
    let mut distances = vec![vec![n + 1; n]; n];
    for (u, edges) in graph.iter().enumerate() {
        distances[u][u] = 0;
        for &v in edges {
            distances[u][v] = distances[u][v].min(1);
        }
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                distances[u][v] = distances[u][v].min(distances[u][k] + distances[k][v]);
            }
        }
    }
    distances
        .into_iter()
        .map(|row| row.into_iter().map(|d| (d <= n).then_some(d)).collect())
        .collect()
}

#[test]
fn wider_seeded_graphs_match_floyd() {
    const SEED: u64 = 0xb151_2026_1029;
    let mut state = SEED;
    for case in 0..40 {
        let n = 4 + case % 17;
        let mut graph = vec![vec![]; n];
        for edges in &mut graph {
            for v in 0..n {
                if next_word(&mut state) >> 60 < 3 {
                    edges.push(v);
                }
            }
        }
        let before = graph.clone();
        for (source, expected) in floyd(&graph).into_iter().enumerate() {
            assert_eq!(
                bfs(&graph, source),
                Ok(expected),
                "seed={SEED:#x}, case={case}, source={source}"
            );
        }
        assert_eq!(graph, before, "seed={SEED:#x}, case={case}");
    }
}

#[test]
fn adjacency_permutation_and_duplicate_edges_preserve_distances() {
    const SEED: u64 = 0xb152_2026_1029;
    let mut state = SEED;
    for case in 0..32 {
        let n = 5 + case % 12;
        let mut graph = vec![vec![]; n];
        for edges in &mut graph {
            for v in 0..n {
                if next_word(&mut state) >> 61 == 0 {
                    edges.push(v);
                }
            }
        }
        let altered: Vec<Vec<usize>> = graph
            .iter()
            .map(|edges| edges.iter().rev().flat_map(|&v| [v, v, v]).collect())
            .collect();
        for (source, expected) in floyd(&graph).into_iter().enumerate() {
            assert_eq!(
                bfs(&graph, source),
                Ok(expected.clone()),
                "seed={SEED:#x}, case={case}, original source={source}"
            );
            assert_eq!(
                bfs(&altered, source),
                Ok(expected),
                "seed={SEED:#x}, case={case}, altered source={source}"
            );
        }
    }
}

#[test]
fn long_chain_and_competing_frontier_paths() {
    let n = 2048;
    let mut graph: Vec<Vec<usize>> = (0..n)
        .map(|u| if u + 1 < n { vec![u + 1] } else { vec![] })
        .collect();
    assert_eq!(bfs(&graph, 0), Ok((0..n).map(Some).collect()));
    graph[0].push(n - 1);
    let mut expected: Vec<Option<usize>> = (0..n).map(Some).collect();
    expected[n - 1] = Some(1);
    assert_eq!(bfs(&graph, 0), Ok(expected));
    // Stack traversal discovers vertex 3 through the longer right branch first.
    assert_eq!(
        bfs(&[vec![1, 2], vec![3], vec![4], vec![], vec![3]], 0),
        Ok(vec![Some(0), Some(1), Some(1), Some(2), Some(2)])
    );
}
