use super::*;

// Independent exponential oracle: enumerate connected degree-two edge subsets,
// then take the transitive closure of common-cycle membership.
fn oracle(n: usize, edges: &[Edge]) -> Vec<Vec<usize>> {
    let m = edges.len();
    let mut same = vec![vec![false; m]; m];
    for (i, row) in same.iter_mut().enumerate() {
        row[i] = true;
    }
    for mask in 1usize..(1 << m) {
        let ids: Vec<_> = (0..m).filter(|i| mask & (1 << i) != 0).collect();
        let mut degree = vec![0; n];
        let mut reach = vec![vec![false; n]; n];
        for &id in &ids {
            let Edge { u, v } = edges[id];
            degree[u] += 1;
            degree[v] += 1;
            reach[u][v] = true;
            reach[v][u] = true;
        }
        if degree.iter().any(|&d| d != 0 && d != 2) {
            continue;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    reach[i][j] |= reach[i][k] && reach[k][j];
                }
            }
        }
        let start = edges[ids[0]].u;
        if (0..n).any(|v| degree[v] != 0 && !reach[start][v]) {
            continue;
        }
        for &i in &ids {
            for &j in &ids {
                same[i][j] = true;
            }
        }
    }
    for k in 0..m {
        for i in 0..m {
            for j in 0..m {
                same[i][j] |= same[i][k] && same[k][j];
            }
        }
    }
    let mut used = vec![false; m];
    let mut result = Vec::new();
    for i in 0..m {
        if used[i] {
            continue;
        }
        let block: Vec<_> = (i..m).filter(|&j| same[i][j]).collect();
        for &j in &block {
            used[j] = true;
        }
        result.push(block);
    }
    result
}

fn check(n: usize, edges: &[Edge]) {
    let expected = oracle(n, edges);
    assert_eq!(
        biconnected_components(n, edges),
        Ok(expected.clone()),
        "{edges:?}"
    );
    let reversed: Vec<_> = edges.iter().map(|e| Edge { u: e.v, v: e.u }).collect();
    assert_eq!(
        biconnected_components(n + 1, &reversed),
        Ok(expected.clone())
    );
    let mut partition: Vec<_> = expected.into_iter().flatten().collect();
    partition.sort_unstable();
    assert_eq!(partition, (0..edges.len()).collect::<Vec<_>>());
}

#[test]
fn cycle_subset_oracle() {
    for n in 0..=3 {
        let choices: Vec<_> = (0..n)
            .flat_map(|u| (u..n).map(move |v| Edge { u, v }))
            .collect();
        for len in 0..=4u32 {
            for mut code in 0..choices.len().pow(len) {
                let edges: Vec<_> = (0..len)
                    .map(|_| {
                        let e = choices[code % choices.len()];
                        code /= choices.len();
                        e
                    })
                    .collect();
                check(n, &edges);
            }
        }
    }
    let choices: Vec<_> = (0..5)
        .flat_map(|u| (u + 1..5).map(move |v| Edge { u, v }))
        .collect();
    for mask in 0..1usize << choices.len() {
        let edges: Vec<_> = choices
            .iter()
            .enumerate()
            .filter_map(|(i, &e)| (mask & (1 << i) != 0).then_some(e))
            .collect();
        check(5, &edges);
    }
}

#[test]
fn articulation_parallel_loops() {
    let edges = [
        (0, 1),
        (1, 2),
        (2, 0),
        (2, 3),
        (3, 4),
        (4, 2),
        (4, 5),
        (5, 5),
        (0, 1),
    ];
    let edges: Vec<_> = edges.into_iter().map(|(u, v)| Edge { u, v }).collect();
    assert_eq!(
        biconnected_components(7, &edges),
        Ok(vec![vec![0, 1, 2, 8], vec![3, 4, 5], vec![6], vec![7]])
    );
    assert_eq!(biconnected_components(0, &[]), Ok(vec![]));
}

#[test]
fn endpoint_precedence() {
    assert_eq!(
        biconnected_components(0, &[Edge { u: 2, v: 1 }]),
        Err(Error::InvalidEndpoint {
            edge: 0,
            vertex: 2,
            vertices: 0
        })
    );
    assert_eq!(
        biconnected_components(
            2,
            &[
                Edge {
                    u: 0,
                    v: usize::MAX
                },
                Edge { u: 2, v: 0 }
            ]
        ),
        Err(Error::InvalidEndpoint {
            edge: 0,
            vertex: usize::MAX,
            vertices: 2
        })
    );
}

#[test]
fn long_chain_without_recursion() {
    let n = 30_000;
    let edges: Vec<_> = (1..n).map(|v| Edge { u: v - 1, v }).collect();
    let got = biconnected_components(n, &edges).unwrap();
    assert_eq!(got, (0..n - 1).map(|i| vec![i]).collect::<Vec<_>>());
}
