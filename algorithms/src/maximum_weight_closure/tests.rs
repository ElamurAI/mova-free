use super::*;
#[test]
fn exhaustive_closures() {
    let n = 4;
    let pairs: Vec<_> = (0..n)
        .flat_map(|u| (0..n).filter(move |&v| u != v).map(move |v| (u, v)))
        .collect();
    for bits in 0..1usize << pairs.len() {
        let edges: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter_map(|(i, &e)| (bits & (1 << i) != 0).then_some(e))
            .collect();
        for weights in [vec![-2, 3, -1, 2], vec![0, 0, 0, 0], vec![2, -3, 1, -2]] {
            let got = maximum_weight_closure(&weights, &edges).unwrap();
            let mut best = 0i128;
            for mask in 0..1usize << n {
                if edges
                    .iter()
                    .all(|&(u, v)| mask & (1 << u) == 0 || mask & (1 << v) != 0)
                {
                    best = best.max(
                        (0..n)
                            .filter(|&v| mask & (1 << v) != 0)
                            .map(|v| i128::from(weights[v]))
                            .sum(),
                    );
                }
            }
            assert_eq!(got.weight, best);
            assert_eq!(
                got.weight,
                got.selected.iter().map(|&v| i128::from(weights[v])).sum()
            );
            assert!(
                edges
                    .iter()
                    .all(|&(u, v)| !got.selected.contains(&u) || got.selected.contains(&v))
            );
            assert!(got.selected.windows(2).all(|p| p[0] < p[1]));
        }
    }
}
#[test]
fn extremes_and_errors() {
    assert_eq!(
        maximum_weight_closure(&[8, -3], &[(0, 1)]).unwrap().weight,
        5
    );
    assert_eq!(
        maximum_weight_closure(&[i64::MAX, i64::MIN], &[(0, 1)])
            .unwrap()
            .weight,
        0
    );
    assert_eq!(
        maximum_weight_closure(&[i64::MAX, i64::MAX, 1], &[]),
        Err(Error::Overflow)
    );
    assert_eq!(maximum_weight_closure(&[], &[]).unwrap().selected, vec![]);
    assert_eq!(
        maximum_weight_closure(&[], &[(0, 0)]),
        Err(Error::InvalidImplication { index: 0 })
    );
    assert_eq!(
        maximum_weight_closure(&vec![0; MAX_VERTICES + 1], &[]),
        Err(Error::LimitExceeded)
    );
}
