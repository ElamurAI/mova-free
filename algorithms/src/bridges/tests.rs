use super::*;

// Independent Boolean transitive closure, with edge or vertex deletion.
pub(crate) fn components(
    n: usize,
    edges: &[Edge],
    skip_edge: Option<usize>,
    skip_vertex: Option<usize>,
) -> usize {
    let mut reach = vec![vec![false; n]; n];
    for (v, row) in reach.iter_mut().enumerate() {
        row[v] = true;
    }
    for (id, e) in edges.iter().enumerate() {
        if Some(id) != skip_edge && Some(e.u) != skip_vertex && Some(e.v) != skip_vertex {
            reach[e.u][e.v] = true;
            reach[e.v][e.u] = true;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                reach[i][j] |= reach[i][k] && reach[k][j];
            }
        }
    }
    (0..n)
        .filter(|&v| {
            Some(v) != skip_vertex && !(0..v).any(|w| Some(w) != skip_vertex && reach[v][w])
        })
        .count()
}

pub(crate) fn cases(mut check: impl FnMut(usize, &[Edge])) {
    // All ordered edge sequences through length five on at most three vertices.
    for n in 0..=3 {
        let alphabet: Vec<_> = (0..n)
            .flat_map(|u| (u..n).map(move |v| Edge { u, v }))
            .collect();
        check(n, &[]);
        for len in 1..=5 {
            for mut code in 0..alphabet.len().pow(len) {
                let mut edges = Vec::new();
                for _ in 0..len {
                    edges.push(alphabet[code % alphabet.len()]);
                    code /= alphabet.len();
                }
                check(n, &edges);
            }
        }
    }
    // Every simple graph on four and five vertices.
    for n in 4..=5 {
        let alphabet: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| Edge { u, v }))
            .collect();
        for mask in 0..1usize << alphabet.len() {
            let edges: Vec<_> = alphabet
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (mask & (1 << i) != 0).then_some(e))
                .collect();
            check(n, &edges);
        }
    }
}

#[test]
fn deletion_oracle() {
    cases(|n, edges| {
        let base = components(n, edges, None, None);
        let expected: Vec<_> = (0..edges.len())
            .filter(|&id| components(n, edges, Some(id), None) > base)
            .collect();
        assert_eq!(
            bridges(n, edges).unwrap(),
            expected,
            "n={n}, edges={edges:?}"
        );
    });
}

#[test]
fn parallel_loops_and_disconnected() {
    let edges = [
        Edge { u: 0, v: 1 },
        Edge { u: 0, v: 1 },
        Edge { u: 1, v: 1 },
        Edge { u: 1, v: 2 },
        Edge { u: 3, v: 4 },
    ];
    assert_eq!(bridges(6, &edges), Ok(vec![3, 4]));
    assert_eq!(bridges(0, &[]), Ok(vec![]));
    assert_eq!(bridges(4, &[]), Ok(vec![]));
}

#[test]
fn validation_precedence() {
    assert_eq!(
        bridges(2, &[Edge { u: 3, v: 4 }, Edge { u: 5, v: 0 }]),
        Err(Error::InvalidEndpoint {
            edge: 0,
            vertex: 3,
            vertices: 2
        })
    );
    assert_eq!(
        bridges(
            2,
            &[
                Edge { u: 0, v: 1 },
                Edge {
                    u: 0,
                    v: usize::MAX
                }
            ]
        ),
        Err(Error::InvalidEndpoint {
            edge: 1,
            vertex: usize::MAX,
            vertices: 2
        })
    );
    assert!(bridges(0, &[Edge { u: 0, v: 0 }]).is_err());
}

#[test]
fn transformations() {
    cases(|n, edges| {
        let expected = bridges(n, edges).unwrap();
        let reversed: Vec<_> = edges.iter().map(|e| Edge { u: e.v, v: e.u }).collect();
        assert_eq!(bridges(n + 1, &reversed).unwrap(), expected);
        let reordered: Vec<_> = edges.iter().rev().copied().collect();
        let mut mapped: Vec<_> = bridges(n, &reordered)
            .unwrap()
            .into_iter()
            .map(|id| edges.len() - 1 - id)
            .collect();
        mapped.sort_unstable();
        assert_eq!(mapped, expected);
    });
}

#[test]
fn deep_chain_without_recursion() {
    let n = 30_000;
    let edges: Vec<_> = (1..n).map(|v| Edge { u: v - 1, v }).collect();
    assert_eq!(bridges(n, &edges).unwrap(), (0..n - 1).collect::<Vec<_>>());
}
