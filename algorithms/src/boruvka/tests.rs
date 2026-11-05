use super::*;
// Independent connectivity closure, deliberately no DSU or existing MST code.
fn closure(n: usize, edges: &[Edge], mask: usize) -> Vec<Vec<bool>> {
    let mut r = vec![vec![false; n]; n];
    for (i, row) in r.iter_mut().enumerate() {
        row[i] = true;
    }
    for (i, e) in edges.iter().enumerate() {
        if mask & (1 << i) != 0 {
            r[e.u][e.v] = true;
            r[e.v][e.u] = true;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                r[i][j] |= r[i][k] && r[k][j];
            }
        }
    }
    r
}
fn oracle(n: usize, edges: &[Edge]) -> (i128, usize) {
    let target = closure(n, edges, (1 << edges.len()) - 1);
    let components = (0..n).filter(|&i| !(0..i).any(|j| target[i][j])).count();
    let mut best = None;
    for mask in 0usize..1 << edges.len() {
        if mask.count_ones() as usize != n - components {
            continue;
        }
        if closure(n, edges, mask) == target {
            let sum = edges
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, e)| i128::from(e.weight))
                .sum();
            best = Some(best.map_or(sum, |b: i128| b.min(sum)));
        }
    }
    (best.unwrap(), components)
}
#[test]
fn exhaustive_small_signed_forests() {
    let pairs = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    // Each possible edge is absent, negative, zero, or positive.
    for mut word in 0usize..4usize.pow(6) {
        let mut edges = Vec::new();
        for &(u, v) in &pairs {
            let d = word % 4;
            word /= 4;
            if d > 0 {
                edges.push(Edge {
                    u,
                    v,
                    weight: d as i64 - 2,
                });
            }
        }
        let got = minimum_spanning_forest(4, &edges).unwrap();
        assert_eq!((got.total_weight, got.components), oracle(4, &edges));
        assert_eq!(got.edge_indices.len(), 4 - got.components);
        let mask = got.edge_indices.iter().fold(0, |m, &i| m | (1 << i));
        assert_eq!(
            closure(4, &edges, mask),
            closure(4, &edges, (1 << edges.len()) - 1)
        );
    }
}
#[test]
fn signed_wide_total_and_ties() {
    let edges = [
        Edge {
            u: 0,
            v: 1,
            weight: i64::MIN,
        },
        Edge {
            u: 1,
            v: 2,
            weight: i64::MIN,
        },
        Edge {
            u: 0,
            v: 2,
            weight: i64::MAX,
        },
        Edge {
            u: 0,
            v: 0,
            weight: i64::MIN,
        },
        Edge {
            u: 1,
            v: 0,
            weight: i64::MIN,
        },
    ];
    let f = minimum_spanning_forest(4, &edges).unwrap();
    assert_eq!(f.total_weight, 2 * i128::from(i64::MIN));
    assert_eq!(f.edge_indices, vec![0, 1]);
    assert_eq!(f.components, 2);
    let positive = [
        Edge {
            u: 0,
            v: 1,
            weight: i64::MAX,
        },
        Edge {
            u: 1,
            v: 2,
            weight: i64::MAX,
        },
    ];
    assert_eq!(
        minimum_spanning_forest(3, &positive).unwrap().total_weight,
        2 * i128::from(i64::MAX)
    );
}
#[test]
fn boundary_and_validation() {
    assert_eq!(
        minimum_spanning_forest(0, &[]).unwrap(),
        Forest {
            edge_indices: vec![],
            total_weight: 0,
            components: 0
        }
    );
    assert_eq!(minimum_spanning_forest(3, &[]).unwrap().components, 3);
    assert_eq!(
        minimum_spanning_forest(MAX_VERTICES + 1, &[]),
        Err(Error::TooManyVertices)
    );
    assert_eq!(
        minimum_spanning_forest(
            0,
            &[Edge {
                u: 0,
                v: 0,
                weight: 0
            }]
        ),
        Err(Error::InvalidEndpoint { edge: 0, vertex: 0 })
    );
    assert_eq!(
        minimum_spanning_forest(
            2,
            &[Edge {
                u: 0,
                v: 2,
                weight: 0
            }]
        ),
        Err(Error::InvalidEndpoint { edge: 0, vertex: 2 })
    );
    let too_many = vec![
        Edge {
            u: 0,
            v: 0,
            weight: 0
        };
        MAX_EDGES + 1
    ];
    assert_eq!(
        minimum_spanning_forest(1, &too_many),
        Err(Error::TooManyEdges)
    );
}
