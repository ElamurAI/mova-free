use super::*;
fn graph(n: usize, mask: usize) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; n];
    let mut bit = 0;
    for (u, row) in g.iter_mut().enumerate() {
        for v in 0..n {
            if u != v {
                if mask >> bit & 1 != 0 {
                    row.push(v);
                }
                bit += 1;
            }
        }
    }
    g
}
fn reachable(g: &[Vec<usize>], s: usize, removed: Option<usize>) -> Vec<bool> {
    let mut seen = vec![false; g.len()];
    if removed == Some(s) {
        return seen;
    }
    seen[s] = true;
    let mut q = vec![s];
    while let Some(v) = q.pop() {
        for &w in &g[v] {
            if !seen[w] && removed != Some(w) {
                seen[w] = true;
                q.push(w);
            }
        }
    }
    seen
}

#[test]
fn exhaustive_edge_deletion() {
    for mask in 0..4096 {
        let g = graph(4, mask);
        let cyclic = (0..4).any(|u| g[u].iter().any(|&v| reachable(&g, v, None)[u]));
        let result = transitive_reduction_dag(&g);
        if cyclic {
            assert_eq!(result, Err(Error::Cycle));
            continue;
        }
        let actual = result.unwrap();
        let mut expected = vec![vec![]; 4];
        for u in 0..4 {
            for &v in &g[u] {
                let mut removed = g.clone();
                removed[u].retain(|&w| w != v);
                if !reachable(&removed, u, None)[v] {
                    expected[u].push(v);
                }
            }
        }
        assert_eq!(actual, expected, "essential edges");
        for s in 0..4 {
            assert_eq!(reachable(&actual, s, None), reachable(&g, s, None));
        }
        assert_eq!(transitive_reduction_dag(&actual).unwrap(), actual);
    }
}
#[test]
fn validation_and_duplicates() {
    assert_eq!(transitive_reduction_dag(&[vec![0]]), Err(Error::Cycle));
    assert_eq!(
        transitive_reduction_dag(&[vec![1]]),
        Err(Error::InvalidVertex)
    );
    assert_eq!(
        transitive_reduction_dag(&vec![vec![]; 257]),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        transitive_reduction_dag(&[vec![1, 1, 2], vec![2], vec![]]).unwrap(),
        vec![vec![1], vec![2], vec![]]
    );
}
