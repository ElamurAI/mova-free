use super::*;
fn distance(a: &[u8], b: &[u8]) -> usize {
    let mut table = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in table.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, v) in table[0].iter_mut().enumerate() {
        *v = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            table[i][j] = [
                table[i - 1][j] + 1,
                table[i][j - 1] + 1,
                table[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]),
            ]
            .into_iter()
            .min()
            .unwrap();
        }
    }
    table[a.len()][b.len()]
}
#[test]
fn exhaustive_metric_oracle() {
    let mut words = vec![vec![]];
    for n in 1..=4 {
        for mask in 0..1usize << n {
            words.push((0..n).map(|i| ((mask >> i) & 1) as u8).collect());
        }
    }
    for shift in 0..words.len() {
        let mut rotated = words.clone();
        rotated.rotate_left(shift);
        rotated.push(vec![]);
        let tree = BkTree::new(&rotated).unwrap();
        for q in &words {
            for radius in [0, 1, 2, 4, usize::MAX] {
                let expected: Vec<_> = rotated
                    .iter()
                    .enumerate()
                    .filter_map(|(i, w)| {
                        let d = distance(q, w);
                        (d <= radius).then_some((i, d))
                    })
                    .collect();
                assert_eq!(tree.within(q, radius).unwrap(), expected);
            }
        }
    }
}
#[test]
fn byte_semantics_and_boundaries() {
    let words = vec![vec![255, 0], vec![255], vec![], vec![255, 0]];
    assert_eq!(
        BkTree::new(&words).unwrap().within(&[255, 0], 0).unwrap(),
        vec![(0, 0), (3, 0)]
    );
    assert_eq!(BkTree::default().within(&[], usize::MAX).unwrap(), vec![]);
    assert!(BkTree::default().within(&[0; 257], 0).is_err());
    assert_eq!(
        BkTree::new(&vec![vec![0; 257]; 4097])
            .unwrap_err()
            .to_string(),
        "too many words"
    );
    assert_eq!(
        BkTree::new(&[vec![0; 257]]).unwrap_err().to_string(),
        "word too long"
    );
    let tree = BkTree::new(&vec![vec![0; 256]; 4096]).unwrap();
    assert_eq!(tree.within(&[1; 256], 256).unwrap().len(), 4096);
}
