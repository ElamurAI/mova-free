use super::{ShortSideTooLong, length};

fn oracle(a: &[u8], b: &[u8]) -> usize {
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    dp[a.len()][b.len()]
}
fn words() -> Vec<Vec<u8>> {
    (0..=5)
        .flat_map(|n| {
            (0..1_usize << n).map(move |bits| {
                (0..n)
                    .map(|i| if bits & (1 << i) == 0 { 0 } else { 255 })
                    .collect()
            })
        })
        .collect()
}
#[test]
fn exhaustive_lcs_against_matrix() {
    let words = words();
    for a in &words {
        for b in &words {
            let expected = oracle(a, b);
            assert!(expected <= a.len().min(b.len()));
            assert_eq!(length(a, b), Ok(expected), "{a:?} {b:?}");
            assert_eq!(length(b, a), Ok(expected));
        }
    }
}
#[test]
fn top_bit_borrow_and_full_word() {
    let unique: Vec<u8> = (0..64).collect();
    assert_eq!(length(&unique, &unique), Ok(64));
    assert_eq!(length(&unique, &[63]), Ok(1));
    let mut long = vec![255; 70];
    long[69] = 63;
    assert_eq!(length(&unique, &long), Ok(1)); // position 63 in state
    assert_eq!(length(&unique, &[255; 80]), Ok(0)); // borrow from zero
    let mut reverse = unique.clone();
    reverse.reverse();
    assert_eq!(length(&unique, &reverse), Ok(1));
    for seed in 0..16 {
        let a: Vec<u8> = (0..64).map(|i| ((i * 7 + seed) % 13) as u8).collect();
        let b: Vec<u8> = (0..91).map(|i| ((i * 11 + seed * 3) % 17) as u8).collect();
        assert_eq!(length(&a, &b), Ok(oracle(&a, &b)));
    }
}
#[test]
fn length_cap_and_empty_domain() {
    assert_eq!(
        length(&[1; 65], &[1; 66]),
        Err(ShortSideTooLong { length: 65 })
    );
    assert_eq!(
        length(&[1; 66], &[1; 65]),
        Err(ShortSideTooLong { length: 65 })
    );
    assert_eq!(length(&[1; 64], &[1; 100]), Ok(64));
    assert_eq!(length(&[], &[1; 100]), Ok(0));
    assert_eq!(length(&[1; 100], &[]), Ok(0));
    assert_eq!(length(&[], &[]), Ok(0));
}
#[test]
fn subsequence_not_substring_and_fibers() {
    assert_eq!(length(b"abc", b"axbyc"), Ok(3));
    assert_eq!(length(b"ab", b"ba"), Ok(1));
    assert_eq!(length("é".as_bytes(), "è".as_bytes()), Ok(1));
    assert_eq!(length(b"ab", b"ac"), length(b"xy", b"xz"));
    assert_ne!(oracle(b"abc", b"axbyc"), 1); // false substring oracle
}
