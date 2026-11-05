use super::*;
use std::collections::BTreeMap;

fn oracle(bytes: &[u8]) -> BTreeMap<Vec<u8>, (Range<usize>, usize)> {
    let mut result = BTreeMap::new();
    for start in 0..bytes.len() {
        for end in start + 1..=bytes.len() {
            let part = &bytes[start..end];
            if part.iter().eq(part.iter().rev()) {
                let entry = result.entry(part.to_vec()).or_insert((start..end, 0));
                entry.1 += 1;
            }
        }
    }
    result
}
fn check(bytes: &[u8]) {
    let expected = oracle(bytes);
    let tree = build(bytes).unwrap();
    let mut actual = BTreeMap::new();
    for (i, p) in tree.palindromes().iter().enumerate() {
        let word = &bytes[p.range.clone()];
        assert!(
            actual
                .insert(word.to_vec(), (p.range.clone(), p.occurrences))
                .is_none()
        );
        let suffix = (1..word.len()).find(|&j| word[j..].iter().eq(word[j..].iter().rev()));
        let linked = p.suffix_link.map(|j| {
            assert!(j < i);
            &bytes[tree.palindromes()[j].range.clone()]
        });
        assert_eq!(linked, suffix.map(|j| &word[j..]));
    }
    assert_eq!(actual, expected);
    assert_eq!(tree.distinct_count(), expected.len());
    let longest = expected
        .values()
        .map(|(r, _)| r.clone())
        .min_by_key(|r| (std::cmp::Reverse(r.len()), r.start));
    assert_eq!(tree.longest(), longest);
}
#[test]
fn exhaustive_substrings() {
    for n in 0..=9 {
        for mask in 0..1usize << n {
            let bytes: Vec<u8> = (0..n).map(|bit| ((mask >> bit) & 1) as u8).collect();
            check(&bytes);
        }
    }
}
#[test]
fn first_witness_and_overlap() {
    check(b"abacaba");
    check(b"aaaaaa");
    check(&[255, 0, 255, 0, 255]);
    assert_eq!(build(b"aba!cdc").unwrap().longest(), Some(0..3));
    assert_eq!(build(b"aaaa").unwrap().palindromes()[0].occurrences, 4);
}
#[test]
fn length_boundary() {
    assert_eq!(build(&vec![0; MAX_LEN + 1]), Err(Error::LengthLimit));
    let tree = build(&vec![0; MAX_LEN]).unwrap();
    assert_eq!(tree.distinct_count(), MAX_LEN);
    assert_eq!(tree.longest(), Some(0..MAX_LEN));
    assert_eq!(tree.palindromes()[0].occurrences, MAX_LEN);
}
#[test]
fn empty_and_reconstruction_ambiguity() {
    assert_eq!(build(&[]).unwrap().longest(), None);
    assert_eq!(build(b"a").unwrap(), build(b"b").unwrap());
    assert_ne!(oracle(b"a"), oracle(b"b"));
}
