use super::*;

fn enumerate(items: &[Item], capacity: usize) -> u128 {
    if items.is_empty() {
        return 0;
    }
    let item = items[0];
    (0..=item.count)
        .filter(|&n| n * item.weight <= capacity)
        .map(|n| n as u128 * item.value + enumerate(&items[1..], capacity - n * item.weight))
        .max()
        .unwrap()
}

#[test]
fn exhaustive_multiplicities() {
    for code in 0..4096usize {
        let mut x = code;
        let mut items = Vec::new();
        for _ in 0..3 {
            items.push(Item {
                weight: x % 4,
                value: ((x / 4) % 2) as u128,
                count: (x / 8) % 2,
            });
            x /= 16;
        }
        for capacity in 0..=7 {
            assert_eq!(
                bounded_knapsack(&items, capacity),
                Ok(enumerate(&items, capacity))
            );
        }
    }
}

#[test]
fn bundles_and_duplicate_items() {
    for count in 0..=19 {
        let items = [
            Item {
                weight: 2,
                value: 7,
                count,
            },
            Item {
                weight: 2,
                value: 7,
                count: 3,
            },
        ];
        for capacity in 0..=40 {
            assert_eq!(
                bounded_knapsack(&items, capacity),
                Ok(enumerate(&items, capacity))
            );
        }
    }
    assert_eq!(bounded_knapsack(&[], 0), Ok(0));
}

#[test]
fn genuine_and_infeasible_overflow() {
    let big = Item {
        weight: 1,
        value: u128::MAX,
        count: usize::MAX,
    };
    assert_eq!(bounded_knapsack(&[big], 1), Ok(u128::MAX));
    assert_eq!(bounded_knapsack(&[big], 2), Err(Error::ValueOverflow));
    assert_eq!(
        bounded_knapsack(
            &[Item {
                weight: usize::MAX,
                ..big
            }],
            10
        ),
        Ok(0)
    );
    assert_eq!(
        bounded_knapsack(
            &[Item {
                weight: 0,
                count: 0,
                ..big
            }],
            0
        ),
        Ok(0)
    );
    assert_eq!(
        bounded_knapsack(
            &[Item {
                weight: 0,
                count: 2,
                ..big
            }],
            0
        ),
        Err(Error::ValueOverflow)
    );
    assert_eq!(
        bounded_knapsack(
            &[
                big,
                Item {
                    weight: 0,
                    value: 1,
                    count: 1
                }
            ],
            1
        ),
        Err(Error::ValueOverflow)
    );
    assert_eq!(bounded_knapsack(&[big, big], 1), Ok(u128::MAX));
}

#[test]
fn limits_and_monotonicity() {
    assert_eq!(
        bounded_knapsack(&[], usize::MAX),
        Err(Error::CapacityTooLarge)
    );
    let item = Item {
        weight: 1,
        value: 1,
        count: usize::MAX,
    };
    assert_eq!(
        bounded_knapsack(&vec![item; MAX_ITEMS + 1], 0),
        Err(Error::TooManyItems)
    );
    assert_eq!(
        bounded_knapsack(&[item; 2], MAX_CAPACITY),
        Err(Error::WorkLimit)
    );
    assert_eq!(bounded_knapsack(&[], MAX_CAPACITY), Ok(0));
    let mut previous = 0;
    for capacity in 0..100 {
        let value = bounded_knapsack(&[item], capacity).unwrap();
        assert!(value >= previous);
        assert_eq!(value, capacity as u128);
        previous = value;
    }
}
