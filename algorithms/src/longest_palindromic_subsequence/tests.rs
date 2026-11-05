use super::*;
fn oracle(s: &[u8]) -> usize {
    (0..1usize << s.len())
        .map(|mask| {
            let selected: Vec<_> = s
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, b)| *b)
                .collect();
            if selected.iter().eq(selected.iter().rev()) {
                selected.len()
            } else {
                0
            }
        })
        .max()
        .unwrap()
}
#[test]
fn exhaustive_subsequence_oracle() {
    for n in 0..=8 {
        for mut word in 0..3usize.pow(n) {
            let s: Vec<_> = (0..n)
                .map(|_| {
                    let b = (word % 3) as u8;
                    word /= 3;
                    b
                })
                .collect();
            assert_eq!(length(&s), oracle(&s));
            let reversed: Vec<_> = s.iter().rev().copied().collect();
            assert_eq!(length(&s), length(&reversed));
        }
    }
}
#[test]
fn boundaries_generic_and_loss() {
    assert_eq!(length::<u8>(&[]), 0);
    assert_eq!(length(b"a"), 1);
    assert_eq!(length(b"aa"), 2);
    assert_eq!(length(b"abc"), 1);
    assert_eq!(length(b"bbbab"), 4);
    assert_eq!(length(b"agbdba"), 5);
    assert_eq!(length(&["one", "two", "one"]), 3);
    assert_eq!(length(&vec![9; 1000]), 1000);
    assert_eq!(length(b"abc"), length(b"xyz"));
    assert!(length(b"abc") < length(b"aaa"));
    for s in [b"abc".as_slice(), b"abb", b"acbca"] {
        let mut wrapped = vec![255];
        wrapped.extend_from_slice(s);
        wrapped.push(255);
        assert_eq!(length(&wrapped), length(s) + 2);
    }
}
