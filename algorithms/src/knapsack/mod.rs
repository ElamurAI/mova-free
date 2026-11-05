//! Exact 0/1 knapsack with one capacity row.

/// One independently selectable item; equal items remain separate choices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    pub weight: usize,
    pub value: u64,
}

/// The requested DP row cannot be represented or allocated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    CapacityOverflow,
    AllocationFailed,
}

/// Maximum total value among subsets whose total weight is at most capacity.
/// Each item, including a zero-weight item, is used at most once.
/// Values accumulate exactly in u128 on supported Rust targets (usize <=64 bits).
/// Takes O(items.len()*(capacity+1)) time and O(capacity+1) space.
/// Dimension arithmetic is checked before allocation; allocator refusal is an error.
pub fn max_value(items: &[Item], capacity: usize) -> Result<u128, Error> {
    let width = capacity.checked_add(1).ok_or(Error::CapacityOverflow)?;
    let bytes = width
        .checked_mul(std::mem::size_of::<u128>())
        .ok_or(Error::CapacityOverflow)?;
    if bytes > isize::MAX as usize {
        return Err(Error::CapacityOverflow);
    }
    let mut best = Vec::new();
    best.try_reserve_exact(width)
        .map_err(|_| Error::AllocationFailed)?;
    best.resize(width, 0u128);
    for item in items {
        if item.weight > capacity {
            continue;
        }
        for c in (item.weight..=capacity).rev() {
            let candidate = best[c - item.weight] + u128::from(item.value);
            best[c] = best[c].max(candidate);
        }
    }
    Ok(best[capacity])
}

#[cfg(test)]
mod tests;
