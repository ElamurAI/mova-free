use super::*;
fn assert_tree(n: usize, edges: &[(usize, usize)], code: &[usize]) {
    assert_eq!(edges.len(), n.saturating_sub(1));
    let mut degree = vec![0; n];
    let mut reached = vec![false; n];
    for &(u, v) in edges {
        assert!(u < v && v < n);
        degree[u] += 1;
        degree[v] += 1;
    }
    if n > 0 {
        reached[0] = true;
        for _ in 0..n {
            for &(u, v) in edges {
                if reached[u] || reached[v] {
                    reached[u] = true;
                    reached[v] = true;
                }
            }
        }
        assert!(reached.iter().all(|&x| x));
    }
    for (v, &d) in degree.iter().enumerate() {
        assert_eq!(
            d,
            usize::from(n > 1) + code.iter().filter(|&&x| x == v).count()
        );
    }
    assert!(edges.windows(2).all(|p| p[0] < p[1]));
}
// Independent quadratic leaf elimination over an adjacency matrix.
fn reference_encode(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut a = vec![vec![false; n]; n];
    for &(u, v) in edges {
        a[u][v] = true;
        a[v][u] = true;
    }
    let mut out = Vec::new();
    for _ in 0..n.saturating_sub(2) {
        let u = (0..n)
            .find(|&u| a[u].iter().filter(|&&v| v).count() == 1)
            .unwrap();
        let v = (0..n).find(|&v| a[u][v]).unwrap();
        out.push(v);
        a[u][v] = false;
        a[v][u] = false;
    }
    out
}
#[test]
fn every_short_word_and_degree_law() {
    for n in 0usize..=6 {
        for mut word in 0..n.pow(n.saturating_sub(2) as u32).max(1) {
            let mut code = vec![0; n.saturating_sub(2)];
            for v in &mut code {
                *v = word % n;
                word /= n;
            }
            let edges = decode(n, &code).unwrap();
            assert_tree(n, &edges, &code);
            assert_eq!(reference_encode(n, &edges), code);
            assert_eq!(encode(n, &edges).unwrap(), code);
            let reversed: Vec<_> = edges.iter().rev().map(|&(u, v)| (v, u)).collect();
            assert_eq!(encode(n, &reversed).unwrap(), code);
        }
    }
}
#[test]
fn canonical_minimum_leaf() {
    let tree = [(4, 0), (4, 1), (4, 2), (4, 3), (3, 5)];
    assert_eq!(encode(6, &tree).unwrap(), vec![4, 4, 4, 3]);
    assert_eq!(decode(2, &[]).unwrap(), vec![(0, 1)]);
    assert_eq!(decode(0, &[]).unwrap(), vec![]);
    assert_eq!(decode(1, &[]).unwrap(), vec![]);
}
#[test]
fn reject_invalid_tree_and_code() {
    assert_eq!(encode(MAX_VERTICES + 1, &[]), Err(Error::TooManyVertices));
    assert_eq!(decode(MAX_VERTICES + 1, &[]), Err(Error::TooManyVertices));
    assert_eq!(encode(3, &[(0, 1)]), Err(Error::WrongLength));
    assert_eq!(decode(3, &[]), Err(Error::WrongLength));
    assert_eq!(
        decode(3, &[3]),
        Err(Error::InvalidLabel {
            position: 0,
            label: 3
        })
    );
    assert_eq!(
        encode(2, &[(0, 2)]),
        Err(Error::InvalidLabel {
            position: 0,
            label: 2
        })
    );
    assert_eq!(encode(2, &[(0, 0)]), Err(Error::SelfLoop { edge: 0 }));
    assert_eq!(
        encode(3, &[(0, 1), (1, 0)]),
        Err(Error::RepeatedEdge { edge: 1 })
    );
    assert_eq!(encode(4, &[(0, 1), (1, 2), (2, 0)]), Err(Error::NotTree));
}
