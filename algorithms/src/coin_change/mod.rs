//! Minimum count of unbounded positive-denomination coins.
//! Original implementation of the standard amount dynamic program.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ZeroDenomination,
    AmountOverflow,
    AllocationFailed,
}
/// None means no exact sum exists. Duplicates do not change the answer.
/// Zeros are rejected even for amount zero. O((amount+1) * (denominations.len()+1))
/// time and O(amount+1) space, with checked dimensions and fallible reservation.
pub fn minimum_coins(denominations: &[usize], amount: usize) -> Result<Option<usize>, Error> {
    if denominations.contains(&0) {
        return Err(Error::ZeroDenomination);
    }
    let len = amount.checked_add(1).ok_or(Error::AmountOverflow)?;
    let bytes = len
        .checked_mul(std::mem::size_of::<Option<usize>>())
        .ok_or(Error::AmountOverflow)?;
    if bytes > isize::MAX as usize {
        return Err(Error::AmountOverflow);
    }
    let mut best: Vec<Option<usize>> = Vec::new();
    best.try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    best.resize(len, None);
    best[0] = Some(0);
    for total in 1..=amount {
        for &coin in denominations {
            if coin <= total
                && let Some(previous) = best[total - coin]
            {
                // Positive coins imply previous <= total - coin < total.
                let candidate = previous + 1;
                if best[total].is_none_or(|old| candidate < old) {
                    best[total] = Some(candidate);
                }
            }
        }
    }
    Ok(best[amount])
}
#[cfg(test)]
mod tests;
