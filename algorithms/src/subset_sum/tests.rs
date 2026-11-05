use super::*;

fn oracle(values: &[usize], target: usize) -> bool {
    (0..1usize << values.len()).any(|mask| {
        values
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &x)| x as u128)
            .sum::<u128>()
            == target as u128
    })
}

#[test]
fn exhaustive_independent_masks_and_witnesses() {
    for len in 0..=6 {
        for encoded in 0usize..4usize.pow(len) {
            let mut code = encoded;
            let values: Vec<_> = (0..len)
                .map(|_| {
                    let x = code % 4;
                    code /= 4;
                    x
                })
                .collect();
            for target in 0..=20 {
                let actual = subset_sum(&values, target).unwrap();
                assert_eq!(actual.is_some(), oracle(&values, target));
                if let Some(indices) = actual {
                    assert!(indices.windows(2).all(|w| w[0] < w[1]));
                    assert!(indices.iter().all(|&i| i < values.len()));
                    assert_eq!(indices.iter().map(|&i| values[i]).sum::<usize>(), target);
                }
            }
        }
    }
}

#[test]
fn empty_zero_and_unreachable() {
    assert_eq!(subset_sum(&[], 0), Ok(Some(vec![])));
    assert_eq!(subset_sum(&[], 1), Ok(None));
    assert_eq!(subset_sum(&[0, 0], 0), Ok(Some(vec![])));
    assert_eq!(subset_sum(&[0, 0], 1), Ok(None));
    assert_eq!(subset_sum(&[3], 6), Ok(None));
}

#[test]
fn deterministic_first_discovery_and_unique_items() {
    assert_eq!(subset_sum(&[2, 2, 4], 4), Ok(Some(vec![0, 1])));
    assert_eq!(subset_sum(&[0, 3, 0, 2, 5], 5), Ok(Some(vec![1, 3])));
    assert_eq!(subset_sum(&[1, 2, 3, 4], 6), Ok(Some(vec![0, 1, 2])));
}

#[test]
fn boundaries_and_allocation_policy() {
    assert_eq!(
        subset_sum(&[usize::MAX, MAX_TARGET], MAX_TARGET),
        Ok(Some(vec![1]))
    );
    assert_eq!(subset_sum(&[], MAX_TARGET + 1), Err(Error::TargetTooLarge));
    assert_eq!(subset_sum(&[], usize::MAX), Err(Error::TargetTooLarge));
}

#[test]
fn permutation_preserves_feasibility_and_scale() {
    for target in 0..=15 {
        let a = subset_sum(&[1, 3, 4, 7], target).unwrap();
        let b = subset_sum(&[7, 4, 3, 1], target).unwrap();
        assert_eq!(a.is_some(), b.is_some());
        assert_eq!(a, subset_sum(&[2, 6, 8, 14], target * 2).unwrap());
    }
}
