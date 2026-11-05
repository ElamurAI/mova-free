//! Exact bounded nonnegative knapsack, using binary multiplicity decomposition.
//! Original implementation; classical context: Martello and Toth, Knapsack
//! Problems: Algorithms and Computer Implementations (1990).

pub const MAX_CAPACITY: usize = 1_000_000;
pub const MAX_ITEMS: usize = 100_000;
pub const MAX_UPDATES: usize = 20_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    pub weight: usize,
    pub value: u128,
    pub count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    CapacityTooLarge,
    TooManyItems,
    WorkLimit,
    AllocationFailed,
    ValueOverflow,
}

/// Maximum value with total weight at most `capacity`; no selection witness.
/// Counts are independent even for duplicate items. Zero counts do nothing;
/// zero weights contribute every copy. Arbitrary usize counts are accepted.
/// Positive-weight counts are truncated to capacity/weight before multiplication.
/// Validation order: capacity, item count, update budget, allocation, values.
/// ValueOverflow means the mathematical optimum exceeds u128, not merely an
/// infeasible intermediate sum. Resource errors take precedence over overflow.
/// With B binary bundles, time O(items.len() + capacity * B), space O(capacity).
/// Equal objective values retain the existing row entry.
pub fn bounded_knapsack(items: &[Item], capacity: usize) -> Result<u128, Error> {
    if capacity > MAX_CAPACITY {
        return Err(Error::CapacityTooLarge);
    }
    if items.len() > MAX_ITEMS {
        return Err(Error::TooManyItems);
    }
    let mut updates = 0usize;
    for item in items.iter().filter(|item| item.weight != 0) {
        bundles(item.count.min(capacity / item.weight), |copies| {
            let weight = item.weight * copies;
            updates = updates
                .checked_add(capacity - weight + 1)
                .ok_or(Error::WorkLimit)?;
            if updates > MAX_UPDATES {
                return Err(Error::WorkLimit);
            }
            Ok(())
        })?;
    }
    let mut dp = Vec::new();
    dp.try_reserve_exact(capacity + 1)
        .map_err(|_| Error::AllocationFailed)?;
    dp.resize(capacity + 1, 0u128);
    let mut free = 0u128;
    for item in items {
        if item.weight == 0 {
            let value = item
                .value
                .checked_mul(item.count as u128)
                .ok_or(Error::ValueOverflow)?;
            free = free.checked_add(value).ok_or(Error::ValueOverflow)?;
            continue;
        }
        bundles(item.count.min(capacity / item.weight), |copies| {
            let weight = item.weight * copies;
            let value = item
                .value
                .checked_mul(copies as u128)
                .ok_or(Error::ValueOverflow)?;
            for limit in (weight..=capacity).rev() {
                let candidate = dp[limit - weight]
                    .checked_add(value)
                    .ok_or(Error::ValueOverflow)?;
                dp[limit] = dp[limit].max(candidate);
            }
            Ok(())
        })?;
    }
    free.checked_add(dp[capacity]).ok_or(Error::ValueOverflow)
}

fn bundles(
    mut count: usize,
    mut visit: impl FnMut(usize) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut next = 1;
    while count != 0 {
        let take = next.min(count);
        visit(take)?;
        count -= take;
        // All callers cap positive-weight count at MAX_CAPACITY.
        next *= 2;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
