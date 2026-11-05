use super::*;

fn subsets(items: &[Item], capacity: usize) -> u128 {
    (0..1usize << items.len())
        .filter_map(|mask| {
            let mut weight = 0;
            let mut value = 0;
            for (i, item) in items.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    weight += item.weight;
                    value += u128::from(item.value);
                }
            }
            (weight <= capacity).then_some(value)
        })
        .max()
        .unwrap()
}

#[test]
fn exhaustive_subsets() {
    for n in 0..=4 {
        for mut code in 0..9usize.pow(n) {
            let items: Vec<_> = (0..n)
                .map(|_| {
                    let digit = code % 9;
                    code /= 9;
                    Item {
                        weight: digit / 3,
                        value: (digit % 3) as u64,
                    }
                })
                .collect();
            for capacity in 0..=5 {
                assert_eq!(
                    max_value(&items, capacity),
                    Ok(subsets(&items, capacity)),
                    "{items:?}, capacity={capacity}"
                );
            }
        }
    }
}

#[test]
fn zero_weight_once_and_wide_values() {
    let items = [Item {
        weight: 0,
        value: u64::MAX,
    }; 2];
    assert_eq!(max_value(&items, 0), Ok(2 * u128::from(u64::MAX)));
    assert_eq!(
        max_value(
            &[Item {
                weight: 1,
                value: 7
            }],
            4
        ),
        Ok(7)
    );
    assert_eq!(max_value(&[], 0), Ok(0));
}

#[test]
fn dimension_overflow_before_allocation() {
    assert_eq!(max_value(&[], usize::MAX), Err(Error::CapacityOverflow));
    assert_eq!(
        max_value(&[], usize::MAX / 16),
        Err(Error::CapacityOverflow)
    );
    assert_eq!(
        max_value(&[], isize::MAX as usize / 16),
        Err(Error::CapacityOverflow)
    );
}

#[test]
fn larger_subsets_with_wide_values_and_zero_weights() {
    let mut seed = 0x94d049bb133111ebu64;
    for case in 0..64 {
        let items: Vec<_> = (0..12)
            .map(|i| {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                Item {
                    weight: if i % 4 == 0 {
                        0
                    } else {
                        (seed >> 32) as usize % 6
                    },
                    value: if i % 3 == 0 {
                        u64::MAX - (seed & 255)
                    } else {
                        seed & 1023
                    },
                }
            })
            .collect();
        for capacity in [0, 1, 7, 15] {
            assert_eq!(
                max_value(&items, capacity),
                Ok(subsets(&items, capacity)),
                "case={case}, capacity={capacity}, items={items:?}"
            );
        }
    }
    let items = [
        Item {
            weight: usize::MAX,
            value: u64::MAX,
        },
        Item {
            weight: 0,
            value: 9,
        },
        Item {
            weight: 1,
            value: 4,
        },
    ];
    assert_eq!(max_value(&items, 1), Ok(13));
}

#[test]
fn permutation_and_capacity_order_match_subset_oracle() {
    for case in 0..32usize {
        let items: Vec<_> = (0..8)
            .map(|i| Item {
                weight: (case + i * 3) % 6,
                value: ((case * 17 + i * 11) % 41) as u64,
            })
            .collect();
        let mut permuted = items.clone();
        permuted.reverse();
        permuted.rotate_left(case % 8);
        let mut previous = 0;
        for capacity in 0..=16 {
            let value = max_value(&items, capacity).unwrap();
            assert_eq!(
                value,
                subsets(&items, capacity),
                "case={case}, capacity={capacity}, items={items:?}"
            );
            assert_eq!(
                max_value(&permuted, capacity),
                Ok(value),
                "permutation case={case}, capacity={capacity}"
            );
            assert!(
                previous <= value,
                "capacity order case={case}, capacity={capacity}"
            );
            previous = value;
        }
    }
}
