use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    let mut all = vec![vec![]];
    for len in 1..=max {
        for bits in 0..1usize << len {
            all.push((0..len).map(|i| ((bits >> i) & 1) as u8).collect());
        }
    }
    all
}

fn oracle(a: &[u8], b: &[u8]) -> Match {
    for len in (1..=a.len().min(b.len())).rev() {
        for i in 0..=a.len() - len {
            for j in 0..=b.len() - len {
                if a[i..i + len] == b[j..j + len] {
                    return Match {
                        a_start: i,
                        b_start: j,
                        len,
                    };
                }
            }
        }
    }
    Match::default()
}
#[test]
fn bounded_oracle() {
    let all = words(5);
    for a in &all {
        for b in &all {
            assert_eq!(longest_common_substring(a, b), oracle(a, b));
        }
    }
}
#[test]
fn ties_contiguity_and_empty() {
    assert_eq!(
        longest_common_substring(b"abXcd", b"cdYab"),
        Match {
            a_start: 0,
            b_start: 3,
            len: 2
        }
    );
    assert_eq!(
        longest_common_substring(b"ab", b"abXab"),
        Match {
            a_start: 0,
            b_start: 0,
            len: 2
        }
    );
    assert_eq!(longest_common_substring(b"abc", b"ac").len, 1);
    assert_eq!(longest_common_substring(b"", b"a"), Match::default());
    assert_eq!(longest_common_substring(b"x", b"y"), Match::default());
}
#[test]
fn witnesses_fibers_and_units() {
    for a in words(4) {
        for b in words(3) {
            let w = longest_common_substring(&a, &b);
            assert_eq!(
                a[w.a_start..w.a_start + w.len],
                b[w.b_start..w.b_start + w.len]
            );
            let fiber: Vec<_> = words(4)
                .into_iter()
                .filter(|x| oracle(x, &b) == w)
                .collect();
            assert!(fiber.contains(&a));
            assert_eq!(longest_common_substring(&b, &a).len, w.len);
        }
    }
    assert_eq!(
        longest_common_substring("é".as_bytes(), "é".as_bytes()).len,
        2
    );
    assert_eq!(longest_common_substring(&['é'], &['é']).len, 1);
}
