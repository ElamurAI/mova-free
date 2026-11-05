use super::{Error, Item, Schedule, select};
fn oracle(items: &[Item]) -> u128 {
    (0usize..(1 << items.len()))
        .filter_map(|mask| {
            let selected: Vec<_> = (0..items.len()).filter(|i| mask & (1 << i) != 0).collect();
            let valid = selected.iter().enumerate().all(|(p, &i)| {
                selected[p + 1..]
                    .iter()
                    .all(|&j| items[i].end <= items[j].start || items[j].end <= items[i].start)
            });
            valid.then(|| selected.iter().map(|&i| u128::from(items[i].weight)).sum())
        })
        .max()
        .unwrap()
}
fn verify(items: &[Item], schedule: &Schedule) {
    assert_eq!(schedule.total_weight, oracle(items));
    let mut seen = std::collections::BTreeSet::new();
    for &i in &schedule.indices {
        assert!(i < items.len());
        assert!(seen.insert(i));
    }
    for pair in schedule.indices.windows(2) {
        assert!(items[pair[0]].end <= items[pair[1]].start);
    }
    assert_eq!(
        schedule.total_weight,
        schedule
            .indices
            .iter()
            .map(|&i| u128::from(items[i].weight))
            .sum()
    );
}
#[test]
fn exhaustive_subset_oracle() {
    let choices = [
        Item {
            start: -1,
            end: 0,
            weight: 0,
        },
        Item {
            start: 0,
            end: 1,
            weight: 2,
        },
        Item {
            start: -1,
            end: 1,
            weight: 3,
        },
        Item {
            start: 1,
            end: 2,
            weight: 2,
        },
    ];
    for n in 0..=6 {
        for mut code in 0..4usize.pow(n) {
            let items: Vec<_> = (0..n)
                .map(|_| {
                    let item = choices[code % 4];
                    code /= 4;
                    item
                })
                .collect();
            verify(&items, &select(&items).unwrap());
        }
    }
}
#[test]
fn adjacency_extremes_ties_and_original_indices() {
    let items = [
        Item {
            start: 0,
            end: i64::MAX,
            weight: u64::MAX,
        },
        Item {
            start: i64::MIN,
            end: 0,
            weight: u64::MAX,
        },
    ];
    assert_eq!(
        select(&items).unwrap(),
        Schedule {
            indices: vec![1, 0],
            total_weight: 2 * u128::from(u64::MAX)
        }
    );
    let item = Item {
        start: 0,
        end: 2,
        weight: 4,
    };
    assert_eq!(select(&[item, item]).unwrap().indices, vec![0]);
    assert_eq!(
        select(&[Item { weight: 0, ..item }]).unwrap().indices,
        Vec::<usize>::new()
    );
    assert_eq!(
        select(&[]).unwrap(),
        Schedule {
            indices: vec![],
            total_weight: 0
        }
    );
}
#[test]
fn first_invalid_interval() {
    let items = [
        Item {
            start: 0,
            end: 1,
            weight: 1,
        },
        Item {
            start: 2,
            end: 2,
            weight: 9,
        },
        Item {
            start: 4,
            end: 3,
            weight: 8,
        },
    ];
    assert_eq!(select(&items), Err(Error::InvalidInterval { index: 1 }));
    assert_eq!(
        select(&items[2..]),
        Err(Error::InvalidInterval { index: 0 })
    );
}
#[test]
fn translation_scaling_permutation_and_ambiguity() {
    let items = [
        Item {
            start: -3,
            end: -1,
            weight: 3,
        },
        Item {
            start: -1,
            end: 2,
            weight: 4,
        },
        Item {
            start: -2,
            end: 1,
            weight: 6,
        },
    ];
    let best = select(&items).unwrap();
    let shifted: Vec<_> = items
        .iter()
        .map(|x| Item {
            start: x.start + 12,
            end: x.end + 12,
            ..*x
        })
        .collect();
    assert_eq!(select(&shifted).unwrap(), best);
    let scaled: Vec<_> = items
        .iter()
        .map(|x| Item {
            weight: x.weight * 3,
            ..*x
        })
        .collect();
    let result = select(&scaled).unwrap();
    assert_eq!(result.indices, best.indices);
    assert_eq!(result.total_weight, best.total_weight * 3);
    let mut reversed = items;
    reversed.reverse();
    assert_eq!(select(&reversed).unwrap().total_weight, best.total_weight);
    let alternative = [Item {
        start: 99,
        end: 100,
        weight: 7,
    }];
    assert_eq!(
        select(&alternative).unwrap().total_weight,
        best.total_weight
    );
    assert_ne!(select(&alternative).unwrap().indices, best.indices);
}

#[test]
fn equal_score_alternative_schedule_skips_later_item() {
    let items = [
        Item {
            start: 0,
            end: 1,
            weight: 1,
        },
        Item {
            start: 1,
            end: 2,
            weight: 1,
        },
        Item {
            start: 0,
            end: 2,
            weight: 2,
        },
    ];
    // Sorted order is 0,2,1; the final take ties the encompassing item.
    assert_eq!(
        select(&items).unwrap(),
        Schedule {
            indices: vec![2],
            total_weight: 2
        }
    );
}
