use super::*;
fn trial(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}
#[test]
fn exhaustive_intervals() {
    for low in 0..80 {
        for high in low..100 {
            assert_eq!(
                segmented_sieve(low, high).unwrap(),
                (low..=high).filter(|&n| trial(n)).collect::<Vec<_>>()
            );
        }
    }
}
#[test]
fn boundaries_and_limits() {
    assert_eq!(segmented_sieve(10, 2), Err(SieveError::ReversedInterval));
    assert_eq!(
        segmented_sieve(u64::MAX, u64::MAX - 1),
        Err(SieveError::ReversedInterval)
    );
    assert_eq!(segmented_sieve(0, u64::MAX), Err(SieveError::HighLimit));
    assert_eq!(segmented_sieve(0, MAX_WIDTH), Err(SieveError::WidthLimit));
    assert_eq!(segmented_sieve(0, 1).unwrap(), vec![]);
    assert_eq!(segmented_sieve(2, 2).unwrap(), vec![2]);
    assert_eq!(segmented_sieve(0, MAX_WIDTH - 1).unwrap().len(), 78498);
}
#[test]
fn squares_and_upper_endpoint() {
    for p in [2, 3, 5, 7, 11, 97, 997] {
        let n = p * p;
        assert_eq!(
            segmented_sieve(n - 1, n + 1).unwrap(),
            (n - 1..=n + 1).filter(|&v| trial(v)).collect::<Vec<_>>()
        );
    }
    assert_eq!(
        segmented_sieve(MAX_HIGH - 100, MAX_HIGH).unwrap(),
        (MAX_HIGH - 100..=MAX_HIGH)
            .filter(|&n| trial(n))
            .collect::<Vec<_>>()
    );
}
#[test]
fn concatenation_and_order() {
    for low in [0, 1, 2, 47, 1000, 9990] {
        let mut joined = segmented_sieve(low, low + 50).unwrap();
        joined.extend(segmented_sieve(low + 51, low + 100).unwrap());
        let whole = segmented_sieve(low, low + 100).unwrap();
        assert_eq!(whole, joined);
        assert!(whole.windows(2).all(|p| p[0] < p[1]));
    }
}
#[test]
fn reconstruction_gaps() {
    assert_eq!(segmented_sieve(14, 16), segmented_sieve(24, 28));
    assert_eq!(segmented_sieve(11, 13), segmented_sieve(8, 16));
}
