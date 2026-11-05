use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}
#[test]
fn exhaustive_roundtrip() {
    for s in words(10) {
        let codes = encode(&s).unwrap();
        assert_eq!(decode(&codes, s.len()), Ok(s));
    }
}
#[test]
fn known_and_malformed() {
    assert_eq!(encode(b"ABABABA"), Ok(vec![65, 66, 256, 258]));
    assert_eq!(decode(&[65, 256], 3), Ok(b"AAA".to_vec()));
    assert_eq!(decode(&[256], 100), Err(Error::InvalidCode));
    assert_eq!(decode(&[65, 258], 100), Err(Error::InvalidCode));
    assert_eq!(decode(&[65, 256], 2), Err(Error::OutputLimit));
    assert_eq!(decode(&[4096], 0), Err(Error::InvalidCode));
    assert_eq!(decode(&[], 0), Ok(vec![]));
}
#[test]
fn dictionary_freeze() {
    let mut state = 7u32;
    let s: Vec<u8> = (0..20000)
        .map(|_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            (state >> 24) as u8
        })
        .collect();
    let codes = encode(&s).unwrap();
    assert!(codes.iter().all(|&c| c < 4096));
    assert_eq!(decode(&codes, s.len()), Ok(s));
    assert_eq!(encode(&vec![0; 65537]), Err(Error::InputLimit));
}
