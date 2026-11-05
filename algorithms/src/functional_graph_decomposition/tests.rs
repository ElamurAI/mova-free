use super::*;
#[test]
fn exhaustive_successors() {
    for n in 0..=6usize {
        for mut code in 0..n.pow(n as u32) {
            let mut s = vec![0; n];
            for x in &mut s {
                *x = code % n;
                code /= n;
            }
            let d = decompose(&s).unwrap();
            for start in 0..n {
                let mut path = Vec::new();
                let mut v = start;
                while !path.contains(&v) {
                    path.push(v);
                    v = s[v];
                }
                let distance = path.iter().position(|&x| x == v).unwrap();
                assert_eq!(d.distance[start], distance);
                assert_eq!(d.entry[start], v);
                let cycle = &d.cycles[d.cycle_id[start]];
                assert_eq!(cycle.len(), path.len() - distance);
                assert_eq!(cycle[0], *cycle.iter().min().unwrap());
                for i in 0..cycle.len() {
                    assert_eq!(s[cycle[i]], cycle[(i + 1) % cycle.len()]);
                }
            }
            assert!(d.cycles.windows(2).all(|p| p[0][0] < p[1][0]));
        }
    }
}
#[test]
fn coordinates_and_errors() {
    let d = decompose(&[2, 0, 3, 2, 4]).unwrap();
    assert_eq!(d.cycles, vec![vec![2, 3], vec![4]]);
    assert_eq!(d.entry, vec![2, 2, 2, 3, 4]);
    assert_eq!(d.distance, vec![1, 2, 0, 0, 0]);
    assert_eq!(decompose(&[1]), Err(Error::InvalidSuccessor { vertex: 0 }));
    assert!(decompose(&[]).unwrap().cycles.is_empty());
    assert_eq!(
        decompose(&vec![0; MAX_VERTICES + 1]),
        Err(Error::LimitExceeded)
    );
}
