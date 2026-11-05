use super::wildcard_match;
fn words(a: &[u8], n: usize) -> Vec<Vec<u8>> {
    (0..a.len().pow(n as u32))
        .map(|mut c| {
            (0..n)
                .map(|_| {
                    let b = a[c % a.len()];
                    c /= a.len();
                    b
                })
                .collect()
        })
        .collect()
}
fn oracle(t: &[u8], p: &[u8]) -> bool {
    match p.split_first() {
        None => t.is_empty(),
        Some((&b'*', tail)) => (0..=t.len()).any(|k| oracle(&t[k..], tail)),
        Some((&token, tail)) => {
            !t.is_empty() && (token == b'?' || token == t[0]) && oracle(&t[1..], tail)
        }
    }
}
#[test]
fn exhaustive_recursive_partition_oracle() {
    for n in 0..=5 {
        for t in words(&[0, 255], n) {
            for m in 0..=5 {
                for p in words(&[0, 255, b'?', b'*'], m) {
                    assert_eq!(wildcard_match(&t, &p), oracle(&t, &p), "{t:?}, {p:?}");
                }
            }
        }
    }
}
#[test]
fn full_bytes_no_escapes_and_empty() {
    assert!(wildcard_match(b"", b""));
    assert!(wildcard_match(b"", b"***"));
    assert!(!wildcard_match(b"", b"?"));
    assert!(!wildcard_match(b"abc", b"b"));
    assert!(wildcard_match(b"abc", b"*b*"));
    assert!(wildcard_match(&[0, 255], b"??"));
    assert!(!wildcard_match("é".as_bytes(), b"?"));
    assert!(wildcard_match("é".as_bytes(), b"??"));
    assert!(!wildcard_match(b"*", br"\*"));
    assert!(wildcard_match(br"\xyz", br"\*"));
    assert!(wildcard_match(b"a*b", b"a?b"));
}
#[test]
fn consecutive_star_collapse() {
    for n in 0..=4 {
        for p in words(b"ab?*", n) {
            let doubled: Vec<_> = p
                .iter()
                .flat_map(|&b| if b == b'*' { vec![b, b] } else { vec![b] })
                .collect();
            for t in words(b"ab", 4) {
                assert_eq!(wildcard_match(&t, &p), wildcard_match(&t, &doubled));
            }
        }
    }
}
#[test]
fn boolean_fiber_preserves_multiple_texts() {
    for t in words(b"ab", 4) {
        assert_eq!(wildcard_match(&t, b"a*b"), t[0] == b'a' && t[3] == b'b');
        assert_eq!(wildcard_match(&t, b"?*b*"), t[1..].contains(&b'b'));
    }
    assert!(wildcard_match(b"ab", b"*"));
    assert!(wildcard_match(b"ba", b"*"));
    assert!(!wildcard_match(b"ab", b"???"));
}
#[test]
fn long_frontier_and_literal_identity() {
    let t = vec![b'a'; 1024];
    assert!(wildcard_match(&t, b"*a*a*a*"));
    assert!(!wildcard_match(&t, b"*a*a*a*b"));
    for t in words(&[0, 255, b'\\'], 5) {
        assert!(wildcard_match(&t, &t));
    }
}
