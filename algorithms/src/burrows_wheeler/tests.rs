use super::{BwtError, inverse, transform};
fn words(n: usize) -> impl Iterator<Item = Vec<u8>> {
    (0..3usize.pow(n as u32)).map(move |mut c| {
        (0..n)
            .map(|_| {
                let b = (c % 3) as u8;
                c /= 3;
                b
            })
            .collect()
    })
}
fn oracle(t: &[u8]) -> (Vec<u8>, usize) {
    if t.is_empty() {
        return (vec![], 0);
    }
    let n = t.len();
    let mut rotations: Vec<_> = (0..n)
        .map(|i| {
            let row: Vec<_> = t[i..].iter().chain(&t[..i]).copied().collect();
            (row, i)
        })
        .collect();
    rotations.sort();
    let primary = rotations.iter().position(|(_, i)| *i == 0).unwrap();
    (
        rotations.iter().map(|(row, _)| row[n - 1]).collect(),
        primary,
    )
}
#[test]
fn exhaustive_materialized_rotation_oracle() {
    for n in 0..=8 {
        for t in words(n) {
            let e = transform(&t);
            assert_eq!(e, oracle(&t), "{t:?}");
            assert_eq!(inverse(&e.0, e.1), Ok(t));
        }
    }
}
#[test]
fn periodic_ties_and_boundaries() {
    assert_eq!(transform(b""), (vec![], 0));
    assert_eq!(transform(b"x"), (b"x".to_vec(), 0));
    assert_eq!(transform(b"abab"), (b"bbaa".to_vec(), 0));
    assert_eq!(transform(b"baba"), (b"bbaa".to_vec(), 2));
    assert_eq!(transform(b"aaaa"), (b"aaaa".to_vec(), 0));
    assert_eq!(transform(b"banana"), (b"nnbaaa".to_vec(), 3));
    let all: Vec<u8> = (0..=255).rev().collect();
    let e = transform(&all);
    assert_eq!(e, oracle(&all));
    assert_eq!(inverse(&e.0, e.1), Ok(all));
}
#[test]
fn inverse_reachability_by_independent_search() {
    for n in 0..=5 {
        let mut reachable = std::collections::BTreeMap::new();
        for t in words(n) {
            assert!(reachable.insert(oracle(&t), t).is_none());
        }
        for c in words(n) {
            for p in 0..=n {
                let actual = inverse(&c, p);
                if let Some(t) = reachable.get(&(c.clone(), p)) {
                    assert_eq!(actual, Ok(t.clone()));
                } else {
                    let expected = if (n == 0 && p != 0) || (n > 0 && p >= n) {
                        BwtError::InvalidPrimaryIndex
                    } else {
                        BwtError::InvalidTransform
                    };
                    assert_eq!(actual, Err(expected), "{c:?}, {p}");
                }
            }
        }
    }
    assert_eq!(inverse(b"", usize::MAX), Err(BwtError::InvalidPrimaryIndex));
    assert_eq!(
        inverse(b"a", usize::MAX),
        Err(BwtError::InvalidPrimaryIndex)
    );
}
#[test]
fn column_projection_has_rotation_fiber() {
    assert_eq!(transform(b"abab").0, transform(b"baba").0);
    assert_ne!(transform(b"abab").1, transform(b"baba").1);
    assert_eq!(inverse(b"bbaa", 1), Err(BwtError::InvalidTransform));
    assert_eq!(inverse(b"bbaa", 3), Err(BwtError::InvalidTransform));
}
#[test]
fn multiset_and_increasing_relabel() {
    for t in words(6) {
        let (c, p) = transform(&t);
        let mut sc = c.clone();
        let mut st = t.clone();
        sc.sort();
        st.sort();
        assert_eq!(sc, st);
        let mapped: Vec<_> = t.iter().map(|b| b * 17 + 5).collect();
        let expected: Vec<_> = c.iter().map(|b| b * 17 + 5).collect();
        assert_eq!(transform(&mapped), (expected, p));
    }
}
