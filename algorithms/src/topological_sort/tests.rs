use super::*;
#[test]
fn boundaries_and_validation() {
    assert_eq!(topological_sort(&[]), Ok(vec![]));
    assert_eq!(
        topological_sort(&[vec![], vec![], vec![]]),
        Ok(vec![0, 1, 2])
    );
    assert_eq!(
        topological_sort(&[vec![0]]),
        Err(TopologicalSortError::Cycle)
    );
    assert_eq!(
        topological_sort(&[vec![1], vec![0], vec![]]),
        Err(TopologicalSortError::Cycle)
    );
    assert_eq!(
        topological_sort(&[vec![0], vec![2]]),
        Err(TopologicalSortError::InvalidEdge {
            from: 1,
            to: 2,
            vertices: 2
        })
    );
    assert_eq!(
        topological_sort(&[vec![2, 2], vec![2], vec![]]),
        Ok(vec![0, 1, 2])
    );
    assert_eq!(
        topological_sort(&[vec![1], vec![], vec![]]),
        Ok(vec![0, 1, 2])
    );
    assert_eq!(
        topological_sort(&[vec![], vec![0], vec![]]),
        Ok(vec![1, 0, 2])
    );
}

fn permutations(prefix: &mut Vec<usize>, n: usize, out: &mut Vec<Vec<usize>>) {
    if prefix.len() == n {
        out.push(prefix.clone());
        return;
    }
    for v in 0..n {
        if !prefix.contains(&v) {
            prefix.push(v);
            permutations(prefix, n, out);
            prefix.pop();
        }
    }
}
#[test]
fn exhaustive_permutation_oracle() {
    for n in 0..=3 {
        let mut orders = vec![];
        permutations(&mut vec![], n, &mut orders);
        for mask in 0usize..(1 << (n * n)) {
            let graph: Vec<Vec<usize>> = (0..n)
                .map(|u| (0..n).filter(|&v| mask & (1 << (u * n + v)) != 0).collect())
                .collect();
            let before = graph.clone();
            let expected = orders
                .iter()
                .find(|order| {
                    let mut position = vec![0; n];
                    for (i, &v) in order.iter().enumerate() {
                        position[v] = i;
                    }
                    graph
                        .iter()
                        .enumerate()
                        .all(|(u, edges)| edges.iter().all(|&v| position[u] < position[v]))
                })
                .cloned()
                .ok_or(TopologicalSortError::Cycle);
            assert_eq!(topological_sort(&graph), expected, "n={n}, mask={mask}");
            assert_eq!(graph, before);
        }
    }
}

fn next_word(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

fn permutation_oracle(graph: &[Vec<usize>]) -> Result<Vec<usize>, TopologicalSortError> {
    let mut orders = vec![];
    permutations(&mut vec![], graph.len(), &mut orders);
    orders
        .into_iter()
        .find(|order| {
            let mut positions = vec![0; graph.len()];
            for (i, &v) in order.iter().enumerate() {
                positions[v] = i;
            }
            graph
                .iter()
                .enumerate()
                .all(|(u, edges)| edges.iter().all(|&v| positions[u] < positions[v]))
        })
        .ok_or(TopologicalSortError::Cycle)
}

#[test]
fn seeded_dags_match_independent_permutation_oracle() {
    const SEED: u64 = 0x7010_2026_1029;
    let mut state = SEED;
    for case in 0..32 {
        let n = 4 + case % 4;
        let mut hidden_order: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            let j = (next_word(&mut state) % (i + 1) as u64) as usize;
            hidden_order.swap(i, j);
        }
        let mut graph = vec![vec![]; n];
        for (i, &u) in hidden_order.iter().enumerate() {
            for &v in &hidden_order[i + 1..] {
                if next_word(&mut state) >> 63 == 0 {
                    graph[u].push(v);
                }
            }
        }
        let before = graph.clone();
        let expected = permutation_oracle(&graph);
        assert_eq!(
            topological_sort(&graph),
            expected,
            "seed={SEED:#x}, case={case}"
        );
        assert_eq!(graph, before, "seed={SEED:#x}, case={case}");
    }
}

#[test]
fn duplicate_reordered_edges_preserve_dynamic_lexicographic_choice() {
    // Several smaller labels become ready after larger vertices are emitted.
    let graph = vec![
        vec![],
        vec![0],
        vec![1, 4],
        vec![1, 4, 6],
        vec![],
        vec![3, 2],
        vec![],
    ];
    let expected = permutation_oracle(&graph);
    assert_eq!(expected, Ok(vec![5, 2, 3, 1, 0, 4, 6]));
    let duplicated: Vec<Vec<usize>> = graph
        .iter()
        .map(|edges| edges.iter().rev().flat_map(|&v| [v, v, v]).collect())
        .collect();
    assert_eq!(topological_sort(&graph), expected);
    assert_eq!(topological_sort(&duplicated), expected);
}

#[test]
fn long_cycle_witness_survives_disconnected_vertices() {
    let n = 96;
    let mut graph: Vec<Vec<usize>> = (0..n)
        .map(|u| if u + 1 < n { vec![u + 1] } else { vec![] })
        .collect();
    graph.extend([vec![], vec![]]);
    assert_eq!(topological_sort(&graph), Ok((0..n + 2).collect()));
    // The existing chain 17->...->95 plus this back edge witnesses a cycle.
    graph[n - 1].push(17);
    assert_eq!(topological_sort(&graph), Err(TopologicalSortError::Cycle));
    graph[n - 1].clear();
    assert_eq!(topological_sort(&graph), Ok((0..n + 2).collect()));
}
