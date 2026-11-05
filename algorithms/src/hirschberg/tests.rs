use super::*;

fn brute(a: &[u8], b: &[u8]) -> usize {
    let mut best = 0;
    for mask in 0..(1usize << a.len()) {
        let subsequence: Vec<_> = a
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, x)| *x)
            .collect();
        let mut position = 0;
        for &x in b {
            if position < subsequence.len() && subsequence[position] == x {
                position += 1;
            }
        }
        if position == subsequence.len() {
            best = best.max(position);
        }
    }
    best
}

fn check(a: &[u8], b: &[u8]) {
    let pairs = hirschberg(a, b).unwrap();
    assert_eq!(pairs.len(), brute(a, b));
    for &(i, j) in &pairs {
        assert!(i < a.len() && j < b.len());
        assert_eq!(a[i], b[j]);
    }
    for pair in pairs.windows(2) {
        assert!(pair[0].0 < pair[1].0 && pair[0].1 < pair[1].1);
    }
    assert_eq!(hirschberg(a, b).unwrap(), pairs);
}

#[test]
fn exhaustive_subsequence_oracle() {
    let mut words = Vec::new();
    for len in 0..=5 {
        for bits in 0..(1usize << len) {
            words.push(
                (0..len)
                    .map(|i| ((bits >> i) & 1) as u8)
                    .collect::<Vec<_>>(),
            );
        }
    }
    for a in &words {
        for b in &words {
            check(a, b);
        }
    }
}

#[test]
fn witness_and_tie_policy() {
    check(b"cabacabad", b"abacdcba");
    assert_eq!(hirschberg(b"ab", b"ba"), Ok(vec![(1, 0)]));
    assert_eq!(hirschberg(b"a", b"aaa"), Ok(vec![(0, 2)]));
    assert_eq!(hirschberg(b"aaa", b"a"), Ok(vec![(2, 0)]));
    assert_eq!(
        hirschberg(b"same", b"same"),
        Ok(vec![(0, 0), (1, 1), (2, 2), (3, 3)])
    );
    assert_eq!(hirschberg(b"aaa", b"bbb"), Ok(vec![]));
}

#[test]
fn dimensions_and_empty() {
    assert_eq!(
        hirschberg(&vec![0; MAX_LENGTH + 1], &[]),
        Err(Error::LengthLimit)
    );
    assert_eq!(
        hirschberg(&vec![0; 4001], &vec![0; 4000]),
        Err(Error::WorkLimit)
    );
    assert_eq!(hirschberg(&vec![0; MAX_LENGTH], &[]), Ok(vec![]));
    assert_eq!(row(usize::MAX), Err(Error::AllocationFailed));
    assert_eq!(hirschberg(&[(); 9], &[(); 3]).unwrap().len(), 3);
}

#[test]
fn symmetry_length_and_extension() {
    for a in [b"abc".as_slice(), b"aab", b"", b"cbaab"] {
        for b in [b"bbc".as_slice(), b"aba", b""] {
            let n = hirschberg(a, b).unwrap().len();
            assert_eq!(n, hirschberg(b, a).unwrap().len());
            let mut aa = a.to_vec();
            aa.push(255);
            let mut bb = b.to_vec();
            bb.push(255);
            assert_eq!(hirschberg(&aa, &bb).unwrap().len(), n + 1);
        }
    }
}
