use super::*;
// Strictly smaller than every nontrivial rotation; length-one words qualify.
fn lyndon(word: &[u8]) -> bool {
    (1..word.len()).all(|cut| {
        let rotation: Vec<u8> = word[cut..].iter().chain(&word[..cut]).copied().collect();
        word < rotation.as_slice()
    })
}
// Enumerate every partition, independently of Duval's scan and period rule.
fn partitions(
    bytes: &[u8],
    start: usize,
    current: &mut Vec<Range<usize>>,
    solutions: &mut Vec<Vec<Range<usize>>>,
) {
    if start == bytes.len() {
        solutions.push(current.clone());
        return;
    }
    for end in start + 1..=bytes.len() {
        let word = &bytes[start..end];
        if lyndon(word) && current.last().is_none_or(|r| &bytes[r.clone()] >= word) {
            current.push(start..end);
            partitions(bytes, end, current, solutions);
            current.pop();
        }
    }
}
#[test]
fn exhaustive_partitions() {
    for n in 0..=8 {
        for mask in 0..1usize << n {
            let bytes: Vec<u8> = (0..n).map(|bit| ((mask >> bit) & 1) as u8).collect();
            let mut solutions = Vec::new();
            partitions(&bytes, 0, &mut Vec::new(), &mut solutions);
            assert_eq!(solutions.len(), 1);
            assert_eq!(factorize(&bytes), solutions[0]);
        }
    }
}
#[test]
fn periodic_and_unsigned_bytes() {
    assert_eq!(factorize(b"ababab"), vec![0..2, 2..4, 4..6]);
    assert_eq!(factorize(&[255, 0, 255, 0]), vec![0..1, 1..3, 3..4]);
    assert_eq!(factorize(b"cba"), vec![0..1, 1..2, 2..3]);
}
#[test]
fn coverage_and_order() {
    for bytes in [b"banana".as_slice(), b"mississippi", b"aaaabaaaab", b""] {
        let factors = factorize(bytes);
        assert_eq!(
            factors
                .iter()
                .flat_map(|r| bytes[r.clone()].to_vec())
                .collect::<Vec<_>>(),
            bytes
        );
        assert!(factors.iter().all(|r| lyndon(&bytes[r.clone()])));
        assert!(
            factors
                .windows(2)
                .all(|p| bytes[p[0].clone()] >= bytes[p[1].clone()])
        );
    }
}
#[test]
fn range_observation_fibers() {
    assert_eq!(factorize(b"ab"), factorize(b"xy"));
    assert_eq!(factorize(b"aa"), vec![0..1, 1..2]);
    assert!(factorize(&[]).is_empty());
}
