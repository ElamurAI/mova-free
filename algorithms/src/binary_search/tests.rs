use super::*;

#[test]
fn bounded_linear_oracle() {
    // Every sequence of length <= 7 over three values; sorting admits duplicates.
    for len in 0..=7u32 {
        for mut code in 0..3usize.pow(len) {
            let mut values = Vec::new();
            for _ in 0..len {
                values.push((code % 3) as i32 - 1);
                code /= 3;
            }
            values.sort();
            for target in -2..=2 {
                let expected = values
                    .iter()
                    .position(|v| *v >= target)
                    .unwrap_or(values.len());
                assert_eq!(lower_bound(&values, &target), expected);
                assert_eq!(
                    binary_search_first(&values, &target),
                    values.iter().position(|v| *v == target)
                );
            }
        }
    }
}

#[test]
fn boundaries_and_borrowed_values() {
    assert_eq!(lower_bound::<u64>(&[], &0), 0);
    let values = [0, 0, 1, u64::MAX];
    assert_eq!(lower_bound(&values, &0), 0);
    assert_eq!(lower_bound(&values, &u64::MAX), 3);
    assert_eq!(binary_search_first(&values, &2), None);
    assert_eq!(lower_bound(&["a", "c", "c"], &"b"), 1);
    assert_eq!(binary_search_first(&["a", "c", "c"], &"c"), Some(1));
}
#[test]
fn wider_duplicate_runs_linear_oracle() {
    for len in [8, 31, 64, 127, 257, 1025] {
        let values: Vec<i64> = (0..len).map(|i| (i / 17) as i64 * 11 - 300).collect();
        for target in -310..=380 {
            let expected = values.iter().position(|x| *x >= target).unwrap_or(len);
            assert_eq!(
                lower_bound(&values, &target),
                expected,
                "len={len}, target={target}"
            );
            assert_eq!(
                binary_search_first(&values, &target),
                values.iter().position(|x| *x == target),
                "len={len}, target={target}"
            );
        }
    }
}

#[test]
fn descending_order_and_translation() {
    use std::cmp::Reverse;
    let values: Vec<_> = (-70i64..=70).rev().flat_map(|x| [Reverse(x); 3]).collect();
    for target in -72..=72 {
        let target = Reverse(target);
        let expected = values
            .iter()
            .position(|x| *x >= target)
            .unwrap_or(values.len());
        assert_eq!(lower_bound(&values, &target), expected, "target={target:?}");
        assert_eq!(
            binary_search_first(&values, &target),
            values.iter().position(|x| *x == target)
        );
    }
    let original = [-100i64, -100, -1, 0, 15, 200];
    let shifted = original.map(|x| x + 1000);
    for target in -102..=202 {
        assert_eq!(
            lower_bound(&original, &target),
            lower_bound(&shifted, &(target + 1000))
        );
    }
}
