use super::{PatternTooLong, find_all};

fn words(max: usize) -> Vec<Vec<u8>> {
    let mut all = vec![vec![]];
    let mut level = vec![vec![]];
    for _ in 0..max {
        let mut next = Vec::new();
        for word in level {
            for byte in [0, 128, 255] {
                let mut extended = word.clone();
                extended.push(byte);
                next.push(extended);
            }
        }
        all.extend(next.clone());
        level = next;
    }
    all
}

fn naive(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() {
        (0..=text.len()).collect()
    } else {
        text.windows(pattern.len())
            .enumerate()
            .filter_map(|(i, window)| (window == pattern).then_some(i))
            .collect()
    }
}

#[test]
fn exhaustive_windows() {
    for text in words(6) {
        for pattern in words(4) {
            assert_eq!(
                find_all(&text, &pattern).unwrap(),
                naive(&text, &pattern),
                "{text:?}/{pattern:?}"
            );
        }
    }
}

#[test]
fn word_boundaries_and_overlaps() {
    for length in [1, 7, 8, 31, 32, 63, 64] {
        let pattern = vec![255; length];
        assert_eq!(
            find_all(&vec![255; length + 2], &pattern),
            Ok(vec![0, 1, 2])
        );
        let mut mismatch = pattern.clone();
        mismatch[length - 1] = 128;
        assert_eq!(find_all(&pattern, &mismatch), Ok(vec![]));
    }
    let pattern: Vec<u8> = (192..=255).collect();
    let mut text = vec![0];
    text.extend(&pattern);
    text.extend(&pattern);
    assert_eq!(find_all(&text, &pattern), Ok(vec![1, 65]));
    assert_eq!(find_all(b"", b""), Ok(vec![0]));
    assert_eq!(find_all(b"abc", b""), Ok(vec![0, 1, 2, 3]));
}

#[test]
fn typed_length_rejection() {
    for length in [65, 128] {
        let error = PatternTooLong { length };
        assert_eq!(find_all(b"", &vec![0; length]), Err(error));
        assert_eq!(find_all(b"abc", &vec![0; length]), Err(error));
        assert_eq!(
            error.to_string(),
            format!("pattern length {length} exceeds 64 bytes")
        );
    }
}

#[test]
fn oracle_control_and_fibers() {
    assert_ne!(naive(b"aaa", b"aa"), vec![0]);
    assert_eq!(find_all(b"aaa", b"b"), find_all(b"ccc", b"b"));
    assert_ne!(find_all(b"aaa", b"aa").unwrap(), vec![0]);
}
