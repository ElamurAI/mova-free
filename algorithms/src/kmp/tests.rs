use super::*;

fn words(max: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}

#[test]
fn exhaustive_windows() {
    for text in words(6) {
        for needle in words(4) {
            let expected: Vec<_> = if needle.is_empty() {
                (0..=text.len()).collect()
            } else {
                text.windows(needle.len())
                    .enumerate()
                    .filter_map(|(i, window)| (window == needle).then_some(i))
                    .collect()
            };
            assert_eq!(find_all(&text, &needle), expected, "{text:?} {needle:?}");
        }
    }
}

#[test]
fn overlaps_fallback_and_bytes() {
    assert_eq!(find_all(b"aaaaa", b"aaa"), vec![0, 1, 2]);
    assert_eq!(find_all(b"abababacaba", b"abac"), vec![4]);
    assert_eq!(find_all("éé".as_bytes(), "é".as_bytes()), vec![0, 2]);
    assert_eq!(find_all(&[255, 0, 255], &[255]), vec![0, 2]);
    assert_eq!(find_all(b"", b""), vec![0]);
}

fn naive(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    text.windows(pattern.len())
        .enumerate()
        .filter_map(|(i, bytes)| (bytes == pattern).then_some(i))
        .collect()
}

#[test]
fn wide_bytes_and_periodic_overlaps_match_naive() {
    let alphabet = [0, 1, 127, 128, 255];
    let mut seed = 0x9e3779b97f4a7c15u64;
    for case in 0..256 {
        let text: Vec<_> = (0..64)
            .map(|_| {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                alphabet[(seed >> 32) as usize % alphabet.len()]
            })
            .collect();
        for len in 0..=9 {
            let start = case % (text.len() - len + 1);
            let pattern = &text[start..start + len];
            assert_eq!(
                find_all(&text, pattern),
                naive(&text, pattern),
                "case={case}, pattern={pattern:?}, text={text:?}"
            );
        }
    }
    for period in [vec![0], vec![0, 255], vec![128, 0, 128]] {
        let text = period.repeat(12);
        let pattern = period.repeat(3);
        assert_eq!(
            find_all(&text, &pattern),
            naive(&text, &pattern),
            "period={period:?}"
        );
    }
    let text = b"abababababababababababababacabababababab";
    let pattern = b"abababababac";
    assert_eq!(find_all(text, pattern), naive(text, pattern));
}

#[test]
fn sentinel_padding_shifts_only_nonempty_matches() {
    for text in words(6) {
        for pattern in words(3).into_iter().filter(|p| !p.is_empty()) {
            // Sentinel 254 occurs in neither binary input nor nonempty pattern.
            for padding in 1..=3 {
                let mut padded = vec![254; padding];
                padded.extend_from_slice(&text);
                padded.extend([254; 2]);
                let expected: Vec<_> = naive(&text, &pattern)
                    .into_iter()
                    .map(|offset| offset + padding)
                    .collect();
                assert_eq!(
                    find_all(&padded, &pattern),
                    expected,
                    "text={text:?}, pattern={pattern:?}, padding={padding}"
                );
            }
        }
    }
}
