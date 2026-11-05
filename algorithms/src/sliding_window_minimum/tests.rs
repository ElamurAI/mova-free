use super::*;
fn oracle(values: &[i32], window: usize) -> Vec<usize> {
    if window == 0 || window > values.len() {
        return vec![];
    }
    (0..=values.len() - window)
        .map(|start| {
            (start..start + window)
                .min_by_key(|&i| (values[i], i))
                .unwrap()
        })
        .collect()
}
#[test]
fn exhaustive_windows_against_independent_scan() {
    for n in 0..=8u32 {
        for mut code in 0..3usize.pow(n) {
            let values: Vec<i32> = (0..n)
                .map(|_| {
                    let value = (code % 3) as i32 - 1;
                    code /= 3;
                    value
                })
                .collect();
            let before = values.clone();
            for window in 0..=values.len() + 1 {
                assert_eq!(
                    minimum_indices(&values, window),
                    oracle(&values, window),
                    "values={values:?}, width={window}"
                );
            }
            assert!(minimum_indices(&values, usize::MAX).is_empty());
            assert_eq!(values, before);
        }
    }
}
#[test]
fn expiration_and_equal_values_preserve_leftmost_ties() {
    assert_eq!(minimum_indices(&[2, 2, 2, 2], 2), vec![0, 1, 2]);
    assert_eq!(minimum_indices(&[0, 9, 8, 7, 6], 2), vec![0, 2, 3, 4]);
    assert_eq!(minimum_indices(&[1, 2, 3, 4, 5], 3), vec![0, 1, 2]);
    assert_eq!(minimum_indices(&[5, 4, 3, 2, 1], 3), vec![2, 3, 4]);
    assert_eq!(
        minimum_indices(&[i32::MAX, i32::MIN, i32::MIN], 2),
        vec![1, 1]
    );
}
#[test]
fn shift_and_width_boundaries() {
    for n in 0..=32 {
        let values: Vec<i32> = (0..n).map(|i| (i * 13 % 17) - 8).collect();
        let shifted: Vec<_> = values.iter().map(|x| x + 100).collect();
        assert_eq!(
            minimum_indices(&values, 1),
            (0..values.len()).collect::<Vec<_>>()
        );
        for window in 1..=values.len() {
            assert_eq!(
                minimum_indices(&values, window),
                minimum_indices(&shifted, window)
            );
        }
    }
}
#[test]
fn negative_controls_reject_stale_and_rightmost_candidates() {
    assert_ne!(minimum_indices(&[0, 3, 2], 2), vec![0, 0]);
    assert_ne!(minimum_indices(&[1, 1, 1], 2), vec![1, 2]);
}

#[test]
fn witness_shift_and_contained_window_retention() {
    for n in 1..=6u32 {
        for mut code in 0..3usize.pow(n) {
            let values: Vec<i32> = (0..n)
                .map(|_| {
                    let value = (code % 3) as i32 - 1;
                    code /= 3;
                    value
                })
                .collect();
            for width in 1..=values.len() {
                let full = minimum_indices(&values, width);
                for offset in 0..=values.len() - width {
                    let local = minimum_indices(&values[offset..], width);
                    assert_eq!(
                        local.iter().map(|&i| i + offset).collect::<Vec<_>>(),
                        full[offset..]
                    );
                }
                for (s, &winner) in full.iter().enumerate() {
                    for middle in s..=s + width {
                        let candidates =
                            [(s, middle), (middle, s + width)]
                                .into_iter()
                                .filter_map(|(a, b)| {
                                    minimum_indices(&values[a..b], b - a)
                                        .first()
                                        .map(|&i| i + a)
                                });
                        assert_eq!(candidates.min_by_key(|&i| (values[i], i)), Some(winner));
                    }
                    for a in s..=winner {
                        for b in winner + 1..=s + width {
                            assert_eq!(minimum_indices(&values[a..b], b - a), vec![winner - a]);
                        }
                    }
                }
            }
        }
    }
    // Containment without retention of the original witness changes the minimum.
    assert_eq!(minimum_indices(&[0, 2, 1], 3), vec![0]);
    assert_eq!(minimum_indices(&[2, 1], 2), vec![1]);
}

#[test]
fn simultaneous_output_fibers_and_width_one_information_loss() {
    for mut code in 0..81usize {
        let values: Vec<i32> = (0..4)
            .map(|_| {
                let value = (code % 3) as i32 - 1;
                code /= 3;
                value
            })
            .collect();
        let actual = minimum_indices(&values, 2);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    let candidate = [a, b, c];
                    let compatible = candidate.iter().enumerate().all(|(s, &j)| {
                        (s..s + 2).contains(&j)
                            && (s..s + 2).all(|k| {
                                values[j] <= values[k] && (k >= j || values[j] < values[k])
                            })
                    });
                    assert_eq!(compatible, actual == candidate);
                }
            }
        }
    }
    assert_eq!(
        minimum_indices(&[5, -8, 3], 1),
        minimum_indices(&[-7, 4, 99], 1)
    );
    assert_eq!(minimum_indices(&[1, 1], 2), vec![0]);
    assert_ne!(minimum_indices(&[1, 1], 2), vec![1]);
}

#[test]
fn order_embedding_and_invalid_width_preserve_observations() {
    for values in [vec![], vec![1, 0], vec![2, 2, -1, 3, -1, 0]] {
        let before = values.clone();
        let mapped: Vec<_> = values.iter().map(|&x| 5 * x + 2).collect();
        for width in 0..=values.len() + 1 {
            assert_eq!(
                minimum_indices(&values, width),
                minimum_indices(&mapped, width)
            );
        }
        assert!(minimum_indices(&values, usize::MAX).is_empty());
        assert_eq!(values, before);
    }
    // A merely nondecreasing map can merge unequal values and move the leftmost tie.
    assert_ne!(minimum_indices(&[1, 0], 2), minimum_indices(&[0, 0], 2));
}
