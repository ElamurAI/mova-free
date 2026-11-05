use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    let mut all = vec![vec![]];
    for len in 1..=max {
        for bits in 0..1usize << len {
            all.push((0..len).map(|i| ((bits >> i) & 1) as u8).collect());
        }
    }
    all
}

// Enumerate suffix edit scripts, independent of the rolling table/storage order.
fn oracle(a: &[u8], b: &[u8]) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let mut best = (1 + oracle(&a[1..], b))
        .min(1 + oracle(a, &b[1..]))
        .min(usize::from(a[0] != b[0]) + oracle(&a[1..], &b[1..]));
    if a.len() > 1 && b.len() > 1 && a[0] == b[1] && a[1] == b[0] {
        best = best.min(1 + oracle(&a[2..], &b[2..]));
    }
    best
}
#[test]
fn bounded_oracle() {
    let all = words(4);
    for a in &all {
        for b in &all {
            assert_eq!(distance(a, b), oracle(a, b));
        }
    }
}
#[test]
fn restricted_is_not_full_damerau_or_metric() {
    assert_eq!(distance(b"CA", b"AC"), 1);
    assert_eq!(distance(b"AC", b"ABC"), 1);
    assert_eq!(distance(b"CA", b"ABC"), 3);
    // Full Damerau can transpose CA -> AC then insert B: cost two.
    assert!(distance(b"CA", b"ABC") > distance(b"CA", b"AC") + distance(b"AC", b"ABC"));
    assert_eq!(distance(b"abcd", b"badc"), 2);
}
#[test]
fn boundaries_symmetry_fibers_units() {
    for a in words(3) {
        for b in words(3) {
            let d = distance(&a, &b);
            assert_eq!(d, distance(&b, &a));
            assert!(d >= a.len().abs_diff(b.len()) && d <= a.len().max(b.len()));
            assert_eq!(d == 0, a == b);
            assert!(
                words(3)
                    .iter()
                    .filter(|x| oracle(x, &b) == d)
                    .any(|x| x == &a)
            );
        }
    }
    assert_eq!(distance(b"", b"abc"), 3);
    assert_eq!(distance(&['é'], &['e']), 1);
    assert_eq!(distance("é".as_bytes(), b"e"), 2);
}
