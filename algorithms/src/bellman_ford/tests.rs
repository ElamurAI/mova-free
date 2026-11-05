use super::*;

// Independent all-pairs intermediate-vertex closure, not edge-count rounds.
fn oracle(
    graph: &[Vec<(usize, i64)>],
    source: usize,
) -> Result<Vec<Option<i128>>, BellmanFordError> {
    let n = graph.len();
    let mut d = vec![vec![None; n]; n];
    for (u, edges) in graph.iter().enumerate() {
        d[u][u] = Some(0i128);
        for &(v, w) in edges {
            let w = i128::from(w);
            d[u][v] = Some(d[u][v].map_or(w, |old| old.min(w)));
        }
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                if let (Some(a), Some(b)) = (d[u][k], d[k][v]) {
                    let c = a + b;
                    d[u][v] = Some(d[u][v].map_or(c, |old| old.min(c)));
                }
            }
        }
    }
    if (0..n).any(|v| d[source][v].is_some() && d[v][v].unwrap() < 0) {
        Err(BellmanFordError::ReachableNegativeCycle)
    } else {
        Ok(d[source].clone())
    }
}

#[test]
fn exhaustive_signed_graphs() {
    for n in 1..=3usize {
        for mut code in 0..3usize.pow((n * n) as u32) {
            let mut graph = vec![vec![]; n];
            for edges in &mut graph {
                for v in 0..n {
                    match code % 3 {
                        1 => edges.push((v, -1)),
                        2 => edges.push((v, 1)),
                        _ => {}
                    }
                    code /= 3;
                }
            }
            for source in 0..n {
                assert_eq!(
                    bellman_ford(&graph, source),
                    oracle(&graph, source),
                    "{graph:?}, {source}"
                );
            }
        }
    }
}

#[test]
fn cycles_and_full_width_paths() {
    assert_eq!(
        bellman_ford(&[vec![(0, -1)]], 0),
        Err(BellmanFordError::ReachableNegativeCycle)
    );
    assert_eq!(
        bellman_ford(&[vec![(0, 0)], vec![(1, -1)]], 0),
        Ok(vec![Some(0), None])
    );
    let graph = vec![vec![(1, 5), (1, i64::MIN)], vec![(2, i64::MIN)], vec![]];
    assert_eq!(
        bellman_ford(&graph, 0),
        Ok(vec![
            Some(0),
            Some(i128::from(i64::MIN)),
            Some(2 * i128::from(i64::MIN))
        ])
    );
    let graph = vec![vec![(1, i64::MAX)], vec![(2, i64::MAX)], vec![]];
    assert_eq!(
        bellman_ford(&graph, 0).unwrap()[2],
        Some(2 * i128::from(i64::MAX))
    );
    assert_eq!(
        bellman_ford(&[vec![(1, -5)], vec![(0, 5)]], 0),
        Ok(vec![Some(0), Some(-5)])
    );
}

#[test]
fn validation_precedence() {
    assert_eq!(
        bellman_ford(&[], 0),
        Err(BellmanFordError::InvalidSource {
            source: 0,
            vertices: 0
        })
    );
    assert_eq!(
        bellman_ford(&[vec![(9, 0)]], 1),
        Err(BellmanFordError::InvalidSource {
            source: 1,
            vertices: 1
        })
    );
    assert_eq!(
        bellman_ford(&[vec![], vec![(2, 0)]], 0),
        Err(BellmanFordError::InvalidEdge {
            from: 1,
            to: 2,
            vertices: 2
        })
    );
}

// A successful distance vector has both lower-bound inequalities and attained
// tight paths. Inequalities alone would also admit artificially low distances.
fn distance_certificate(graph: &[Vec<(usize, i64)>], d: &[Option<i128>]) -> bool {
    if d.len() != graph.len() || d[0] != Some(0) {
        return false;
    }
    let mut tight = vec![false; graph.len()];
    tight[0] = true;
    for _ in 0..graph.len() {
        for (u, edges) in graph.iter().enumerate() {
            for &(v, w) in edges {
                if let Some(a) = d[u] {
                    let Some(b) = d[v] else { return false };
                    if b > a + i128::from(w) {
                        return false;
                    }
                    if tight[u] && b == a + i128::from(w) {
                        tight[v] = true;
                    }
                }
            }
        }
    }
    d.iter()
        .zip(tight)
        .all(|(v, reached)| v.is_some() == reached)
}

#[test]
fn potential_reweighting_correspondence() {
    // All 64 directed loop-free topologies; nonnegative base weights rule out
    // negative cycles. Potentials may produce negative transformed edges.
    for mask in 0..64usize {
        let mut graph = vec![vec![]; 3];
        let mut bit = 0;
        for (u, edges) in graph.iter_mut().enumerate() {
            for v in 0..3 {
                if u != v {
                    if mask & (1 << bit) != 0 {
                        edges.push((v, (u + v) as i64));
                    }
                    bit += 1;
                }
            }
        }
        for h in [[0i64, 0, 0], [3, -4, 2], [-5, 7, -2]] {
            let transformed: Vec<Vec<_>> = graph
                .iter()
                .enumerate()
                .map(|(u, es)| es.iter().map(|&(v, w)| (v, w + h[u] - h[v])).collect())
                .collect();
            for s in 0..3 {
                let d = bellman_ford(&graph, s).unwrap();
                let expected: Vec<_> = d
                    .iter()
                    .enumerate()
                    .map(|(v, d)| d.map(|x| x + i128::from(h[s]) - i128::from(h[v])))
                    .collect();
                assert_eq!(bellman_ford(&transformed, s).unwrap(), expected);
            }
        }
    }
    let cycle = [vec![(1, -2)], vec![(0, 1)]];
    let transformed = [vec![(1, 5)], vec![(0, -6)]];
    assert_eq!(bellman_ford(&cycle, 0), bellman_ford(&transformed, 0));
}

#[test]
fn tight_certificate_accepts_exactly_distance_fiber() {
    let graph = [vec![(1, -2), (2, 4)], vec![(2, 3)], vec![], vec![(3, -1)]];
    let actual = bellman_ford(&graph, 0).unwrap();
    let mut accepted = 0;
    for a in -3..=2 {
        for b in -1..=5 {
            for absent in [None, Some(0)] {
                let candidate = vec![Some(0), Some(a), Some(b), absent];
                let member = distance_certificate(&graph, &candidate);
                assert_eq!(member, candidate == actual);
                accepted += usize::from(member);
            }
        }
    }
    assert_eq!(accepted, 1);
    // Satisfies edge inequalities but cannot be attained by tight paths.
    assert!(!distance_certificate(
        &graph,
        &[Some(0), Some(-3), Some(0), None]
    ));
}

#[test]
fn output_compatible_graph_construction() {
    for a in -2i64..=2 {
        for b in -2i64..=2 {
            let expected = vec![Some(0), Some(i128::from(a)), Some(i128::from(b)), None];
            let base = vec![vec![(1, a), (2, b)], vec![], vec![], vec![]];
            for slack in 0..=2 {
                let mut graph = base.clone();
                graph[1].push((2, b - a + slack));
                graph[3].push((3, -1));
                assert_eq!(bellman_ford(&graph, 0).unwrap(), expected);
            }
            let mut outside = base.clone();
            outside[1].push((2, b - a - 1));
            assert_ne!(bellman_ford(&outside, 0).unwrap(), expected);
        }
    }
}
