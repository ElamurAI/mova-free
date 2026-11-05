use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    let mut result = vec![vec![]];
    for n in 1..=max {
        for mask in 0..(1usize << n) {
            result.push((0..n).map(|i| ((mask >> i) & 1) as u8).collect());
        }
    }
    result
}
fn oracle(t: &[u8], p: &[u8]) -> Vec<usize> {
    (0..=t.len())
        .filter(|&i| t.get(i..i + p.len()) == Some(p))
        .collect()
}
#[test]
fn exhaustive_offsets() {
    for t in words(8) {
        for p in words(6) {
            assert_eq!(find_all(&t, &p), oracle(&t, &p), "{t:?} {p:?}");
        }
    }
}
#[test]
fn periodic_and_nonperiodic() {
    let t = vec![b'a'; 20_000];
    assert_eq!(
        find_all(&t, &vec![b'a'; 1000]),
        (0..=19_000).collect::<Vec<_>>()
    );
    let mut p = vec![b'a'; 1000];
    p[999] = b'b';
    assert!(find_all(&t, &p).is_empty());
    for p in [b"abacaba".as_slice(), b"ababab", b"baaaaa", b"aaabaaa"] {
        let t = p.repeat(700);
        assert_eq!(find_all(&t, p), oracle(&t, p));
    }
}
#[test]
fn boundaries_and_bytes() {
    assert_eq!(find_all(b"aaa", b"aa"), vec![0, 1]);
    assert_eq!(find_all(b"", b""), vec![0]);
    assert_eq!(find_all(b"x", b""), vec![0, 1]);
    assert!(find_all(b"x", b"xx").is_empty());
    let t: Vec<u8> = (0..=255).chain(0..=255).collect();
    for i in 0..256 {
        assert_eq!(find_all(&t, &t[i..i + 1]), vec![i, i + 256]);
    }
}
