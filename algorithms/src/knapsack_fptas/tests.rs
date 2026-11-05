use super::*;
fn brute(a: &[Item], cap: usize) -> u128 {
    (0..1usize << a.len())
        .filter_map(|mask| {
            let mut w = 0;
            let mut v = 0;
            for (i, x) in a.iter().enumerate() {
                if mask >> i & 1 == 1 {
                    w += x.weight;
                    v += u128::from(x.value);
                }
            }
            (w <= cap).then_some(v)
        })
        .max()
        .unwrap()
}
#[test]
fn oracle_and_certificate() {
    for seed in 0..50 {
        let a: Vec<_> = (0..7)
            .map(|i| Item {
                weight: (seed * 7 + i * 3) % 12,
                value: ((seed * 13 + i * 17) % 101) as u64,
            })
            .collect();
        for p in [1, 2, 5, 10] {
            let opt = brute(&a, 18);
            let c = max_value(&a, 18, p, 2_000_000).unwrap();
            assert!(c.value <= opt, "feasible objective");
            assert!(
                (opt - c.value) * 100 <= c.loss_numerator,
                "additive certificate"
            );
            assert!(
                c.value * 100 >= opt * u128::from(100 - p),
                "relative guarantee"
            );
        }
    }
}
#[test]
fn zero_weight_and_single_use() {
    let a = [
        Item {
            weight: 0,
            value: 7,
        },
        Item {
            weight: 1,
            value: 11,
        },
    ];
    assert_eq!(max_value(&a, 1, 10, 10000).unwrap().value, 18, "single use");
    assert_eq!(max_value(&a, 0, 1, 10000).unwrap().value, 7);
    assert_eq!(max_value(&[], 0, 1, 0).unwrap().value, 0);
    assert_eq!(
        max_value(
            &[Item {
                weight: usize::MAX,
                value: u64::MAX
            }],
            usize::MAX,
            1,
            10000
        )
        .unwrap()
        .value,
        u128::from(u64::MAX)
    );
}
#[test]
fn validation_and_budget() {
    for p in [0, 3, 15, 25, 100] {
        assert_eq!(max_value(&[], 0, p, 0), Err(Error::UnsupportedPercent));
    }
    let a = [Item {
        weight: 1,
        value: 10,
    }];
    let c = max_value(&a, 1, 10, 1000).unwrap();
    assert_eq!(max_value(&a, 1, 10, c.work).unwrap(), c);
    assert_eq!(max_value(&a, 1, 10, c.work - 1), Err(Error::WorkBudget));
    assert_eq!(max_value(&a, 0, 10, 0), Err(Error::WorkBudget));
}
#[test]
fn nonexact_scaling() {
    let mut found = false;
    for x in 50..150 {
        let a = [
            Item {
                weight: 1,
                value: 10000,
            },
            Item {
                weight: 1,
                value: x,
            },
            Item {
                weight: 1,
                value: x + 1,
            },
        ];
        let c = max_value(&a, 2, 10, 10000).unwrap();
        let opt = brute(&a, 2);
        found |= c.value < opt;
        assert!(c.value * 100 >= opt * 90);
    }
    assert!(found, "fixture exercises approximation");
}
