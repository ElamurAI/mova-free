use super::*;
#[test]
fn exhaustive_sorted_oracle() {
    for width in 1..=5 {
        for code in 0..625usize {
            let mut v = Vec::new();
            let mut c = code;
            let mut h = HistogramQuantiles::new(3, 7, width).unwrap();
            for _ in 0..4 {
                let x = 3 + (c % 5) as u64;
                c /= 5;
                v.push(x);
                h.update(x).unwrap();
            }
            v.sort_unstable();
            for (r, &x) in v.iter().enumerate() {
                let q = h.quantile(r as u64).unwrap();
                assert!(q.value_min <= x && x <= q.value_max, "value certificate");
                assert!(
                    q.rank_start <= r as u64 && (r as u64) < q.rank_end,
                    "rank certificate"
                );
                assert_eq!(
                    q.rank_start,
                    v.iter().filter(|&&v| v < q.value_min).count() as u64
                );
                assert_eq!(
                    q.rank_end,
                    v.iter().filter(|&&v| v <= q.value_max).count() as u64
                );
            }
            assert_eq!(h.quantile(4), None);
        }
    }
}
#[test]
fn endpoints_resources_and_atomicity() {
    assert_eq!(HistogramQuantiles::new(1, 0, 1), Err(Error::InvalidDomain));
    assert_eq!(HistogramQuantiles::new(0, 1, 0), Err(Error::InvalidDomain));
    assert_eq!(
        HistogramQuantiles::new(0, u64::MAX, 1),
        Err(Error::TooManyBuckets)
    );
    let mut h = HistogramQuantiles::new(0, u64::MAX, u64::MAX).unwrap();
    assert!(h.is_empty());
    assert_eq!(h.quantile(0), None);
    h.update(u64::MAX).unwrap();
    let q = h.quantile(0).unwrap();
    assert_eq!((q.value_min, q.value_max), (u64::MAX, u64::MAX));
    h.total = u64::MAX;
    let old = h.clone();
    assert_eq!(h.update(0), Err(Error::Overflow));
    assert_eq!(h, old);
    let mut h = HistogramQuantiles::new(8, 9, 3).unwrap();
    let old = h.clone();
    assert_eq!(h.update(7), Err(Error::OutOfDomain));
    assert_eq!(h, old);
}
#[test]
fn interval_not_relative_value() {
    let mut h = HistogramQuantiles::new(0, 100, 100).unwrap();
    h.update(0).unwrap();
    let q = h.quantile(0).unwrap();
    assert_eq!(
        (q.value_min, q.value_max),
        (0, 99),
        "bucket is not exact value"
    );
    let mut g = HistogramQuantiles::new(0, 100, 100).unwrap();
    g.update(99).unwrap();
    assert_eq!(g, h, "distinct values yield same histogram fiber");
}
#[test]
fn bucket_boundary_mutation_guard() {
    let mut h = HistogramQuantiles::new(0, 9, 5).unwrap();
    h.update(1).unwrap();
    h.update(8).unwrap();
    assert_eq!(
        h.quantile(1).unwrap().value_min,
        5,
        "rank at boundary selects next bucket"
    );
}
