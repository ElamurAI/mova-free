use super::*;
use std::collections::VecDeque;
// Independent breadth-first search in the sum graph.
fn bfs(coins: &[usize], amount: usize) -> Option<usize> {
    let mut seen = vec![false; amount + 1];
    let mut queue = VecDeque::from([(0, 0)]);
    seen[0] = true;
    while let Some((sum, count)) = queue.pop_front() {
        if sum == amount {
            return Some(count);
        }
        for &coin in coins {
            if coin <= amount - sum && !seen[sum + coin] {
                seen[sum + coin] = true;
                queue.push_back((sum + coin, count + 1));
            }
        }
    }
    None
}
// Enumerates multiplicities, without a DP table.
fn counts(coins: &[usize], amount: usize) -> Option<usize> {
    if coins.is_empty() {
        return (amount == 0).then_some(0);
    }
    (0..=amount / coins[0])
        .filter_map(|k| counts(&coins[1..], amount - k * coins[0]).map(|rest| rest + k))
        .min()
}
#[test]
fn exhaustive_oracles() {
    for mask in 0..64 {
        let coins: Vec<_> = (1..=6).filter(|c| mask & (1 << (c - 1)) != 0).collect();
        for amount in 0..=30 {
            let result = minimum_coins(&coins, amount).unwrap();
            assert_eq!(result, bfs(&coins, amount), "{coins:?} {amount}");
            assert_eq!(result, counts(&coins, amount), "{coins:?} {amount}");
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(minimum_coins(&[], 0), Ok(Some(0)));
    assert_eq!(minimum_coins(&[], 1), Ok(None));
    assert_eq!(minimum_coins(&[0], 0), Err(Error::ZeroDenomination));
    assert_eq!(
        minimum_coins(&[1, 0], usize::MAX),
        Err(Error::ZeroDenomination)
    );
    assert_eq!(minimum_coins(&[1], usize::MAX), Err(Error::AmountOverflow));
    assert_eq!(
        minimum_coins(&[], isize::MAX as usize),
        Err(Error::AmountOverflow)
    );
    assert_eq!(minimum_coins(&[usize::MAX], 2), Ok(None));
    assert_eq!(minimum_coins(&[1, 3, 4], 6), Ok(Some(2)));
    assert_eq!(minimum_coins(&[3], 9), Ok(Some(3)));
}
#[test]
fn transformations() {
    for amount in 0..40 {
        let expected = minimum_coins(&[2, 5, 7], amount).unwrap();
        assert_eq!(minimum_coins(&[7, 2, 5, 2, 7], amount), Ok(expected));
        assert_eq!(minimum_coins(&[6, 15, 21], amount * 3), Ok(expected));
        let extended = minimum_coins(&[2, 5, 7, 1], amount).unwrap().unwrap();
        if let Some(old) = expected {
            assert!(extended <= old);
        }
    }
}
#[test]
fn finite_preimages() {
    let inputs: Vec<_> = (1..=8)
        .filter(|&coin| minimum_coins(&[coin], 12) == Ok(Some(3)))
        .collect();
    assert_eq!(inputs, vec![4]);
}
