use super::*;
#[test]
fn exhaustive() {
    for code in 0..4096 {
        let mut x = code;
        let mut a = [Item {
            weight: 0,
            value: 0,
        }; 4];
        for item in &mut a {
            item.weight = x % 4;
            item.value = (x % 4) as u64 + 1;
            x /= 4;
            item.value += (x % 4) as u64;
            x /= 4;
        }
        for c in 0..8 {
            let oracle = (0..16)
                .filter_map(|m| {
                    let mut w = 0;
                    let mut v = 0;
                    for (i, item) in a.iter().enumerate() {
                        if m & (1 << i) != 0 {
                            w += item.weight;
                            v += item.value as u128;
                        }
                    }
                    (w <= c).then_some(v)
                })
                .max()
                .unwrap();
            assert_eq!(max_value(&a, c, 8), Ok(oracle));
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(max_value(&[], 0, 2), Ok(0));
    assert_eq!(max_value(&[], 0, 1), Err(Error::BudgetExceeded));
    let a = [Item {
        weight: 0,
        value: u64::MAX,
    }; 4];
    assert_eq!(max_value(&a, 0, 8), Ok(4 * u64::MAX as u128));
    let a = [Item {
        weight: usize::MAX,
        value: 7,
    }; 2];
    assert_eq!(max_value(&a, usize::MAX, 4), Ok(7));
}
