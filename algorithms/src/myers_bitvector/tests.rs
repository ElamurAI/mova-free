use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}
fn oracle(a: &[u8], b: &[u8]) -> usize {
    let mut d = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in d[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]));
        }
    }
    d[a.len()][b.len()]
}
#[test]
fn exhaustive_dp() {
    let w = words(5);
    for a in &w {
        for b in &w {
            assert_eq!(distance(a, b), Ok(oracle(a, b)));
        }
    }
}
#[test]
fn boundaries() {
    for n in [1, 63, 64] {
        let a = vec![255; n];
        assert_eq!(distance(&a, &a), Ok(0));
        assert_eq!(distance(&a, &vec![0; n]), Ok(n));
    }
    assert_eq!(distance(&[], b"abc"), Ok(3));
    assert_eq!(distance(&[0; 65], &[]), Err(Error::PatternTooLong));
    assert_eq!(distance(&[], &vec![0; 65537]), Err(Error::TextTooLong));
}
