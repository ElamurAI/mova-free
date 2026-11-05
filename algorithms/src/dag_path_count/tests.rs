use super::*;

fn enumerate(graph: &[Vec<usize>], at: usize, counts: &mut [u128]) {
    counts[at] += 1;
    for &to in &graph[at] {
        enumerate(graph, to, counts);
    }
}

#[test]
fn exhaustive_path_enumeration() {
    // Every multiplicity 0..2 on the six forward pairs; all sources and
    // vertex permutations through reversal test non-numeric topological order.
    for mut code in 0..3usize.pow(6) {
        let mut graph = vec![vec![]; 4];
        for (from, row) in graph.iter_mut().enumerate() {
            for to in from + 1..4 {
                for _ in 0..code % 3 {
                    row.push(to);
                }
                code /= 3;
            }
        }
        for source in 0..4 {
            let mut expected = vec![0; 4];
            enumerate(&graph, source, &mut expected);
            assert_eq!(dag_path_count(&graph, source), Ok(expected.clone()));
            let mut reversed = vec![vec![]; 4];
            for (u, row) in graph.iter().enumerate() {
                for &v in row {
                    reversed[3 - u].push(3 - v);
                }
            }
            expected.reverse();
            assert_eq!(dag_path_count(&reversed, 3 - source), Ok(expected));
        }
    }
}

#[test]
fn validation_precedence_and_global_cycles() {
    assert_eq!(
        dag_path_count(&[], 0),
        Err(Error::InvalidSource {
            source: 0,
            vertices: 0
        })
    );
    assert_eq!(
        dag_path_count(&[vec![9]], 1),
        Err(Error::InvalidSource {
            source: 1,
            vertices: 1
        })
    );
    assert_eq!(
        dag_path_count(&[vec![0], vec![9]], 0),
        Err(Error::Graph(TopologicalSortError::InvalidEdge {
            from: 1,
            to: 9,
            vertices: 2
        }))
    );
    assert_eq!(
        dag_path_count(&[vec![], vec![2], vec![1]], 0),
        Err(Error::Graph(TopologicalSortError::Cycle))
    );
    assert_eq!(
        dag_path_count(&[vec![0]], 0),
        Err(Error::Graph(TopologicalSortError::Cycle))
    );
}

#[test]
fn exact_maximum_and_overflow() {
    let mut graph = vec![vec![]; 130];
    // Doubling produces 2^127; add all powers into vertex 128 to get MAX.
    for (i, row) in graph.iter_mut().enumerate().take(127) {
        row.extend([i + 1, i + 1]);
    }
    for row in graph.iter_mut().take(128) {
        row.push(128);
    }
    let counts = dag_path_count(&graph, 0).unwrap();
    assert_eq!(counts[127], 1u128 << 127);
    assert_eq!(counts[128], u128::MAX);
    graph[127].push(128);
    assert_eq!(
        dag_path_count(&graph, 0),
        Err(Error::Overflow { from: 127, to: 128 })
    );
    // A cycle elsewhere wins over an otherwise reachable overflowing count.
    graph[129].push(129);
    assert_eq!(
        dag_path_count(&graph, 0),
        Err(Error::Graph(TopologicalSortError::Cycle))
    );
}

#[test]
fn unreachable_exponential_component_and_long_chain() {
    let mut graph = vec![vec![]; 200];
    for (i, row) in graph.iter_mut().enumerate().take(199).skip(1) {
        row.extend([i + 1, i + 1]);
    }
    let mut expected = vec![0; 200];
    expected[0] = 1;
    assert_eq!(dag_path_count(&graph, 0), Ok(expected));
    let n = 30_000;
    let mut chain = vec![vec![]; n];
    for (i, row) in chain.iter_mut().enumerate().skip(1) {
        row.push(i - 1);
    }
    assert_eq!(dag_path_count(&chain, n - 1), Ok(vec![1; n]));
}
