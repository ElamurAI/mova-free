use super::{BandTooWide, MAX_BAND_CELLS, distance};

fn oracle(a: &[u8], b: &[u8]) -> usize {
    let mut table = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in table.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in table[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            table[i][j] = (table[i - 1][j] + 1)
                .min(table[i][j - 1] + 1)
                .min(table[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]));
        }
    }
    table[a.len()][b.len()]
}
fn words() -> Vec<Vec<u8>> {
    (0..=5)
        .flat_map(|n| {
            (0..1_usize << n).map(move |bits| {
                (0..n)
                    .map(|i| if bits & (1 << i) == 0 { 0 } else { 255 })
                    .collect()
            })
        })
        .collect()
}

#[test]
fn exhaustive_distance_and_thresholds() {
    let words = words();
    for a in &words {
        for b in &words {
            let expected = oracle(a, b);
            assert!(expected >= a.len().abs_diff(b.len()));
            assert!(expected <= a.len().max(b.len()));
            for k in (0..=6).chain([usize::MAX]) {
                assert_eq!(
                    distance(a, b, k),
                    Ok((expected <= k).then_some(expected)),
                    "{a:?} {b:?} k={k}"
                );
            }
        }
    }
}
#[test]
fn unit_cost_boundaries_and_byte_semantics() {
    assert_eq!(distance(b"kitten", b"sitting", 3), Ok(Some(3)));
    assert_eq!(distance(b"kitten", b"sitting", 2), Ok(None));
    assert_eq!(distance(b"ab", b"ba", 2), Ok(Some(2)));
    assert_eq!(distance("é".as_bytes(), b"e", 2), Ok(Some(2)));
    assert_eq!(distance(b"", b"", 0), Ok(Some(0)));
    assert_eq!(distance(b"", b"abc", 2), Ok(None));
    assert_eq!(distance(b"abc", b"", usize::MAX), Ok(Some(3)));
}
#[test]
fn narrow_band_resource_boundary_and_pruning() {
    let supported = vec![7; MAX_BAND_CELLS - 1];
    assert_eq!(distance(&supported, &supported, usize::MAX), Ok(Some(0)));
    let wide = vec![7; MAX_BAND_CELLS];
    assert_eq!(
        distance(&wide, &wide, usize::MAX),
        Err(BandTooWide {
            cells: MAX_BAND_CELLS + 1
        })
    );
    assert_eq!(distance(&wide, &wide, 0), Ok(Some(0)));
    assert_eq!(distance(&wide, &[], usize::MAX), Ok(Some(wide.len())));
    assert_eq!(distance(&wide, b"x", 1), Ok(None));
    let mut shifted = vec![1; 200];
    shifted[0] = 2;
    assert_eq!(distance(&shifted, &vec![1; 201], 2), Ok(Some(2)));
}
#[test]
fn fibers_and_false_oracle() {
    assert_eq!(distance(b"ab", b"ac", 1), distance(b"xy", b"xz", 1));
    assert_eq!(distance(b"ab", b"cd", 0), distance(b"x", b"y", 0));
    // Hamming-only and transposition-cost-one alternatives are false oracles.
    assert_eq!(oracle(b"abc", b"bc"), 1);
    assert_ne!(oracle(b"ab", b"ba"), 1);
    assert_eq!(distance(b"abc", b"bc", 1), Ok(Some(1)));
}
