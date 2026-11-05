use super::*;

// Exhaustive partial injections, independent of layered augmenting paths.
fn optimum(graph: &[Vec<usize>], row: usize, occupied: u64) -> usize {
    if row == graph.len() {
        return 0;
    }
    let mut best = optimum(graph, row + 1, occupied);
    for &right in &graph[row] {
        if occupied & (1 << right) == 0 {
            best = best.max(1 + optimum(graph, row + 1, occupied | (1 << right)));
        }
    }
    best
}
fn verify(graph: &[Vec<usize>], r: usize, mates: &[Option<usize>]) -> usize {
    assert_eq!(mates.len(), graph.len());
    let mut used = vec![false; r];
    for (left, mate) in mates.iter().enumerate() {
        if let Some(right) = *mate {
            assert!(graph[left].contains(&right));
            assert!(!used[right]);
            used[right] = true;
        }
    }
    used.iter().filter(|&&b| b).count()
}
#[test]
fn exhaustive_partial_injections() {
    for l in 0..=4 {
        for r in 0..=3 {
            for mask in 0..1usize << (l * r) {
                let graph: Vec<Vec<_>> = (0..l)
                    .map(|u| (0..r).filter(|&v| mask >> (u * r + v) & 1 == 1).collect())
                    .collect();
                let mates = maximum_matching(&graph, r).unwrap();
                assert_eq!(verify(&graph, r, &mates), optimum(&graph, 0, 0));
                assert_eq!(mates, maximum_matching(&graph, r).unwrap());
                let other = crate::bipartite_matching::bipartite_matching(&graph, r).unwrap();
                assert_eq!(verify(&graph, r, &other), verify(&graph, r, &mates));
            }
        }
    }
}
#[test]
fn alternating_reroute_and_duplicate_permutations() {
    let graph = vec![vec![0, 1], vec![0], vec![1, 2], vec![2, 3]];
    let mates = maximum_matching(&graph, 4).unwrap();
    assert_eq!(mates, vec![Some(1), Some(0), Some(2), Some(3)]);
    let duplicate: Vec<Vec<_>> = graph
        .iter()
        .map(|row| row.iter().rev().flat_map(|&r| [r, r]).collect())
        .collect();
    assert_eq!(
        verify(&duplicate, 4, &maximum_matching(&duplicate, 4).unwrap()),
        4
    );
    let complete = vec![vec![0, 1]; 2];
    assert_eq!(
        maximum_matching(&complete, 2).unwrap(),
        vec![Some(0), Some(1)]
    );
    assert_eq!(
        maximum_matching(&[vec![1, 0], vec![1, 0]], 2).unwrap(),
        vec![Some(1), Some(0)]
    );
}
#[test]
fn deep_augmenting_path_without_recursion() {
    let n = 20_000;
    let mut graph: Vec<Vec<_>> = (0..n - 1).map(|u| vec![u, u + 1]).collect();
    graph.push(vec![0]);
    let mates = maximum_matching(&graph, n).unwrap();
    assert_eq!(verify(&graph, n, &mates), n);
    assert_eq!(mates[n - 1], Some(0));
    assert_eq!(mates[0], Some(1));
}
#[test]
fn limits_and_first_error() {
    assert_eq!(maximum_matching(&[], usize::MAX), Err(Error::LimitExceeded));
    assert_eq!(
        maximum_matching(&vec![vec![]; MAX_VERTICES + 1], 0),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        maximum_matching(&[vec![3], vec![3]], MAX_VERTICES),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        maximum_matching(&[vec![9], vec![0; MAX_EDGES]], 1),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        maximum_matching(&[vec![2, 9], vec![3]], 2),
        Err(Error::InvalidEdge {
            left: 0,
            right: 2,
            right_vertices: 2
        })
    );
    assert_eq!(maximum_matching(&[], 0), Ok(vec![]));
    assert_eq!(maximum_matching(&[vec![], vec![]], 0), Ok(vec![None, None]));
}
#[test]
fn disconnected_additivity_and_projection_fibers() {
    let a = vec![vec![0], vec![0], vec![1]];
    let b = vec![vec![0, 1], vec![1]];
    let mut joined = a.clone();
    joined.extend(b.iter().map(|row| row.iter().map(|v| v + 2).collect()));
    let count = |g: &[Vec<usize>], r| verify(g, r, &maximum_matching(g, r).unwrap());
    assert_eq!(count(&joined, 4), count(&a, 2) + count(&b, 2));
    assert_eq!(
        maximum_matching(&[vec![0]], 1),
        maximum_matching(&[vec![0, 0]], 1)
    );
    assert_eq!(count(&[vec![0], vec![0]], 1), count(&[vec![], vec![0]], 1));
}
