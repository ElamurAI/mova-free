use super::*;
#[test]
fn all_intervals_including_non_power_lengths() {
    for n in 0..=67 {
        let values: Vec<_> = (0..n).map(|i| (i * 19 % 31) as i64 - 15).collect();
        let table = SparseTable::from_slice(&values);
        assert_eq!(table.len(), n);
        for start in 0..n {
            for end in start + 1..=n {
                assert_eq!(
                    table.query(start, end),
                    values[start..end].iter().min(),
                    "n={n} range={start}..{end}"
                );
            }
        }
    }
}
#[test]
fn empty_invalid_and_extreme_values() {
    let empty = SparseTable::<i64>::from_slice(&[]);
    assert!(empty.is_empty());
    assert_eq!(empty.query(0, 0), None);
    assert_eq!(empty.query(0, 1), None);
    let table = SparseTable::from_slice(&[i64::MAX, 0, i64::MIN, 7]);
    for (start, end) in [
        (0, 0),
        (3, 2),
        (0, 5),
        (usize::MAX, usize::MAX),
        (0, usize::MAX),
    ] {
        assert_eq!(table.query(start, end), None);
    }
    assert_eq!(table.query(0, 4), Some(&i64::MIN));
    assert_eq!(table.query(0, 1), Some(&i64::MAX));
    assert_eq!(table.query(3, 4), Some(&7));
}
#[derive(Clone, Debug)]
struct Tagged {
    key: i32,
    id: usize,
}
impl PartialEq for Tagged {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Tagged {}
impl PartialOrd for Tagged {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Tagged {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
#[test]
fn equal_minima_return_leftmost_identity() {
    let values: Vec<_> = [3, 1, 1, 4, 1, 2, 1, 1, 8]
        .into_iter()
        .enumerate()
        .map(|(id, key)| Tagged { key, id })
        .collect();
    let table = SparseTable::from_slice(&values);
    for start in 0..values.len() {
        for end in start + 1..=values.len() {
            let best = (start..end).min_by_key(|&i| values[i].key).unwrap();
            assert_eq!(
                table.query(start, end).unwrap().id,
                best,
                "range={start}..{end}"
            );
        }
    }
}
#[test]
fn owned_snapshot_handles_generic_noncopy_values() {
    let mut values = vec!["pear".to_string(), "apple".to_string(), "plum".to_string()];
    let table = SparseTable::from_slice(&values);
    values[1] = "zzz".to_string();
    assert_eq!(table.query(0, 3).map(String::as_str), Some("apple"));
    assert_eq!(table.query(2, 3).map(String::as_str), Some("plum"));
}
