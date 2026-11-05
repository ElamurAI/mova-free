use super::*;
use std::collections::BTreeSet;
fn substrings(s: &[u8]) -> BTreeSet<Vec<u8>> {
    (0..s.len())
        .flat_map(|i| (i + 1..=s.len()).map(move |j| s[i..j].to_vec()))
        .collect()
}
#[test]
fn exhaustive_substring_oracle() {
    for n in 0..=7 {
        for mut word in 0..3usize.pow(n) {
            let s: Vec<_> = (0..n)
                .map(|_| {
                    let b = (word % 3) as u8;
                    word /= 3;
                    b
                })
                .collect();
            let expected = substrings(&s);
            let a = SuffixAutomaton::new(&s);
            assert_eq!(a.distinct_substrings(), expected.len() as u128);
            assert!(a.contains(&[]));
            for q in &expected {
                assert!(a.contains(q));
            }
            for m in 1..=4 {
                for mut bits in 0..4usize.pow(m) {
                    let q: Vec<_> = (0..m)
                        .map(|_| {
                            let b = (bits % 4) as u8;
                            bits /= 4;
                            b
                        })
                        .collect();
                    assert_eq!(a.contains(&q), expected.contains(&q));
                }
            }
            assert!(a.states.len() <= 2 * s.len().max(1));
            for state in a.states.iter().skip(1) {
                assert!(a.states[state.link.unwrap()].longest < state.longest);
            }
        }
    }
}
#[test]
fn boundaries_and_projections() {
    assert_eq!(SuffixAutomaton::new(&[]).distinct_substrings(), 0);
    assert!(!SuffixAutomaton::new(&[]).contains(&[0]));
    for s in [
        b"banana".as_slice(),
        b"abcbc",
        "ї🙂".as_bytes(),
        &[0, 255, 0],
    ] {
        let a = SuffixAutomaton::new(s);
        assert_eq!(a.distinct_substrings(), substrings(s).len() as u128);
        assert!(a.contains(s));
        let reversed: Vec<_> = s.iter().rev().copied().collect();
        assert_eq!(
            a.distinct_substrings(),
            SuffixAutomaton::new(&reversed).distinct_substrings()
        );
    }
    let a = SuffixAutomaton::new(b"ab");
    let b = SuffixAutomaton::new(b"ba");
    assert_eq!(a.distinct_substrings(), b.distinct_substrings());
    assert_ne!(a.contains(b"ab"), b.contains(b"ab"));
    assert_eq!(
        SuffixAutomaton::new(&vec![7; 10000]).distinct_substrings(),
        10000
    );
}
