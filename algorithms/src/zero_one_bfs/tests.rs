use super::*;

fn oracle(graph: &[Vec<(usize, u8)>], source: usize) -> Vec<Option<usize>> {
    fn visit(
        g: &[Vec<(usize, u8)>],
        u: usize,
        cost: usize,
        seen: &mut [bool],
        best: &mut [Option<usize>],
    ) {
        best[u] = Some(best[u].map_or(cost, |old| old.min(cost)));
        seen[u] = true;
        for &(v, w) in &g[u] {
            if !seen[v] {
                visit(g, v, cost + usize::from(w), seen, best);
            }
        }
        seen[u] = false;
    }
    let mut best = vec![None; graph.len()];
    visit(graph, source, 0, &mut vec![false; graph.len()], &mut best);
    best
}

#[test]
fn exhaustive_simple_path_oracle() {
    // Every directed three-vertex binary graph, including self loops.
    for mut code in 0..3usize.pow(9) {
        let mut graph = vec![vec![]; 3];
        for row in &mut graph {
            for to in 0..3 {
                let edge = code % 3;
                code /= 3;
                if edge != 0 {
                    row.push((to, (edge - 1) as u8));
                }
            }
        }
        for source in 0..3 {
            assert_eq!(
                zero_one_bfs(&graph, source).unwrap(),
                oracle(&graph, source)
            );
        }
    }
}

#[test]
fn errors_are_global_and_ordered() {
    assert_eq!(
        zero_one_bfs(&[], 0),
        Err(Error::InvalidSource {
            source: 0,
            vertices: 0
        })
    );
    let g = vec![vec![], vec![(2, 9)]];
    assert_eq!(
        zero_one_bfs(&g, 2),
        Err(Error::InvalidSource {
            source: 2,
            vertices: 2
        })
    );
    assert_eq!(
        zero_one_bfs(&g, 0),
        Err(Error::InvalidEdge {
            from: 1,
            to: 2,
            vertices: 2
        })
    );
    assert_eq!(
        zero_one_bfs(&[vec![], vec![(1, 2)]], 0),
        Err(Error::InvalidWeight {
            from: 1,
            to: 1,
            weight: 2
        })
    );
    assert_eq!(
        zero_one_bfs(&[vec![(0, 255)], vec![(8, 0)]], 0),
        Err(Error::InvalidWeight {
            from: 0,
            to: 0,
            weight: 255
        })
    );
}

#[test]
fn improvements_zero_cycles_and_parallel_edges() {
    let g = vec![
        vec![(1, 1), (2, 0), (2, 1)],
        vec![(3, 1)],
        vec![(1, 0), (0, 0)],
        vec![],
        vec![],
    ];
    assert_eq!(
        zero_one_bfs(&g, 0),
        Ok(vec![Some(0), Some(0), Some(0), Some(1), None])
    );
    assert_eq!(zero_one_bfs(&[vec![(0, 0), (0, 1)]], 0), Ok(vec![Some(0)]));
}

#[test]
fn permutation_duplication_and_input_preservation() {
    let g = vec![vec![(1, 1), (2, 0)], vec![(3, 0)], vec![(1, 0)], vec![]];
    let original = g.clone();
    let expected = zero_one_bfs(&g, 0).unwrap();
    let p = [2, 0, 3, 1];
    let mut mapped = vec![vec![]; 4];
    for (u, row) in g.iter().enumerate() {
        for &(v, w) in row {
            mapped[p[u]].extend([(p[v], w), (p[v], w)]);
        }
        mapped[p[u]].reverse();
    }
    let result = zero_one_bfs(&mapped, p[0]).unwrap();
    for u in 0..4 {
        assert_eq!(result[p[u]], expected[u]);
    }
    assert_eq!(g, original);
}

#[test]
fn long_chain_and_isolated_extension() {
    let n = 2048;
    let mut g = vec![vec![]; n + 1];
    for (u, row) in g.iter_mut().enumerate().take(n - 1) {
        row.push((u + 1, 1));
    }
    let d = zero_one_bfs(&g, 0).unwrap();
    assert_eq!(d[n - 1], Some(n - 1));
    assert_eq!(d[n], None);
    for (u, row) in g.iter().enumerate() {
        for &(v, w) in row {
            assert!(d[v].unwrap() <= d[u].unwrap() + usize::from(w));
        }
    }
}

#[test]
fn redundant_edges_preserve_distance_fiber() {
    let g = vec![vec![(1, 0)], vec![(2, 1)], vec![]];
    let mut h = g.clone();
    h[0].push((2, 1));
    assert_ne!(g, h);
    assert_eq!(zero_one_bfs(&g, 0), zero_one_bfs(&h, 0));
}
