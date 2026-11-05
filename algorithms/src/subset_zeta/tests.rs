use super::*;
fn oracle(v: &[u64], m: u64) -> Vec<u64> {
    (0..v.len())
        .map(|s| {
            let sum: u128 = v
                .iter()
                .enumerate()
                .filter(|(t, _)| t & !s == 0)
                .map(|(_, x)| u128::from(*x))
                .sum();
            (sum % u128::from(m)) as u64
        })
        .collect()
}
#[test]
fn exhaustive_literal_subsets() {
    for n in [1usize, 2, 4, 8] {
        for mut code in 0..3usize.pow(n as u32) {
            let v: Vec<u64> = (0..n)
                .map(|_| {
                    let x = (code % 3) as u64;
                    code /= 3;
                    x
                })
                .collect();
            for m in [1, 2, 4, 7, u64::MAX] {
                let mut actual = v.clone();
                transform(&mut actual, m).unwrap();
                assert_eq!(actual, oracle(&v, m));
            }
        }
    }
}
#[test]
fn wide_residues_and_literal_direction() {
    let v = [u64::MAX, u64::MAX - 1, 1, 8];
    for m in [1, 2, 6, u64::MAX - 1, u64::MAX] {
        let mut a = v;
        transform(&mut a, m).unwrap();
        assert_eq!(a.as_slice(), oracle(&v, m));
    }
    let mut a = [1, 2, 4, 8];
    transform(&mut a, 100).unwrap();
    assert_eq!(a, [1, 3, 5, 15]);
}
#[test]
fn validation_before_mutation_and_cap() {
    for (n, m, e) in [
        (0, 0, Error::ZeroModulus),
        (3, 0, Error::ZeroModulus),
        (0, 1, Error::Empty),
        (3, 1, Error::NotPowerOfTwo),
        (MAX_LEN + 1, 1, Error::NotPowerOfTwo),
        (MAX_LEN * 2, 1, Error::TooLong),
    ] {
        let mut a = vec![u64::MAX; n];
        let old = a.clone();
        assert_eq!(transform(&mut a, m), Err(e));
        assert_eq!(a, old);
    }
    let mut a = vec![0; MAX_LEN];
    a[0] = u64::MAX;
    transform(&mut a, u64::MAX - 1).unwrap();
    assert!(a.iter().all(|&x| x == 1));
}
