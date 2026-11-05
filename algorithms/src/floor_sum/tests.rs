use super::*;
#[test]
fn exhaustive_direct_oracle() {
    for n in 0..24 {
        for m in 1..24 {
            for a in 0..24 {
                for b in 0..24 {
                    let expected = (0..n)
                        .map(|i| (u128::from(a) * u128::from(i) + u128::from(b)) / u128::from(m))
                        .sum();
                    assert_eq!(floor_sum(n, m, a, b), Ok(expected), "{n} {m} {a} {b}");
                }
            }
        }
    }
}
#[test]
fn boundaries_and_overflow() {
    let u = u64::MAX;
    assert_eq!(floor_sum(0, 0, 0, 0), Err(FloorSumError::ZeroModulus));
    assert_eq!(floor_sum(0, 1, u, u), Ok(0));
    assert_eq!(
        floor_sum(u, u, u, u),
        Ok(u128::from(u) * (u128::from(u) + 1) / 2)
    );
    assert_eq!(floor_sum(u, 1, 0, u), Ok(u128::from(u) * u128::from(u)));
    assert_eq!(floor_sum(u, 1, u, 0), Err(FloorSumError::Overflow));
    assert_eq!(floor_sum(2, 1, u, u), Ok(3 * u128::from(u)));
    assert_eq!(
        floor_sum(u, u, u - 1, u - 1),
        Ok(u128::from(u) * (u128::from(u) - 1) / 2)
    );
}
#[test]
fn prefix_and_quotient_shifts() {
    for n in 0..50 {
        for m in 1..20 {
            let s = floor_sum(n, m, 7, 11).unwrap();
            assert_eq!(
                floor_sum(n + 1, m, 7, 11).unwrap() - s,
                u128::from((7 * n + 11) / m)
            );
            assert_eq!(
                floor_sum(n, m, 7 + 3 * m, 11 + 2 * m).unwrap(),
                s + 3 * u128::from(n) * u128::from(n.saturating_sub(1)) / 2 + 2 * u128::from(n)
            );
        }
    }
}
#[test]
fn reconstruction_fibers() {
    assert_eq!(floor_sum(3, 2, 1, 0), Ok(1));
    assert_eq!(floor_sum(1, 7, 99, 7), Ok(1));
    for n in [0, 1, 5, u64::MAX] {
        assert_eq!(floor_sum(n, 9, 0, 0), Ok(0));
    }
}
#[test]
fn large_independent_direct_cases() {
    let mut seed = 19u64;
    for _ in 0..1000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a = seed;
        let b = seed.rotate_left(29);
        let m = seed.rotate_left(41) | 1;
        let n = seed % 50;
        let expected = (0..n)
            .map(|i| (u128::from(a) * u128::from(i) + u128::from(b)) / u128::from(m))
            .sum();
        assert_eq!(floor_sum(n, m, a, b), Ok(expected));
    }
}
