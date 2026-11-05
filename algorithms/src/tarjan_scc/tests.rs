use super::*;

// Independent bounded walk closure, without discovery times or lowlinks.
fn oracle(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut reach = vec![vec![false; n]; n];
    for (start, row_reach) in reach.iter_mut().enumerate() {
        row_reach[start] = true;
        for _ in 0..n {
            for (u, row) in graph.iter().enumerate() {
                if row_reach[u] {
                    for &v in row {
                        row_reach[v] = true;
                    }
                }
            }
        }
    }
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (v, _) in reach.iter().enumerate() {
        if let Some(group) = groups
            .iter_mut()
            .find(|group| reach[v][group[0]] && reach[group[0]][v])
        {
            group.push(v);
        } else {
            groups.push(vec![v]);
        }
    }
    groups
}
#[test]
fn exhaustive_mutual_reachability() {
    for n in 0..=4 {
        let slots: Vec<_> = (0..n)
            .flat_map(|u| (0..n).filter(move |&v| u != v).map(move |v| (u, v)))
            .collect();
        for mask in 0..1usize << slots.len() {
            let mut graph = vec![vec![]; n];
            for (i, &(u, v)) in slots.iter().enumerate() {
                if mask >> i & 1 == 1 {
                    graph[u].push(v);
                }
            }
            let result = components(&graph).unwrap();
            assert_eq!(result, oracle(&graph));
            assert_eq!(
                result,
                crate::strongly_connected_components::components(&graph).unwrap()
            );
        }
    }
}
#[test]
fn active_stack_cross_edges_and_finished_components() {
    let graph = vec![vec![1, 2], vec![0], vec![1, 3], vec![4], vec![3], vec![4]];
    assert_eq!(
        components(&graph).unwrap(),
        vec![vec![0, 1, 2], vec![3, 4], vec![5]]
    );
    assert_eq!(
        components(&[vec![], vec![0], vec![1]]).unwrap(),
        vec![vec![0], vec![1], vec![2]]
    );
}
#[test]
fn transpose_relabel_and_duplicate_fibers() {
    let graph = vec![vec![1], vec![2, 3], vec![0], vec![4], vec![3], vec![]];
    let expected = components(&graph).unwrap();
    let mut transpose = vec![vec![]; graph.len()];
    for (u, row) in graph.iter().enumerate() {
        for &v in row {
            transpose[v].extend([u, u]);
        }
    }
    assert_eq!(components(&transpose).unwrap(), expected);
    for (u, row) in transpose.iter_mut().enumerate() {
        row.push(u);
        row.reverse();
    }
    assert_eq!(components(&transpose).unwrap(), expected);
    let permutation = [4, 2, 0, 5, 1, 3];
    let mut mapped = vec![vec![]; 6];
    for (u, row) in graph.iter().enumerate() {
        mapped[permutation[u]] = row.iter().map(|&v| permutation[v]).collect();
    }
    let mut wanted: Vec<Vec<_>> = expected
        .iter()
        .map(|c| {
            let mut g: Vec<_> = c.iter().map(|&v| permutation[v]).collect();
            g.sort_unstable();
            g
        })
        .collect();
    wanted.sort_unstable_by_key(|c| c[0]);
    assert_eq!(components(&mapped).unwrap(), wanted);
}
#[test]
fn empty_validation_and_resource_precedence() {
    assert_eq!(components(&[]), Ok(vec![]));
    assert_eq!(components(&[vec![0, 0]]), Ok(vec![vec![0]]));
    assert_eq!(
        components(&[vec![], vec![3], vec![8]]),
        Err(Error::InvalidEdge {
            from: 1,
            to: 3,
            vertices: 3
        })
    );
    assert_eq!(
        components(&vec![vec![]; MAX_VERTICES + 1]),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        components(&[vec![9], vec![0; MAX_EDGES]]),
        Err(Error::LimitExceeded)
    );
}
#[test]
fn long_chain_then_cycle() {
    let n = 30_000;
    let mut graph: Vec<Vec<_>> = (0..n)
        .map(|u| if u + 1 < n { vec![u + 1] } else { vec![] })
        .collect();
    assert_eq!(
        components(&graph).unwrap(),
        (0..n).map(|u| vec![u]).collect::<Vec<_>>()
    );
    graph[n - 1].push(0);
    assert_eq!(
        components(&graph).unwrap(),
        vec![(0..n).collect::<Vec<_>>()]
    );
}
