use super::*;
fn sums(a: &[u64], t: u64) -> Vec<u64> {
    (0..1usize << a.len())
        .map(|m| {
            a.iter()
                .enumerate()
                .filter(|(i, _)| m >> i & 1 == 1)
                .map(|(_, x)| x)
                .sum()
        })
        .filter(|&s| s <= t)
        .collect()
}
#[test]
fn exhaustive_oracle() {
    for seed in 0..60 {
        let a: Vec<u64> = (0..8).map(|i| (seed * 23 + i * 17) % 93).collect();
        let feasible = sums(&a, 201);
        let opt = *feasible.iter().max().unwrap();
        for p in [1, 2, 5, 10] {
            let c = max_sum(&a, 201, p, 100000).unwrap();
            assert!(feasible.contains(&c.sum), "feasible witness");
            assert!(
                u128::from(c.sum) * 100 >= u128::from(opt) * u128::from(100 - p),
                "relative guarantee"
            );
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(max_sum(&[], 0, 1, 0).unwrap().sum, 0);
    assert_eq!(max_sum(&[0, 0, 9], 0, 1, 3).unwrap().sum, 0);
    assert_eq!(
        max_sum(&[u64::MAX, u64::MAX], u64::MAX, 1, 100)
            .unwrap()
            .sum,
        u64::MAX
    );
    assert_eq!(max_sum(&[4], 8, 10, 100).unwrap().sum, 4, "single use");
    for p in [0, 3, 11, 15, 25] {
        assert_eq!(max_sum(&[], 0, p, 0), Err(Error::UnsupportedPercent));
    }
    let c = max_sum(&[1, 2, 4], 7, 1, 100).unwrap();
    assert_eq!(max_sum(&[1, 2, 4], 7, 1, c.generated_states).unwrap(), c);
    assert_eq!(
        max_sum(&[1, 2, 4], 7, 1, c.generated_states - 1),
        Err(Error::WorkBudget)
    );
}
#[test]
fn trim_is_nonexact() {
    let a = [100, 101];
    let c = max_sum(&a, 101, 10, 100).unwrap();
    assert_eq!(c.sum, 100, "trim keeps smaller representative");
}
