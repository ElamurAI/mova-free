use super::*;
fn traversal(g: &[Vec<usize>], source: usize) -> Vec<bool> {
    let mut seen = vec![false; g.len()];
    let mut pending = vec![source];
    while let Some(u) = pending.pop() {
        if seen[u] {
            continue;
        }
        seen[u] = true;
        pending.extend(g[u].iter().copied());
    }
    seen
}
#[test]
fn exhaustive_directed_graphs() {
    for n in 0..=4usize {
        for mask in 0..1usize << (n * n) {
            let g: Vec<Vec<_>> = (0..n)
                .map(|u| (0..n).filter(|&v| mask & (1 << (u * n + v)) != 0).collect())
                .collect();
            let c = transitive_closure(&g).unwrap();
            assert_eq!(c.vertices(), n);
            for u in 0..n {
                for (v, want) in traversal(&g, u).into_iter().enumerate() {
                    assert_eq!(c.reachable(u, v), Some(want));
                }
            }
        }
    }
}
#[test]
fn word_boundaries_and_direction() {
    for n in [1, 63, 64, 65, 127, 128, 129] {
        let g: Vec<Vec<_>> = (0..n)
            .map(|u| if u + 1 < n { vec![u + 1] } else { vec![] })
            .collect();
        let c = transitive_closure(&g).unwrap();
        for u in 0..n {
            for v in 0..n {
                assert_eq!(c.reachable(u, v), Some(u <= v));
            }
        }
        assert_eq!(c.reachable(n, 0), None);
        assert_eq!(c.reachable(0, n), None);
    }
}
#[test]
fn validation_limits_and_empty() {
    assert_eq!(
        transitive_closure(&[vec![], vec![3, 2]]),
        Err(Error::InvalidEdge {
            from: 1,
            to: 3,
            vertices: 2
        })
    );
    assert_eq!(
        transitive_closure(&vec![vec![]; 8193]),
        Err(Error::ResourceLimit)
    );
    assert_eq!(
        transitive_closure(&[vec![0; 1_000_001]]),
        Err(Error::ResourceLimit)
    );
    assert_eq!(transitive_closure(&[]).unwrap().reachable(0, 0), None);
}
#[test]
fn idempotence_and_duplicate_fibers() {
    let g = vec![vec![1, 1], vec![2], vec![]];
    let c = transitive_closure(&g).unwrap();
    let saturated: Vec<Vec<_>> = (0..3)
        .map(|u| {
            (0..3)
                .filter(|&v| c.reachable(u, v) == Some(true))
                .collect()
        })
        .collect();
    assert_eq!(transitive_closure(&saturated).unwrap(), c);
    assert_eq!(transitive_closure(&[vec![1], vec![2], vec![]]).unwrap(), c);
}
