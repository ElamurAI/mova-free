use super::*;

fn oracle(input: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut out = Vec::new();
    for point in -2..2 {
        if input.iter().any(|&(s, e)| s <= point && point < e) {
            if let Some((_, end)) = out.last_mut()
                && *end == point
            {
                *end += 1;
                continue;
            }
            out.push((point, point + 1));
        }
    }
    out
}

#[test]
fn exhaustive_independent_unit_cell_oracle() {
    let domain: Vec<_> = (-2..=2)
        .flat_map(|s| (s..=2).map(move |e| (s, e)))
        .collect();
    for len in 0..=4 {
        for encoded in 0usize..domain.len().pow(len) {
            let mut code = encoded;
            let input: Vec<_> = (0..len)
                .map(|_| {
                    let x = domain[code % domain.len()];
                    code /= domain.len();
                    x
                })
                .collect();
            let actual = interval_union(&input).unwrap();
            let expected = oracle(&input);
            assert_eq!(actual.intervals, expected);
            assert_eq!(
                actual.covered_length,
                expected.iter().map(|&(s, e)| (e - s) as u128).sum()
            );
            assert_eq!(interval_union(&actual.intervals).unwrap(), actual);
        }
    }
}

#[test]
fn empty_touching_nested_and_duplicate() {
    assert_eq!(
        interval_union(&[(0, 1), (1, 2)]).unwrap().intervals,
        vec![(0, 2)]
    );
    assert_eq!(
        interval_union(&[]).unwrap(),
        Union {
            intervals: vec![],
            covered_length: 0
        }
    );
    assert_eq!(
        interval_union(&[(2, 2), (-1, -1)]).unwrap().intervals,
        vec![]
    );
    assert_eq!(
        interval_union(&[(3, 5), (0, 2), (2, 3), (1, 4), (0, 2)]).unwrap(),
        Union {
            intervals: vec![(0, 5)],
            covered_length: 5
        }
    );
    assert_eq!(
        interval_union(&[(0, 1), (2, 3)]).unwrap().intervals,
        vec![(0, 1), (2, 3)]
    );
}

#[test]
fn first_error_precedes_empty_filtering() {
    assert_eq!(
        interval_union(&[(0, 0), (3, 1), (4, 2)]),
        Err(Error::ReversedInterval { index: 1 })
    );
    assert_eq!(
        interval_union(&[(i64::MAX, i64::MIN)]),
        Err(Error::ReversedInterval { index: 0 })
    );
}

#[test]
fn full_signed_span_and_disjoint_extremes() {
    assert_eq!(
        interval_union(&[(i64::MIN, 0), (0, i64::MAX)]).unwrap(),
        Union {
            intervals: vec![(i64::MIN, i64::MAX)],
            covered_length: u128::from(u64::MAX)
        }
    );
    assert_eq!(
        interval_union(&[(i64::MIN, i64::MIN + 1), (i64::MAX - 1, i64::MAX)])
            .unwrap()
            .covered_length,
        2
    );
}

#[test]
fn permutation_translation_and_input_preservation() {
    let input = vec![(-5, -1), (3, 5), (-2, 1)];
    let before = input.clone();
    let expected = interval_union(&input).unwrap();
    assert_eq!(input, before);
    let mut reversed = input.clone();
    reversed.reverse();
    assert_eq!(interval_union(&reversed).unwrap(), expected);
    let shifted: Vec<_> = input.iter().map(|&(s, e)| (s + 10, e + 10)).collect();
    let actual = interval_union(&shifted).unwrap();
    assert_eq!(actual.covered_length, expected.covered_length);
    assert_eq!(
        actual.intervals,
        expected
            .intervals
            .iter()
            .map(|&(s, e)| (s + 10, e + 10))
            .collect::<Vec<_>>()
    );
}
