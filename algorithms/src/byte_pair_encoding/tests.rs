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
    for s in words(9) {
        let m = Model::train(&s, 8).unwrap();
        let t = m.encode(&s).unwrap();
        assert_eq!(m.decode(&t, s.len()), Ok(s.clone()));
        assert_eq!(m, Model::train(&s, 8).unwrap());
    }
}
#[test]
fn ties_and_model_identity() {
    let m = Model::train(b"ababcdcd", 1).unwrap();
    assert_eq!(m.merges(), &[(97, 98)]);
    assert_eq!(m.encode(b"abab"), Ok(vec![256, 256]));
    assert_eq!(m.encode(b"cdcd"), Ok(vec![99, 100, 99, 100]));
    let other = Model::train(b"cdcd", 1).unwrap();
    assert_eq!(other.decode(&[256], 2), Ok(b"cd".to_vec()));
    assert_eq!(m.decode(&[256], 2), Ok(b"ab".to_vec()));
    assert_eq!(m.decode(&[257], 0), Err(Error::InvalidToken));
    assert_eq!(m.decode(&[256], 1), Err(Error::OutputLimit));
}
#[test]
fn overlapping_and_limits() {
    let m = Model::train(b"aaaaa", 3).unwrap();
    assert_eq!(m.merges(), &[(97, 97)]);
    assert_eq!(m.encode(b"aaaaa"), Ok(vec![256, 256, 97]));
    assert_eq!(Model::train(&[], 3841), Err(Error::MergeLimit));
    assert_eq!(Model::train(&vec![0; 65537], 0), Err(Error::InputLimit));
    assert_eq!(Model::train(&[], 1).unwrap().decode(&[], 0), Ok(vec![]));
}
