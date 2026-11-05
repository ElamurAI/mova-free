use super::length;

fn oracle(a: &[u8], b: &[u8]) -> usize {
    (0usize..(1 << a.len()))
        .filter_map(|mask| {
            let candidate: Vec<_> = a
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, x)| *x)
                .collect();
            let mut cursor = 0;
            for &x in b {
                if cursor < candidate.len() && candidate[cursor] == x {
                    cursor += 1;
                }
            }
            (cursor == candidate.len()).then_some(cursor)
        })
        .max()
        .unwrap()
}
fn words() -> Vec<Vec<u8>> {
    (0..=5)
        .flat_map(|n| {
            (0..(1usize << n)).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}

#[test]
fn exhaustive_subsequence_oracle() {
    let domain = words();
    for a in &domain {
        for b in &domain {
            assert_eq!(length(a, b), oracle(a, b), "{a:?} {b:?}");
        }
    }
}

#[test]
fn boundaries_and_generic_eq() {
    assert_eq!(length::<u8>(&[], &[]), 0);
    assert_eq!(length(&[1], &[]), 0);
    assert_eq!(length(&[1; 80], &[1; 13]), 13);
    assert_eq!(length(&["alpha", "beta", "gamma"], &["gamma", "alpha"]), 1);
    assert_eq!(length(b"ABCBDAB", b"BDCABA"), 4);
    assert_eq!(length(&[(); 19], &[(); 8]), 8);
}

#[test]
fn symmetry_identity_reversal_and_extension() {
    let domain = words();
    for a in &domain {
        for b in &domain {
            let value = length(a, b);
            assert_eq!(value, length(b, a));
            assert_eq!(length(a, a), a.len());
            assert!(value <= a.len().min(b.len()));
            let ar: Vec<_> = a.iter().rev().copied().collect();
            let br: Vec<_> = b.iter().rev().copied().collect();
            assert_eq!(length(&ar, &br), value);
            let mut ax = a.clone();
            ax.push(9);
            let mut bx = b.clone();
            bx.push(9);
            assert_eq!(length(&ax, &bx), value + 1);
            assert!((value..=value + 1).contains(&length(&ax, b)));
        }
    }
}

#[test]
fn preimage_ambiguity_and_false_multiset_model() {
    assert_eq!(length(b"ab", b"ab"), 2);
    assert_eq!(length(b"cd", b"cd"), 2);
    assert_eq!(length(b"ab", b"ba"), 1);
    let fiber: Vec<_> = words()
        .into_iter()
        .filter(|x| length(x, b"2") == 0)
        .collect();
    assert!(fiber.len() > 1);
    assert_eq!(length(b"aaaa", b"bbbb"), 0);
}
