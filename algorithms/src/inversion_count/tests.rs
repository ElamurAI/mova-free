use super::*;
#[test]
fn exhaustive_pair_oracle() {
    for n in 0..=8 {
        for mut code in 0..3usize.pow(n) {
            let v: Vec<i32> = (0..n)
                .map(|_| {
                    let x = (code % 3) as i32 - 1;
                    code /= 3;
                    x
                })
                .collect();
            let original = v.clone();
            let mut expected = 0u128;
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    if v[i] > v[j] {
                        expected += 1;
                    }
                }
            }
            assert_eq!(count_inversions(&v), Ok(expected), "{v:?}");
            assert_eq!(v, original);
        }
    }
}
#[test]
fn literal_ties_and_non_clone_order() {
    assert_eq!(count_inversions(&[3, 1, 3, 1]), Ok(3));
    assert_eq!(count_inversions(&[1; 64]), Ok(0));
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Key(i32);
    assert_eq!(count_inversions(&[Key(2), Key(1)]), Ok(1));
}
#[test]
fn cap_and_exact_large_total() {
    let v: Vec<_> = (0..MAX_LEN).rev().collect();
    assert_eq!(
        count_inversions(&v),
        Ok((MAX_LEN as u128) * (MAX_LEN as u128 - 1) / 2)
    );
    assert_eq!(count_inversions(&vec![0; MAX_LEN + 1]), Err(Error::TooLong));
}
