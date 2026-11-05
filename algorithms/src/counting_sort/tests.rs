use super::{CountingSortError, counting_sort};

#[test]
fn exhaustive_small_arrays() {
    for len in 0..=7 {
        for mut code in 0..4usize.pow(len) {
            let mut actual = vec![0; len as usize];
            for value in &mut actual {
                *value = code % 4;
                code /= 4;
            }
            let mut expected = actual.clone();
            expected.sort();
            counting_sort(&mut actual, 4).unwrap();
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn boundaries_and_errors_are_atomic() {
    counting_sort(&mut [], 0).unwrap();
    let mut high = [usize::MAX, usize::MAX - 2, usize::MAX];
    counting_sort(&mut high, 3).unwrap();
    assert_eq!(high, [usize::MAX - 2, usize::MAX, usize::MAX]);
    let fixtures = [
        (
            vec![0, usize::MAX],
            usize::MAX,
            CountingSortError::RangeOverflow,
        ),
        (
            vec![9, 7, 8],
            2,
            CountingSortError::BucketLimitExceeded {
                required: 3,
                limit: 2,
            },
        ),
        (
            vec![42],
            0,
            CountingSortError::BucketLimitExceeded {
                required: 1,
                limit: 0,
            },
        ),
        // Guaranteed capacity overflow in Vec<usize>, without allocating huge memory.
        (
            vec![usize::MAX - 1, 0],
            usize::MAX,
            CountingSortError::AllocationFailed,
        ),
    ];
    for (mut values, cap, error) in fixtures {
        let original = values.clone();
        assert_eq!(counting_sort(&mut values, cap), Err(error));
        assert_eq!(values, original);
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn deterministic_large_oracle_and_idempotence() {
    let mut seed = 17usize;
    let mut values: Vec<_> = (0..8192)
        .map(|_| {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed % 1024
        })
        .collect();
    let mut expected = values.clone();
    expected.sort();
    counting_sort(&mut values, 1024).unwrap();
    assert_eq!(values, expected);
    counting_sort(&mut values, 1024).unwrap();
    assert_eq!(values, expected);
}

#[test]
fn histogram_fibers_retain_multiplicity_not_order() {
    let target = [2, 2, 4, 5];
    let histogram = |xs: &[usize]| {
        let mut counts = std::collections::BTreeMap::new();
        for &x in xs {
            *counts.entry(x).or_insert(0usize) += 1;
        }
        counts
    };
    let mut fiber_size = 0;
    for mut code in 0..4usize.pow(4) {
        let mut input = [0; 4];
        for x in &mut input {
            *x = code % 4 + 2;
            code /= 4;
        }
        let compatible = histogram(&input) == histogram(&target);
        let mut actual = input;
        counting_sort(&mut actual, 4).unwrap();
        assert_eq!(actual == target, compatible);
        fiber_size += usize::from(compatible);
    }
    assert_eq!(fiber_size, 12); // 4! / 2!, not a unique inverse.
    let wrong = [2, 4, 4, 5];
    assert_eq!(
        wrong.iter().collect::<std::collections::BTreeSet<_>>(),
        target.iter().collect::<std::collections::BTreeSet<_>>()
    );
    assert_ne!(histogram(&wrong), histogram(&target)); // Support alone is insufficient.
}

#[test]
fn monotone_relabel_requires_transformed_span_budget() {
    for input in [[5, 2, 4, 2], [0, 3, 1, 3], [7, 7, 7, 7]] {
        let mut sorted = input;
        counting_sort(&mut sorted, 8).unwrap();
        let map = |x: usize| x.checked_mul(3).unwrap().checked_add(11).unwrap();
        let mut mapped = input.map(map);
        let span = map(*input.iter().max().unwrap()) - map(*input.iter().min().unwrap()) + 1;
        counting_sort(&mut mapped, span).unwrap();
        assert_eq!(mapped, sorted.map(map));
    }
    let mut expanded = [26, 17, 23, 17];
    let original = expanded;
    assert_eq!(
        counting_sort(&mut expanded, 4),
        Err(CountingSortError::BucketLimitExceeded {
            required: 10,
            limit: 4
        })
    );
    assert_eq!(expanded, original); // Monotonicity does not preserve resource feasibility.
    let mut wrapped = [usize::MAX, 0];
    counting_sort(&mut wrapped, 2).unwrap_err();
    assert!(usize::MAX.wrapping_add(1) < 1usize.wrapping_add(1)); // Wrapping is not monotone.
}

#[test]
fn budget_fiber_and_error_precedence_are_atomic() {
    let input = [9, 7, 9, 8];
    for budget in 0..=6 {
        let mut actual = input;
        let result = counting_sort(&mut actual, budget);
        if budget < 3 {
            assert_eq!(
                result,
                Err(CountingSortError::BucketLimitExceeded {
                    required: 3,
                    limit: budget
                })
            );
            assert_eq!(actual, input);
        } else {
            assert_eq!(result, Ok(()));
            assert_eq!(actual, [7, 8, 9, 9]);
        }
    }
    let mut overflow = [usize::MAX, 0];
    assert_eq!(
        counting_sort(&mut overflow, 0),
        Err(CountingSortError::RangeOverflow)
    );
    assert_eq!(overflow, [usize::MAX, 0]); // Overflow precedes even a zero budget.
    let mut repeated = [42; 32];
    assert_eq!(counting_sort(&mut repeated, 1), Ok(())); // Budget is span, not n.
}
