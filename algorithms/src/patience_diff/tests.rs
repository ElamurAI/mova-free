use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}
#[test]
fn exhaustive_patch() {
    let w = words(5);
    for a in &w {
        for b in &w {
            let p = diff(a, b).unwrap();
            assert_eq!(apply(a, &p, b.len()), Ok(b.clone()));
            let consumed: Vec<u8> = p
                .iter()
                .filter_map(|e| match e {
                    Edit::Equal(x) | Edit::Delete(x) => Some(*x),
                    _ => None,
                })
                .collect();
            assert_eq!(&consumed, a);
            assert_eq!(p, diff(a, b).unwrap());
        }
    }
}
#[test]
fn anchors_and_invalid() {
    assert_eq!(
        diff(b"abc", b"axc"),
        Ok(vec![
            Edit::Equal(b'a'),
            Edit::Delete(b'b'),
            Edit::Insert(b'x'),
            Edit::Equal(b'c')
        ])
    );
    assert_eq!(apply(b"a", &[], 0), Err(Error::InvalidPatch));
    assert_eq!(
        apply(b"a", &[Edit::Equal(b'b')], 0),
        Err(Error::InvalidPatch)
    );
    assert_eq!(
        apply(b"a", &[Edit::Equal(b'a')], 0),
        Err(Error::OutputLimit)
    );
    assert_eq!(diff(&vec![0; 4097], &[]), Err(Error::InputLimit));
}
#[test]
fn unique_anchor_and_nonminimal_witness() {
    let p = diff(b"xAyBz", b"qArBs").unwrap();
    assert!(p.contains(&Edit::Equal(b'A')));
    assert!(p.contains(&Edit::Equal(b'B')));
    let p = diff(b"abab", b"baba").unwrap();
    let cost = p.iter().filter(|e| !matches!(e, Edit::Equal(_))).count();
    assert_eq!(cost, 8);
    assert_eq!(apply(b"abab", &p, 4), Ok(b"baba".to_vec())); /* optimum 2, demonstrating no minimality claim */
}
