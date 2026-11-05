use super::*;
use crate::bridges::tests::cases;

// Enumerate trails directly, without degrees, connectivity or production adjacency.
fn possible(edges: &[Edge], v: usize, mask: usize) -> bool {
    if mask.count_ones() as usize == edges.len() {
        return true;
    }
    edges.iter().enumerate().any(|(id, e)| {
        mask & (1 << id) == 0
            && (e.u == v || e.v == v)
            && possible(edges, if e.u == v { e.v } else { e.u }, mask | (1 << id))
    })
}
fn validate(n: usize, edges: &[Edge], trail: &Trail) {
    assert_eq!(trail.edges.len(), edges.len());
    assert_eq!(
        trail.vertices.len(),
        if n == 0 { 0 } else { edges.len() + 1 }
    );
    let mut ids = trail.edges.clone();
    ids.sort_unstable();
    assert_eq!(ids, (0..edges.len()).collect::<Vec<_>>());
    for (&id, pair) in trail.edges.iter().zip(trail.vertices.windows(2)) {
        let e = edges[id];
        assert!(pair == [e.u, e.v] || pair == [e.v, e.u]);
    }
    assert!(trail.vertices.iter().all(|&v| v < n));
}

#[test]
fn exhaustive_trail_oracle() {
    cases(|n, edges| {
        let expected = edges.is_empty() || (0..n).any(|start| possible(edges, start, 0));
        let actual = eulerian_trail(n, edges).unwrap();
        assert_eq!(actual.is_some(), expected, "n={n}, edges={edges:?}");
        if let Some(trail) = actual {
            validate(n, edges, &trail);
        }
    });
}

#[test]
fn canonical_and_disconnected() {
    let edges = [
        Edge { u: 0, v: 1 },
        Edge { u: 0, v: 1 },
        Edge { u: 1, v: 1 },
        Edge { u: 1, v: 2 },
    ];
    assert_eq!(
        eulerian_trail(4, &edges),
        Ok(Some(Trail {
            vertices: vec![1, 0, 1, 1, 2],
            edges: vec![0, 1, 2, 3]
        }))
    );
    assert_eq!(
        eulerian_trail(0, &[]),
        Ok(Some(Trail {
            vertices: vec![],
            edges: vec![]
        }))
    );
    assert_eq!(
        eulerian_trail(3, &[]),
        Ok(Some(Trail {
            vertices: vec![0],
            edges: vec![]
        }))
    );
    assert_eq!(
        eulerian_trail(2, &[Edge { u: 0, v: 0 }, Edge { u: 1, v: 1 }]),
        Ok(None)
    );
    assert_eq!(
        eulerian_trail(
            4,
            &[
                Edge { u: 0, v: 1 },
                Edge { u: 0, v: 2 },
                Edge { u: 0, v: 3 }
            ]
        ),
        Ok(None)
    );
}

#[test]
fn validation_precedence() {
    assert_eq!(
        eulerian_trail(
            0,
            &[Edge {
                u: usize::MAX,
                v: 0
            }]
        ),
        Err(Error::InvalidEndpoint {
            edge: 0,
            vertex: usize::MAX,
            vertices: 0
        })
    );
    assert_eq!(
        eulerian_trail(2, &[Edge { u: 0, v: 1 }, Edge { u: 0, v: 2 }]),
        Err(Error::InvalidEndpoint {
            edge: 1,
            vertex: 2,
            vertices: 2
        })
    );
}

#[test]
fn transformations() {
    cases(|n, edges| {
        let actual = eulerian_trail(n, edges).unwrap();
        let reversed: Vec<_> = edges.iter().map(|e| Edge { u: e.v, v: e.u }).collect();
        assert_eq!(eulerian_trail(n, &reversed).unwrap(), actual);
        if !edges.is_empty() {
            assert_eq!(eulerian_trail(n + 1, edges).unwrap(), actual);
        }
    });
}

#[test]
fn deep_chain_without_recursion() {
    let n = 30_000;
    let edges: Vec<_> = (1..n).map(|v| Edge { u: v - 1, v }).collect();
    let trail = eulerian_trail(n, &edges).unwrap().unwrap();
    validate(n, &edges, &trail);
    assert_eq!(trail.vertices, (0..n).collect::<Vec<_>>());
}
