use super::*;
#[test]
fn exhaustive_closed_overlap_oracle() {
    let all: Vec<_> = (-1..=1)
        .flat_map(|a| (a..=1).map(move |b| (a, b)))
        .collect();
    for mask in 0..1usize << all.len() {
        let entries: Vec<_> = all
            .iter()
            .enumerate()
            .filter(|&(i, _)| mask & (1 << i) != 0)
            .map(|(_, v)| *v)
            .collect();
        let t = IntervalTree::new(&entries).unwrap();
        for a in -2..=2 {
            for b in a..=2 {
                let expected: Vec<_> = entries
                    .iter()
                    .enumerate()
                    .filter(|&(_, &(l, r))| !(r < a || b < l))
                    .map(|(i, _)| i)
                    .collect();
                assert_eq!(t.overlapping((a, b)).unwrap(), expected);
            }
        }
    }
}
#[test]
fn duplicate_touching_full_width() {
    let intervals = [
        (i64::MIN, i64::MIN),
        (i64::MIN, i64::MAX),
        (0, 0),
        (0, 0),
        (i64::MAX, i64::MAX),
    ];
    let t = IntervalTree::new(&intervals).unwrap();
    assert_eq!(t.overlapping((0, 0)).unwrap(), vec![1, 2, 3]);
    assert_eq!(t.overlapping((i64::MIN, i64::MIN)).unwrap(), vec![0, 1]);
    assert_eq!(t.overlapping((i64::MAX, i64::MAX)).unwrap(), vec![1, 4]);
    assert_eq!(
        t.overlapping((i64::MIN, i64::MAX)).unwrap(),
        vec![0, 1, 2, 3, 4]
    );
}
#[test]
fn validation_and_capacity() {
    assert_eq!(
        IntervalTree::new(&[(1, 0); 4097]).unwrap_err().to_string(),
        "too many intervals"
    );
    assert!(IntervalTree::new(&[(1, 0)]).is_err());
    assert!(IntervalTree::default().overlapping((1, 0)).is_err());
    assert!(
        IntervalTree::default()
            .overlapping((0, 0))
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        IntervalTree::new(&[(0, 0); 4096])
            .unwrap()
            .overlapping((0, 0))
            .unwrap()
            .len(),
        4096
    );
}
