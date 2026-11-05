use super::*;

// Independent subset enumeration: no augmenting paths or mate reassignment.
fn oracle(graph: &[Vec<usize>]) -> usize {
    let edges: Vec<_> = graph
        .iter()
        .enumerate()
        .flat_map(|(l, es)| es.iter().map(move |&r| (l, r)))
        .collect();
    let mut best = 0;
    for mask in 0..(1usize << edges.len()) {
        let chosen: Vec<_> = edges
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &e)| e)
            .collect();
        if chosen
            .iter()
            .enumerate()
            .all(|(i, &(l, r))| chosen[..i].iter().all(|&(a, b)| a != l && b != r))
        {
            best = best.max(chosen.len());
        }
    }
    best
}

#[test]
fn exhaustive_small_graphs() {
    for left in 0..=3usize {
        for right in 0..=3usize {
            for mask in 0..(1usize << (left * right)) {
                let graph: Vec<Vec<_>> = (0..left)
                    .map(|l| {
                        (0..right)
                            .filter(|&r| mask & (1 << (l * right + r)) != 0)
                            .collect()
                    })
                    .collect();
                let result = bipartite_matching(&graph, right).unwrap();
                assert_eq!(result.len(), left);
                let mut used = vec![false; right];
                for (l, &mate) in result.iter().enumerate() {
                    if let Some(r) = mate {
                        assert!(graph[l].contains(&r));
                        assert!(!used[r]);
                        used[r] = true;
                    }
                }
                assert_eq!(result.iter().flatten().count(), oracle(&graph), "{graph:?}");
                assert_eq!(bipartite_matching(&graph, right).unwrap(), result);
            }
        }
    }
}

#[test]
fn reassignment_duplicates_and_ties() {
    assert_eq!(
        bipartite_matching(&[vec![0, 1], vec![0]], 2),
        Ok(vec![Some(1), Some(0)])
    );
    assert_eq!(
        bipartite_matching(&[vec![1, 0, 1], vec![1, 0]], 2),
        Ok(vec![Some(1), Some(0)])
    );
    assert_eq!(
        bipartite_matching(&[vec![0, 0], vec![0]], 1),
        Ok(vec![Some(0), None])
    );
    assert_eq!(
        bipartite_matching(&[vec![], vec![]], 0),
        Ok(vec![None, None])
    );
    assert_eq!(bipartite_matching(&[], 3), Ok(vec![]));
    let n = 1024;
    let mut graph: Vec<_> = (0..n - 1).map(|l| vec![l, l + 1]).collect();
    graph.push(vec![0]);
    let result = bipartite_matching(&graph, n).unwrap();
    assert_eq!(result[n - 1], Some(0));
    for (l, mate) in result[..n - 1].iter().enumerate() {
        assert_eq!(*mate, Some(l + 1));
    }
}

#[test]
fn validates_all_edges_first() {
    assert_eq!(
        bipartite_matching(&[vec![0, 3], vec![9]], 1),
        Err(BipartiteMatchingError::InvalidEdge {
            left: 0,
            right: 3,
            right_vertices: 1
        })
    );
    assert_eq!(
        bipartite_matching(&[vec![0]], 0),
        Err(BipartiteMatchingError::InvalidEdge {
            left: 0,
            right: 0,
            right_vertices: 0
        })
    );
}

fn graph_from_mask(mask: usize) -> Vec<Vec<usize>> {
    (0..3)
        .map(|l| (0..3).filter(|&r| mask & (1 << (3 * l + r)) != 0).collect())
        .collect()
}

#[test]
fn residual_cover_certifies_maximal_cardinality() {
    for mask in 0..512 {
        let graph = graph_from_mask(mask);
        let result = bipartite_matching(&graph, 3).unwrap();
        for (l, mate) in result.iter().enumerate() {
            if let Some(r) = mate {
                assert!(graph[l].contains(r));
                assert!(!result[..l].contains(&Some(*r)));
            }
        }
        let mut left_seen: Vec<_> = result.iter().map(Option::is_none).collect();
        let mut right_seen = [false; 3];
        for _ in 0..3 {
            for l in 0..3 {
                if left_seen[l] {
                    for &r in &graph[l] {
                        if result[l] != Some(r) {
                            right_seen[r] = true;
                        }
                    }
                }
            }
            for l in 0..3 {
                if result[l].is_some_and(|r| right_seen[r]) {
                    left_seen[l] = true;
                }
            }
        }
        // No reached right vertex is free: no augmenting path remains.
        for (r, reached) in right_seen.iter().enumerate() {
            assert!(!reached || result.contains(&Some(r)));
        }
        for (l, es) in graph.iter().enumerate() {
            for &r in es {
                assert!(!left_seen[l] || right_seen[r]);
            }
        }
        let cover =
            left_seen.iter().filter(|&&x| !x).count() + right_seen.iter().filter(|&&x| x).count();
        assert_eq!(cover, result.iter().flatten().count());
        if mask != 0 {
            // The empty cover lies outside the edge-cover relation.
            assert!(graph.iter().any(|es| !es.is_empty()));
        }
    }
}

#[test]
fn edge_addition_and_transpose_cardinality_relations() {
    for mask in 0..512 {
        let graph = graph_from_mask(mask);
        let k = bipartite_matching(&graph, 3)
            .unwrap()
            .iter()
            .flatten()
            .count();
        let transpose: Vec<Vec<_>> = (0..3)
            .map(|r| (0..3).filter(|&l| graph[l].contains(&r)).collect())
            .collect();
        assert_eq!(
            bipartite_matching(&transpose, 3)
                .unwrap()
                .iter()
                .flatten()
                .count(),
            k
        );
        for bit in 0..9 {
            let extended = graph_from_mask(mask | (1 << bit));
            let next = bipartite_matching(&extended, 3)
                .unwrap()
                .iter()
                .flatten()
                .count();
            assert!((k..=k + 1).contains(&next));
        }
    }
    // Cardinality is preserved by row permutation, the returned vector is not.
    assert_ne!(
        bipartite_matching(&[vec![0], vec![]], 1).unwrap(),
        bipartite_matching(&[vec![], vec![0]], 1).unwrap()
    );
}

#[test]
fn exact_output_reconstruction_preserves_ambiguity() {
    let mut fibers = std::collections::BTreeMap::new();
    for mask in 0..512 {
        let graph = graph_from_mask(mask);
        let result = bipartite_matching(&graph, 3).unwrap();
        *fibers.entry(result.clone()).or_insert(0usize) += 1;
        let reconstructed: Vec<Vec<_>> = result
            .iter()
            .map(|mate| mate.iter().copied().collect())
            .collect();
        let duplicated: Vec<Vec<_>> = result
            .iter()
            .map(|mate| mate.iter().flat_map(|&r| [r, r]).collect())
            .collect();
        assert_eq!(bipartite_matching(&reconstructed, 3).unwrap(), result);
        assert_eq!(bipartite_matching(&duplicated, 3).unwrap(), result);
    }
    assert!(fibers.values().any(|&size| size > 1));
    let impossible = vec![Some(0), Some(0), None];
    assert!(!fibers.contains_key(&impossible));
    // A valid maximum matching need not belong to the exact deterministic fiber.
    let graph = [vec![0, 1], vec![0, 1]];
    assert_ne!(
        bipartite_matching(&graph, 2).unwrap(),
        vec![Some(1), Some(0)]
    );
}
