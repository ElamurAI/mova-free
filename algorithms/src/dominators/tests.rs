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
fn exhaustive_vertex_deletion() {
    for mask in 0..4096 {
        let g = graph(4, mask);
        for s in 0..4 {
            let actual = dominators(&g, s).unwrap();
            let reach = reachable(&g, s, None);
            for v in 0..4 {
                let expected: Vec<_> = (0..4)
                    .filter(|&d| reach[v] && !reachable(&g, s, Some(d))[v])
                    .collect();
                assert_eq!(actual[v], expected, "deletion characterization");
            }
        }
    }
}
#[test]
fn validation_and_duplicates() {
    assert_eq!(dominators(&[], 0), Err(Error::InvalidVertex));
    assert_eq!(
        dominators(&vec![vec![]; 257], 999),
        Err(Error::LimitExceeded)
    );
    assert_eq!(dominators(&[vec![1]], 0), Err(Error::InvalidVertex));
    assert_eq!(
        dominators(&[vec![0, 1, 1], vec![], vec![2]], 0).unwrap(),
        vec![vec![0], vec![0, 1], vec![]]
    );
}
