use super::*;
// All parenthesization costs, no memoization or intermediate minimization.
fn costs(d: &[u64]) -> Vec<u128> {
    if d.len() <= 2 {
        return vec![0];
    }
    let mut result = Vec::new();
    for k in 1..d.len() - 1 {
        for a in costs(&d[..=k]) {
            for b in costs(&d[k..]) {
                result
                    .push(a + b + u128::from(d[0]) * u128::from(d[k]) * u128::from(d[d.len() - 1]));
            }
        }
    }
    result
}
#[test]
fn exhaustive_parenthesizations() {
    for len in 0..=7 {
        for mut code in 0..3usize.pow(len) {
            let d: Vec<_> = (0..len)
                .map(|_| {
                    let value = (code % 3 + 1) as u64;
                    code /= 3;
                    value
                })
                .collect();
            assert_eq!(
                minimum_cost(&d),
                Ok(*costs(&d).iter().min().unwrap()),
                "{d:?}"
            );
        }
    }
}
#[test]
fn boundaries_and_overflow() {
    assert_eq!(minimum_cost(&[]), Ok(0));
    assert_eq!(minimum_cost(&[9]), Ok(0));
    assert_eq!(minimum_cost(&[u64::MAX, u64::MAX]), Ok(0));
    assert_eq!(minimum_cost(&[0]), Err(Error::ZeroDimension));
    assert_eq!(minimum_cost(&[1, 0, 3]), Err(Error::ZeroDimension));
    assert_eq!(minimum_cost(&[u64::MAX; 3]), Err(Error::CostOverflow));
    let m = u64::MAX;
    let square = u128::from(m) * u128::from(m);
    // One split overflows; the other remains representable.
    assert_eq!(minimum_cost(&[m, m, 1, 2]), Ok(square + 2 * u128::from(m)));
    assert_eq!(minimum_cost(&[m, m, 1, 2]), Ok(u128::MAX));
    // The split with two individually fitting square products overflows its sum.
    assert_eq!(minimum_cost(&[m, m, 1, m]), Err(Error::CostOverflow));
    assert_eq!(minimum_cost(&[30, 35, 15, 5, 10, 20, 25]), Ok(15125));
}
#[test]
fn transformations() {
    for d in [vec![2, 8, 3, 5], vec![1, 7, 2, 9, 4], vec![5, 5, 5]] {
        let result = minimum_cost(&d).unwrap();
        let reversed: Vec<_> = d.iter().copied().rev().collect();
        let scaled: Vec<_> = d.iter().map(|x| x * 3).collect();
        assert_eq!(minimum_cost(&reversed), Ok(result));
        assert_eq!(minimum_cost(&scaled), Ok(result * 27));
    }
    for n in 1..=20 {
        assert_eq!(minimum_cost(&vec![4; n + 1]), Ok((n as u128 - 1) * 64));
    }
}
#[test]
fn finite_preimages() {
    let inputs: Vec<_> = (1..=12)
        .filter(|&x| minimum_cost(&[2, x, 3]) == Ok(24))
        .collect();
    assert_eq!(inputs, vec![4]);
}
