use super::{AhoCorasick, Match};

fn oracle(patterns: &[&[u8]], text: &[u8]) -> Vec<Match> {
    let mut found = Vec::new();
    for end in 0..=text.len() {
        for (pattern, bytes) in patterns.iter().enumerate() {
            if end >= bytes.len() && &text[end - bytes.len()..end] == *bytes {
                found.push(Match {
                    pattern,
                    start: end - bytes.len(),
                    end,
                });
            }
        }
    }
    found
}

fn binary_words(max: usize) -> Vec<Vec<u8>> {
    let mut words = Vec::new();
    for n in 0..=max {
        for bits in 0..1usize << n {
            words.push((0..n).map(|j| ((bits >> j) & 1) as u8).collect());
        }
    }
    words
}

#[test]
fn exhaustive_against_byte_oracle() {
    let words = binary_words(3);
    let texts = binary_words(6);
    for a in &words {
        for b in &words {
            let patterns = [a.as_slice(), b.as_slice(), a.as_slice()];
            let machine = AhoCorasick::new(&patterns);
            for text in &texts {
                assert_eq!(machine.find_all(text), oracle(&patterns, text));
            }
        }
    }
}

#[test]
fn boundaries_failure_outputs_and_duplicates() {
    let patterns: &[&[u8]] = &[b"", b"he", b"she", b"hers", b"his", b"he", b""];
    let machine = AhoCorasick::new(patterns);
    for text in [b"ushershishe".as_slice(), b"", b"xyz", b"shers"] {
        assert_eq!(machine.find_all(text), oracle(patterns, text));
    }
    assert!(AhoCorasick::new(&[]).find_all(b"abc").is_empty());
    let bytes: Vec<u8> = (0..=255).collect();
    let patterns: Vec<&[u8]> = bytes.chunks(1).collect();
    assert_eq!(
        AhoCorasick::new(&patterns).find_all(&bytes),
        oracle(&patterns, &bytes)
    );
}

#[test]
fn injective_relabel_and_pattern_permutation() {
    let patterns: &[&[u8]] = &[b"", b"aba", b"ba", b"aba"];
    let text = b"ababa";
    let mapped: Vec<Vec<u8>> = patterns
        .iter()
        .map(|p| p.iter().map(|b| b ^ 255).collect())
        .collect();
    let refs: Vec<&[u8]> = mapped.iter().map(Vec::as_slice).collect();
    let mapped_text: Vec<u8> = text.iter().map(|b| b ^ 255).collect();
    let original = AhoCorasick::new(patterns).find_all(text);
    assert_eq!(original, AhoCorasick::new(&refs).find_all(&mapped_text));
    let reverse: Vec<&[u8]> = patterns.iter().rev().copied().collect();
    let mut relabeled = original;
    for m in &mut relabeled {
        m.pattern = patterns.len() - 1 - m.pattern;
    }
    relabeled.sort_by_key(|m| (m.end, m.pattern));
    assert_eq!(relabeled, AhoCorasick::new(&reverse).find_all(text));
    // A noninjective relabel can create matches and is outside the premise.
    assert_ne!(
        AhoCorasick::new(&[b"a"]).find_all(b"b"),
        AhoCorasick::new(&[b"a"]).find_all(b"a")
    );
}

#[test]
fn compatible_text_fiber_and_prefix_extension() {
    let patterns: &[&[u8]] = &[&[1, 1]];
    let machine = AhoCorasick::new(patterns);
    let target = machine.find_all(&[0, 0, 0]);
    let mut predecessors = 0;
    for text in binary_words(3).into_iter().filter(|x| x.len() == 3) {
        let compatible = !text.windows(2).any(|w| w == [1, 1]);
        assert_eq!(machine.find_all(&text) == target, compatible);
        predecessors += usize::from(compatible);
        let mut extended = text.clone();
        extended.push(1);
        assert_eq!(
            machine
                .find_all(&extended)
                .into_iter()
                .filter(|m| m.end <= text.len())
                .collect::<Vec<_>>(),
            machine.find_all(&text)
        );
    }
    assert_eq!(predecessors, 5);
    assert_ne!(machine.find_all(&[1, 1, 0]), target);
}
