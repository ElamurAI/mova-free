use super::*;

fn oracle(graph: &[Vec<(usize, i64)>], source: usize) -> Vec<Option<i128>> {
    // Independent exhaustive path enumeration, no topological sorter or relaxation queue.
    fn paths(g: &[Vec<(usize, i64)>], u: usize, sum: i128, out: &mut [Option<i128>]) {
        out[u] = Some(out[u].map_or(sum, |old| old.min(sum)));
        for &(v, w) in &g[u] {
            paths(g, v, sum + i128::from(w), out);
        }
    }
    let mut out = vec![None; graph.len()];
    paths(graph, source, 0, &mut out);
    out
}

#[test]
fn exhaustive_signed_dags_and_relabeling() {
    for mut code in 0..4usize.pow(6) {
        let mut g = vec![vec![]; 4];
        for (u, row) in g.iter_mut().enumerate() {
            for v in u + 1..4 {
                let digit = code % 4;
                code /= 4;
                if digit != 0 {
                    row.push((v, digit as i64 - 2));
                }
            }
        }
        let p = [3, 1, 0, 2];
        let mut h = vec![vec![]; 4];
        for (u, row) in g.iter().enumerate() {
            for &(v, w) in row {
                h[p[u]].push((p[v], w));
            }
        }
        for source in 0..4 {
            let expected = oracle(&g, source);
            assert_eq!(shortest_paths(&g, source).unwrap(), expected);
            let mapped = shortest_paths(&h, p[source]).unwrap();
            for u in 0..4 {
                assert_eq!(mapped[p[u]], expected[u]);
            }
        }
    }
}

#[test]
fn global_errors_and_precedence() {
    assert_eq!(
        shortest_paths(&[], 0),
        Err(Error::InvalidSource {
            source: 0,
            vertices: 0
        })
    );
    let g = vec![vec![(0, 0)], vec![(3, 0)]];
    assert_eq!(
        shortest_paths(&g, 9),
        Err(Error::InvalidSource {
            source: 9,
            vertices: 2
        })
    );
    assert_eq!(
        shortest_paths(&g, 0),
        Err(Error::InvalidEdge {
            from: 1,
            to: 3,
            vertices: 2
        })
    );
    assert_eq!(
        shortest_paths(&[vec![], vec![(2, -1)], vec![(1, 0)]], 0),
        Err(Error::Cycle)
    );
    assert_eq!(shortest_paths(&[vec![(0, -1)]], 0), Err(Error::Cycle));
    assert_eq!(
        shortest_paths(&[vec![(1, 0)], vec![(0, 0)]], 0),
        Err(Error::Cycle)
    );
}

#[test]
fn full_width_and_negative_paths() {
    let g = vec![vec![(1, i64::MIN), (2, 0)], vec![(2, i64::MIN)], vec![]];
    assert_eq!(
        shortest_paths(&g, 0),
        Ok(vec![
            Some(0),
            Some(i128::from(i64::MIN)),
            Some(2 * i128::from(i64::MIN))
        ])
    );
    let h = vec![vec![(1, i64::MAX)], vec![(2, i64::MAX)], vec![]];
    assert_eq!(
        shortest_paths(&h, 0).unwrap()[2],
        Some(2 * i128::from(i64::MAX))
    );
    assert_eq!(shortest_paths(&[vec![]], 0), Ok(vec![Some(0)]));
}

#[test]
fn duplicates_order_and_input_preservation() {
    let g = vec![vec![(1, 4), (1, -2), (2, 3)], vec![(2, -5)], vec![], vec![]];
    let snapshot = g.clone();
    let mut h = g.clone();
    for row in &mut h {
        row.extend(row.clone());
        row.reverse();
    }
    assert_eq!(shortest_paths(&g, 0), shortest_paths(&h, 0));
    assert_eq!(
        shortest_paths(&g, 0).unwrap(),
        vec![Some(0), Some(-2), Some(-7), None]
    );
    assert_eq!(g, snapshot);
}

#[test]
fn potential_shift_and_distance_fibers() {
    let g = vec![vec![(1, 2), (2, 3)], vec![(2, -1)], vec![]];
    let p = [4i64, -7, 2];
    let mut h = g.clone();
    for (u, row) in h.iter_mut().enumerate() {
        for (v, w) in row {
            *w += p[*v] - p[u];
        }
    }
    let a = shortest_paths(&g, 0).unwrap();
    let b = shortest_paths(&h, 0).unwrap();
    for u in 0..3 {
        assert_eq!(b[u], a[u].map(|x| x + i128::from(p[u] - p[0])));
    }
    let mut redundant = g.clone();
    redundant[0].push((2, 1));
    assert_eq!(shortest_paths(&g, 0), shortest_paths(&redundant, 0));
}

#[test]
fn long_chain_reachability() {
    let n = 2048;
    let mut g = vec![vec![]; n + 1];
    for (u, row) in g.iter_mut().enumerate().take(n - 1) {
        row.push((u + 1, -1));
    }
    let d = shortest_paths(&g, 0).unwrap();
    assert_eq!(d[n - 1], Some(-((n - 1) as i128)));
    assert_eq!(d[n], None);
}
