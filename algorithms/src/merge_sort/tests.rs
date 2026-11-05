use super::*;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Item {
    key: u8,
    identity: usize,
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Item {}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

#[test]
fn exhaustive_stable_bucket_oracle() {
    // Independent oracle enumerates keys and preserves original identities.
    for len in 0..=7u32 {
        for mut code in 0..3usize.pow(len) {
            let mut input = Vec::new();
            for identity in 0..len as usize {
                input.push(Item {
                    key: (code % 3) as u8,
                    identity,
                });
                code /= 3;
            }
            let expected: Vec<_> = (0..3)
                .flat_map(|key| {
                    input
                        .iter()
                        .filter(move |v| v.key == key)
                        .map(|v| (v.key, v.identity))
                })
                .collect();
            merge_sort(&mut input);
            let actual: Vec<_> = input.iter().map(|v| (v.key, v.identity)).collect();
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn long_reverse_and_extremes() {
    let mut values: Vec<_> = (0..1025).rev().collect();
    merge_sort(&mut values);
    assert_eq!(values, (0..1025).collect::<Vec<_>>());
    let mut values = [i64::MAX, 0, i64::MIN, 0];
    merge_sort(&mut values);
    assert_eq!(values, [i64::MIN, 0, 0, i64::MAX]);
    let mut words = ["z".to_owned(), "a".to_owned()];
    merge_sort(&mut words);
    assert_eq!(words, ["a", "z"]);
}
#[test]
fn wider_stable_identity_buckets() {
    let mut seed = 0x6a09e667f3bcc909u64;
    for len in [3, 17, 33, 65, 129, 513] {
        let mut input: Vec<_> = (0..len)
            .map(|identity| {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                Item {
                    key: (seed >> 32) as u8 % 11,
                    identity,
                }
            })
            .collect();
        let expected: Vec<_> = (0..11)
            .flat_map(|key| {
                input
                    .iter()
                    .filter(move |v| v.key == key)
                    .map(|v| (v.key, v.identity))
            })
            .collect();
        merge_sort(&mut input);
        let actual: Vec<_> = input.iter().map(|v| (v.key, v.identity)).collect();
        assert_eq!(actual, expected, "len={len}");
        merge_sort(&mut input);
        assert_eq!(
            input
                .iter()
                .map(|v| (v.key, v.identity))
                .collect::<Vec<_>>(),
            expected,
            "idempotence len={len}"
        );
    }
}

#[test]
fn descending_order_permutation_oracle() {
    use std::cmp::Reverse;
    for len in [0, 1, 2, 31, 256] {
        let input: Vec<_> = (0..len).map(|i| ((i * 73 + 19) % 97) as i64 - 48).collect();
        let mut actual: Vec<_> = input.iter().copied().map(Reverse).collect();
        merge_sort(&mut actual);
        let expected: Vec<_> = (-48..=48)
            .rev()
            .flat_map(|key| {
                input
                    .iter()
                    .filter(move |&&x| x == key)
                    .copied()
                    .map(Reverse)
            })
            .collect();
        assert_eq!(actual, expected, "len={len}");
    }
}
