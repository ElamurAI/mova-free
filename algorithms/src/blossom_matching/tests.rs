use super::*;
fn oracle(n: usize, edges: &[(usize, usize)], mask: usize) -> usize {
    if mask == 0 {
        return 0;
    }
    let v = mask.trailing_zeros() as usize;
    let rest = mask & !(1 << v);
    let mut best = oracle(n, edges, rest);
    for u in v + 1..n {
        if rest & (1 << u) != 0 && edges.contains(&(v, u)) {
            best = best.max(1 + oracle(n, edges, rest & !(1 << u)));
        }
    }
    best
}
#[test]
fn exhaustive_graphs() {
    for n in 0..=6 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for bits in 0..(1usize << pairs.len()) {
            let edges: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (bits & (1 << i) != 0).then_some(e))
                .collect();
            let m = maximum_matching(n, &edges).unwrap();
            for (v, &u) in m.iter().enumerate() {
                if let Some(u) = u {
                    assert_ne!(v, u);
                    assert_eq!(m[u], Some(v));
                    assert!(edges.contains(&(v.min(u), v.max(u))));
                }
            }
            assert_eq!(
                m.iter().filter(|x| x.is_some()).count() / 2,
                oracle(n, &edges, (1 << n) - 1)
            );
        }
    }
}
#[test]
fn boundaries_and_flower() {
    let e = [(0, 1), (1, 2), (2, 0), (1, 3), (2, 4), (4, 5)];
    assert_eq!(
        maximum_matching(6, &e)
            .unwrap()
            .iter()
            .filter(|x| x.is_some())
            .count(),
        6
    );
    let mut reversed = e.to_vec();
    reversed.reverse();
    reversed.push((0, 0));
    reversed.push((0, 1));
    assert_eq!(maximum_matching(6, &e), maximum_matching(6, &reversed));
    assert_eq!(maximum_matching(0, &[]), Ok(vec![]));
    assert_eq!(
        maximum_matching(0, &[(0, 0)]),
        Err(Error::InvalidEdge { index: 0 })
    );
    assert_eq!(
        maximum_matching(MAX_VERTICES + 1, &[]),
        Err(Error::LimitExceeded)
    );
}
