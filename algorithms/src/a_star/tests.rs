use super::*;

// Independent enumeration of simple paths, not priority-queue relaxation.
fn oracle(graph: &[Vec<(usize, u64)>], start: usize, goal: usize) -> Option<u128> {
    fn walk(
        g: &[Vec<(usize, u64)>],
        v: usize,
        goal: usize,
        seen: usize,
        sum: u128,
        best: &mut Option<u128>,
    ) {
        if v == goal {
            if best.is_none_or(|old| sum < old) {
                *best = Some(sum);
            }
            return;
        }
        for &(to, w) in &g[v] {
            if seen & (1 << to) == 0 {
                walk(g, to, goal, seen | (1 << to), sum + u128::from(w), best);
            }
        }
    }
    let mut best = None;
    walk(graph, start, goal, 1 << start, 0, &mut best);
    best
}
fn witness(graph: &[Vec<(usize, u64)>], start: usize, goal: usize, path: &Path) {
    assert_eq!(path.vertices.first(), Some(&start));
    assert_eq!(path.vertices.last(), Some(&goal));
    let mut seen = std::collections::BTreeSet::new();
    for &v in &path.vertices {
        assert!(seen.insert(v));
    }
    let sum: u128 = path
        .vertices
        .windows(2)
        .map(|e| {
            u128::from(
                graph[e[0]]
                    .iter()
                    .filter(|&&(v, _)| v == e[1])
                    .map(|&(_, w)| w)
                    .min()
                    .unwrap(),
            )
        })
        .sum();
    assert_eq!(sum, path.distance);
}
#[test]
fn exhaustive_simple_path_oracle() {
    let mut calls = 0;
    for code in 0..729 {
        let mut digit = code;
        let mut graph = vec![vec![]; 3];
        for (from, row) in graph.iter_mut().enumerate() {
            for to in 0..3 {
                if from != to {
                    let d = digit % 3;
                    digit /= 3;
                    if d != 0 {
                        row.push((to, if d == 1 { 0 } else { 2 }));
                    }
                }
            }
        }
        for goal in 0..3 {
            let h: Vec<u64> = (0..3)
                .map(|v| oracle(&graph, v, goal).unwrap_or(2).min(2) as u64)
                .collect();
            for start in 0..3 {
                for heuristic in [&[0u64; 3][..], &h[..]] {
                    let result = shortest_path(&graph, start, goal, heuristic).unwrap();
                    assert_eq!(
                        result.as_ref().map(|p| p.distance),
                        oracle(&graph, start, goal)
                    );
                    if let Some(path) = result {
                        witness(&graph, start, goal, &path);
                    }
                    calls += 1;
                }
            }
        }
    }
    assert_eq!(calls, 13122);
}
#[test]
fn rejects_inadmissible_and_disconnected_bad_heuristics() {
    let g = vec![vec![(1, 1), (2, 5)], vec![(2, 1)], vec![]];
    assert_eq!(
        shortest_path(&g, 0, 2, &[0, 99, 0]),
        Err(Error::InconsistentHeuristic { from: 1, to: 2 })
    );
    let g = vec![vec![], vec![(2, 0)], vec![]];
    assert_eq!(
        shortest_path(&g, 0, 0, &[0, 1, 0]),
        Err(Error::InconsistentHeuristic { from: 1, to: 2 })
    );
}
#[test]
fn validation_priority_and_boundaries() {
    assert_eq!(
        shortest_path(&[], 0, 0, &[]),
        Err(Error::InvalidVertex {
            vertex: 0,
            vertices: 0
        })
    );
    assert_eq!(
        shortest_path(&[vec![]], 0, 1, &[]),
        Err(Error::InvalidVertex {
            vertex: 1,
            vertices: 1
        })
    );
    assert_eq!(
        shortest_path(&[vec![]], 0, 0, &[]),
        Err(Error::HeuristicLength {
            expected: 1,
            actual: 0
        })
    );
    assert_eq!(
        shortest_path(&[vec![(3, 0)]], 0, 0, &[1]),
        Err(Error::NonzeroGoalHeuristic)
    );
    assert_eq!(
        shortest_path(&[vec![], vec![(8, 0)]], 0, 0, &[0, 0]),
        Err(Error::InvalidEdge { from: 1, to: 8 })
    );
    assert_eq!(
        shortest_path(&[vec![]], 0, 0, &[0]).unwrap(),
        Some(Path {
            distance: 0,
            vertices: vec![0]
        })
    );
}
#[test]
fn wide_costs_and_heuristic_addition() {
    let g = vec![vec![(1, u64::MAX)], vec![(2, u64::MAX)], vec![]];
    let p = shortest_path(&g, 0, 2, &[u64::MAX, u64::MAX, 0])
        .unwrap()
        .unwrap();
    assert_eq!(p.distance, 2 * u128::from(u64::MAX));
    witness(&g, 0, 2, &p);
}
#[test]
fn stale_entries_parallel_edges_zero_cycles_and_ties() {
    let g = vec![
        vec![(1, 9), (1, 1), (2, 1), (0, 0)],
        vec![(0, 0), (3, 1)],
        vec![(3, 1)],
        vec![],
    ];
    let p = shortest_path(&g, 0, 3, &[2, 1, 1, 0]).unwrap().unwrap();
    assert_eq!(
        p,
        Path {
            distance: 2,
            vertices: vec![0, 1, 3]
        }
    );
    // A stale nine-cost entry is popped before exhausting an unreachable query.
    let mut extended = g.clone();
    extended.push(vec![]);
    assert_eq!(shortest_path(&extended, 0, 4, &[0; 5]), Ok(None));
    assert_eq!(g[0][0], (1, 9));
}
#[test]
fn cost_projection_fibers_and_monotonicity() {
    let base = vec![vec![(1, 2)], vec![]];
    let looped = vec![vec![(1, 2), (0, 0)], vec![(1, 7)]];
    assert_eq!(
        shortest_path(&base, 0, 1, &[0, 0]),
        shortest_path(&looped, 0, 1, &[2, 0])
    );
    let heavier = vec![vec![(1, 3)], vec![]];
    assert!(
        shortest_path(&heavier, 0, 1, &[0, 0])
            .unwrap()
            .unwrap()
            .distance
            > shortest_path(&base, 0, 1, &[0, 0])
                .unwrap()
                .unwrap()
                .distance
    );
}
