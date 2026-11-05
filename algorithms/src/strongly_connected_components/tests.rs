use super::*;

fn oracle(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut reach = vec![vec![false; n]; n];
    for (u, edges) in graph.iter().enumerate() {
        reach[u][u] = true;
        for &v in edges {
            reach[u][v] = true;
        }
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                reach[u][v] |= reach[u][k] && reach[k][v];
            }
        }
    }
    let mut assigned = vec![false; n];
    let mut groups = vec![];
    for u in 0..n {
        if assigned[u] {
            continue;
        }
        let group: Vec<_> = (0..n).filter(|&v| reach[u][v] && reach[v][u]).collect();
        for &v in &group {
            assigned[v] = true;
        }
        groups.push(group);
    }
    groups
}

#[test]
fn boundaries_and_global_validation() {
    assert_eq!(components(&[]), Ok(vec![]));
    assert_eq!(
        components(&[vec![0, 0], vec![]]),
        Ok(vec![vec![0], vec![1]])
    );
    assert_eq!(
        components(&[vec![1], vec![0, 2], vec![3], vec![2], vec![]]),
        Ok(vec![vec![0, 1], vec![2, 3], vec![4]])
    );
    assert_eq!(
        components(&[vec![], vec![3], vec![9]]),
        Err(Error::InvalidEdge {
            from: 1,
            to: 3,
            vertices: 3
        })
    );
}

#[test]
fn all_small_graphs_match_mutual_reachability() {
    for n in 0..=3 {
        for mask in 0usize..1 << (n * n) {
            let graph: Vec<Vec<usize>> = (0..n)
                .map(|u| (0..n).filter(|&v| mask & (1 << (u * n + v)) != 0).collect())
                .collect();
            let before = graph.clone();
            assert_eq!(components(&graph), Ok(oracle(&graph)), "n={n},mask={mask}");
            assert_eq!(graph, before);
        }
    }
}

#[test]
fn transpose_and_parallel_arcs_preserve_seeded_partitions() {
    const SEED: u64 = 0x5cc020261029;
    let mut state = SEED;
    for case in 0..64 {
        let n = 4 + case % 13;
        let mut graph = vec![vec![]; n];
        for edges in &mut graph {
            for v in 0..n {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                if state >> 60 < 3 {
                    edges.push(v);
                }
            }
        }
        let mut transposed = vec![vec![]; n];
        for (u, edges) in graph.iter().enumerate() {
            for &v in edges {
                transposed[v].extend([u, u]);
            }
        }
        for edges in &mut transposed {
            edges.reverse();
        }
        let expected = oracle(&graph);
        assert_eq!(
            components(&graph),
            Ok(expected.clone()),
            "seed={SEED:#x},case={case}"
        );
        assert_eq!(
            components(&transposed),
            Ok(expected),
            "seed={SEED:#x},case={case},transpose"
        );
    }
}

#[test]
fn iterative_long_chain_and_cycle() {
    let n = 10_000;
    let mut graph: Vec<Vec<usize>> = (0..n)
        .map(|u| if u + 1 < n { vec![u + 1] } else { vec![] })
        .collect();
    assert_eq!(components(&graph), Ok((0..n).map(|u| vec![u]).collect()));
    graph[n - 1].push(0);
    assert_eq!(components(&graph), Ok(vec![(0..n).collect()]));
}
