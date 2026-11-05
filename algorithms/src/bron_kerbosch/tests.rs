use super::*;
fn brute(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut best = Vec::new();
    for mask in 0usize..1usize << n {
        let s: Vec<_> = (0..n).filter(|&v| mask & (1 << v) != 0).collect();
        if s.iter().enumerate().all(|(i, &u)| {
            s[i + 1..]
                .iter()
                .all(|&v| edges.contains(&(u, v)) || edges.contains(&(v, u)))
        }) && (s.len() > best.len() || (s.len() == best.len() && s < best))
        {
            best = s;
        }
    }
    best
}
#[test]
fn exhaustive_simple_graphs() {
    for n in 0..=5 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0usize..1 << pairs.len() {
            let edges: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, &e)| e)
                .collect();
            assert_eq!(maximum_clique(n, &edges).unwrap(), brute(n, &edges));
        }
    }
}
#[test]
fn lexicographic_maximum_and_bit_63() {
    assert_eq!(
        maximum_clique(4, &[(0, 2), (1, 2), (1, 3)]).unwrap(),
        vec![0, 2]
    );
    assert_eq!(
        maximum_clique(64, &[(63, 62), (0, 63)]).unwrap(),
        vec![0, 63]
    );
    assert_eq!(maximum_clique(64, &[]).unwrap(), vec![0]);
    let complete: Vec<_> = (0..64)
        .flat_map(|u| (u + 1..64).map(move |v| (u, v)))
        .collect();
    assert_eq!(
        maximum_clique(64, &complete).unwrap(),
        (0..64).collect::<Vec<_>>()
    );
}
#[test]
fn validate_simple_domain() {
    assert_eq!(maximum_clique(65, &[]), Err(Error::TooManyVertices));
    assert_eq!(maximum_clique(0, &[(0, 0)]), Err(Error::TooManyEdges));
    assert_eq!(
        maximum_clique(3, &[(0, 3)]),
        Err(Error::InvalidEndpoint { edge: 0, vertex: 3 })
    );
    assert_eq!(
        maximum_clique(3, &[(0, 0)]),
        Err(Error::SelfLoop { edge: 0 })
    );
    assert_eq!(
        maximum_clique(3, &[(0, 1), (1, 0)]),
        Err(Error::RepeatedEdge { edge: 1 })
    );
    assert_eq!(maximum_clique(1, &[]).unwrap(), vec![0]);
}
