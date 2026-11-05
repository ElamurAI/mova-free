use super::*;
fn words() -> Vec<Vec<u8>> {
    (0..=3)
        .flat_map(|n| (0..1usize << n).map(move |v| (0..n).map(|i| ((v >> i) & 1) as u8).collect()))
        .collect()
}
// Enumerates complete monotone paths without a DP table.
fn oracle(a: &[u8], b: &[u8], s: Scoring) -> i64 {
    if a.is_empty() && b.is_empty() {
        return 0;
    }
    let mut values = Vec::new();
    if !a.is_empty() {
        values.push(s.gap_score + oracle(&a[1..], b, s));
    }
    if !b.is_empty() {
        values.push(s.gap_score + oracle(a, &b[1..], s));
    }
    if !a.is_empty() && !b.is_empty() {
        values.push(
            if a[0] == b[0] {
                s.match_score
            } else {
                s.mismatch_score
            } + oracle(&a[1..], &b[1..], s),
        );
    }
    *values.iter().max().unwrap()
}
#[test]
fn exhaustive_optima_and_reconstruction() {
    for match_score in -1..=2 {
        for mismatch_score in -2..=1 {
            for gap_score in -2..=1 {
                let s = Scoring {
                    match_score,
                    mismatch_score,
                    gap_score,
                };
                for a in words() {
                    for b in words() {
                        let result = align(&a, &b, s, 16).unwrap();
                        assert_eq!(result.score, oracle(&a, &b, s));
                        assert_eq!(
                            result
                                .columns
                                .iter()
                                .filter_map(|c| c.0)
                                .collect::<Vec<_>>(),
                            a
                        );
                        assert_eq!(
                            result
                                .columns
                                .iter()
                                .filter_map(|c| c.1)
                                .collect::<Vec<_>>(),
                            b
                        );
                        let score: i64 = result
                            .columns
                            .iter()
                            .map(|c| match c {
                                (Some(x), Some(y)) => {
                                    if x == y {
                                        match_score
                                    } else {
                                        mismatch_score
                                    }
                                }
                                (Some(_), None) | (None, Some(_)) => gap_score,
                                _ => panic!("gap-gap column"),
                            })
                            .sum();
                        assert_eq!(score, result.score);
                    }
                }
            }
        }
    }
}
#[test]
fn deterministic_ties() {
    let s = Scoring {
        match_score: 0,
        mismatch_score: 0,
        gap_score: 0,
    };
    assert_eq!(
        align(b"ab", b"x", s, 6).unwrap().columns,
        vec![(Some(b'a'), None), (Some(b'b'), Some(b'x'))]
    );
    assert_eq!(
        align(b"a", b"xy", s, 6).unwrap().columns,
        vec![(None, Some(b'x')), (Some(b'a'), Some(b'y'))]
    );
    let s = Scoring {
        match_score: -1,
        mismatch_score: -1,
        gap_score: 0,
    };
    assert_eq!(
        align(b"a", b"x", s, 4).unwrap().columns,
        vec![(None, Some(b'x')), (Some(b'a'), None)]
    );
}
#[test]
fn budgets_and_checked_scores() {
    let s = Scoring {
        match_score: 2,
        mismatch_score: -1,
        gap_score: -1,
    };
    assert_eq!(align(b"", b"", s, 0), Err(AlignmentError::CellLimit));
    assert_eq!(
        align(b"", b"", s, 1).unwrap(),
        Alignment {
            score: 0,
            columns: vec![]
        }
    );
    assert_eq!(align(b"a", b"a", s, 3), Err(AlignmentError::CellLimit));
    assert_eq!(align(b"a", b"a", s, 4).unwrap().score, 2);
    for gap_score in [i64::MIN, i64::MAX] {
        assert_eq!(
            align(b"aa", b"", Scoring { gap_score, ..s }, 3),
            Err(AlignmentError::ScoreOverflow)
        );
    }
    assert_eq!(
        align(
            b"a",
            b"a",
            Scoring {
                gap_score: i64::MAX,
                ..s
            },
            4
        ),
        Err(AlignmentError::ScoreOverflow)
    );
    assert_eq!(
        align(
            b"a",
            b"a",
            Scoring {
                match_score: i64::MAX,
                gap_score: 0,
                ..s
            },
            4
        )
        .unwrap()
        .score,
        i64::MAX
    );
}
