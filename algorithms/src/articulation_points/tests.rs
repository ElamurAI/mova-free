use super::*;
use crate::bridges::tests::{cases, components};

#[test]
fn deletion_oracle() {
    cases(|n, edges| {
        let base = components(n, edges, None, None);
        let expected: Vec<_> = (0..n)
            .filter(|&v| components(n, edges, None, Some(v)) > base)
            .collect();
        assert_eq!(
            articulation_points(n, edges).unwrap(),
            expected,
            "n={n}, edges={edges:?}"
        );
    });
}

#[test]
fn roots_loops_parallel_disconnected() {
    let edges = [
        Edge { u: 0, v: 1 },
        Edge { u: 1, v: 2 },
        Edge { u: 2, v: 3 },
        Edge { u: 3, v: 1 },
        Edge { u: 1, v: 1 },
        Edge { u: 1, v: 2 },
        Edge { u: 4, v: 5 },
        Edge { u: 4, v: 6 },
    ];
    assert_eq!(articulation_points(8, &edges), Ok(vec![1, 4]));
    assert_eq!(articulation_points(0, &[]), Ok(vec![]));
    assert_eq!(articulation_points(4, &[]), Ok(vec![]));
    assert_eq!(articulation_points(2, &[Edge { u: 0, v: 1 }]), Ok(vec![]));
}

#[test]
fn validation_precedence() {
    assert_eq!(
        articulation_points(
            2,
            &[Edge {
                u: usize::MAX,
                v: 3
            }]
        ),
        Err(Error::InvalidEndpoint {
            edge: 0,
            vertex: usize::MAX,
            vertices: 2
        })
    );
    assert_eq!(
        articulation_points(2, &[Edge { u: 0, v: 2 }]),
        Err(Error::InvalidEndpoint {
            edge: 0,
            vertex: 2,
            vertices: 2
        })
    );
}

#[test]
fn transformations() {
    cases(|n, edges| {
        let expected = articulation_points(n, edges).unwrap();
        let reversed: Vec<_> = edges
            .iter()
            .rev()
            .map(|e| Edge { u: e.v, v: e.u })
            .collect();
        assert_eq!(articulation_points(n + 1, &reversed).unwrap(), expected);
        if n > 0 {
            let renamed: Vec<_> = edges
                .iter()
                .map(|e| Edge {
                    u: n - 1 - e.u,
                    v: n - 1 - e.v,
                })
                .collect();
            let mut mapped: Vec<_> = articulation_points(n, &renamed)
                .unwrap()
                .into_iter()
                .map(|v| n - 1 - v)
                .collect();
            mapped.sort_unstable();
            assert_eq!(mapped, expected);
        }
    });
}

#[test]
fn deep_chain_without_recursion() {
    let n = 30_000;
    let edges: Vec<_> = (1..n).map(|v| Edge { u: v - 1, v }).collect();
    assert_eq!(
        articulation_points(n, &edges).unwrap(),
        (1..n - 1).collect::<Vec<_>>()
    );
}
