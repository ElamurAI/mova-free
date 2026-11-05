//! Classical weighted interval scheduling by compatible-prefix dynamic programming.
//! Original implementation for positive-duration half-open intervals.

/// Half-open interval [start,end), carrying nonnegative weight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    pub start: i64,
    pub end: i64,
    pub weight: u64,
}

/// Optimal witness in chronological order, using original input indices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule {
    pub indices: Vec<usize>,
    pub total_weight: u128,
}

/// Explicit validation/arithmetic errors; allocation follows Vec semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// First invalid interval in original input order.
    InvalidInterval { index: usize },
    /// Checked prefix addition exceeds u128.
    WeightOverflow,
    /// Input length plus one exceeds usize.
    DimensionOverflow,
}

/// Maximum-weight compatible subset; adjacency is compatible.
/// Order is (end,start,original index); equal take/skip scores skip the current
/// item, including zero weights. This does not minimize original-index order.
/// O(n log n) time and O(n) space. Input remains unchanged.
/// Allocation failure may panic or abort rather than returning an Error.
pub fn select(items: &[Item]) -> Result<Schedule, Error> {
    for (index, item) in items.iter().enumerate() {
        if item.start >= item.end {
            return Err(Error::InvalidInterval { index });
        }
    }
    let width = items.len().checked_add(1).ok_or(Error::DimensionOverflow)?;
    let mut order: Vec<usize> = (0..items.len()).collect();
    order.sort_unstable_by_key(|&i| (items[i].end, items[i].start, i));
    let mut best = vec![0u128; width];
    let mut previous = vec![0usize; items.len()];
    let mut take = vec![false; items.len()];
    for (j, &index) in order.iter().enumerate() {
        let item = items[index];
        let compatible = order[..j].partition_point(|&i| items[i].end <= item.start);
        previous[j] = compatible;
        let candidate = best[compatible]
            .checked_add(u128::from(item.weight))
            .ok_or(Error::WeightOverflow)?;
        take[j] = candidate > best[j];
        best[j + 1] = best[j].max(candidate);
    }
    let mut indices = Vec::new();
    let mut prefix = items.len();
    while prefix > 0 {
        let j = prefix - 1;
        if take[j] {
            indices.push(order[j]);
            prefix = previous[j];
        } else {
            prefix = j;
        }
    }
    indices.reverse();
    Ok(Schedule {
        indices,
        total_weight: best[items.len()],
    })
}

#[cfg(test)]
mod tests;
