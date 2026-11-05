use super::*;
#[test]
fn exact_frequency_oracle() {
    for p in [1, 2, 5, 10] {
        let mut s = SpaceSaving::new(p).unwrap();
        let mut exact = BTreeMap::new();
        for i in 0..3000u64 {
            let key = if i % 3 == 0 {
                u64::MAX
            } else {
                (i * 7919 + i / 7) % 137
            };
            s.update(key).unwrap();
            *exact.entry(key).or_insert(0u64) += 1;
            for key in (0..140).chain([u64::MAX]) {
                let f = exact.get(&key).copied().unwrap_or(0);
                let (lo, hi) = s.bounds(key);
                assert!(
                    lo <= f && f <= hi,
                    "frequency certificate key={key} p={p} n={i}"
                );
                assert!(
                    (hi - lo) as u128 * 100 <= s.len() as u128 * p as u128,
                    "additive normalized bound"
                );
            }
            assert!(s.counters.len() <= 100 / p as usize);
        }
    }
}
#[test]
fn boundaries_atomicity() {
    for p in [0, 3, 11, 15, 100, 255] {
        assert!(SpaceSaving::new(p).is_err());
    }
    let mut s = SpaceSaving::new(10).unwrap();
    assert!(s.is_empty());
    assert_eq!(s.bounds(4), (0, 0));
    s.update(u64::MAX).unwrap();
    assert_eq!(s.bounds(u64::MAX), (1, 1));
    s.total = u64::MAX;
    let old = s.clone();
    assert_eq!(s.update(2), Err(Error::Overflow));
    assert_eq!(s, old);
}
#[test]
fn permutation_fiber_and_replay() {
    let a = [1, 2, 1, 3, 2];
    let b = [3, 2, 1, 2, 1];
    let mut x = SpaceSaving::new(10).unwrap();
    let mut y = x.clone();
    for v in a {
        x.update(v).unwrap();
    }
    for v in b {
        y.update(v).unwrap();
    }
    assert_eq!(x, y, "nonunique ordered stream fiber below capacity");
    for _ in 0..30 {
        for key in 0..50 {
            x.update(key).unwrap();
            y.update(key).unwrap();
        }
    }
    assert_eq!(x, y, "deterministic replay");
}

#[test]
fn replacement_certificate_and_tie() {
    let mut s = SpaceSaving::new(10).unwrap();
    for key in 0..10 {
        s.update(key).unwrap();
    }
    s.update(99).unwrap();
    assert!(
        !s.counters.contains_key(&0),
        "smallest key breaks minimum tie"
    );
    assert_eq!(
        s.counters[&99],
        (2, 1),
        "replacement inherits one unit error"
    );
    assert_eq!(s.bounds(99), (1, 2));
    for key in 0..1000 {
        s.update(key % 27).unwrap();
        assert_eq!(
            s.counters.values().map(|x| x.0).sum::<u64>(),
            s.len(),
            "mass conservation"
        );
    }
}
