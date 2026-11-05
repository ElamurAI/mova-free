//! Union of validated i64 half-open intervals, in canonical increasing order.
//! Original sorted sweep; general sweep-line background: de Berg et al.,
//! Computational Geometry, 3rd edition (2008), chapter 2.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ReversedInterval { index: usize },
    AllocationFailed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Union {
    pub intervals: Vec<(i64, i64)>,
    pub covered_length: u128,
}

/// Validates every interval before allocating. Empty intervals disappear;
/// overlapping or touching intervals merge. Length uses widened subtraction.
/// O(n log(n+1)) time and O(n) space; input order is not changed.
pub fn interval_union(intervals: &[(i64, i64)]) -> Result<Union, Error> {
    for (index, &(start, end)) in intervals.iter().enumerate() {
        if start > end {
            return Err(Error::ReversedInterval { index });
        }
    }
    let mut merged = Vec::new();
    merged
        .try_reserve_exact(intervals.len())
        .map_err(|_| Error::AllocationFailed)?;
    merged.extend(
        intervals
            .iter()
            .copied()
            .filter(|&(start, end)| start != end),
    );
    merged.sort_unstable();
    let mut count = 0;
    for read in 0..merged.len() {
        let (start, end) = merged[read];
        if count != 0 && start <= merged[count - 1].1 {
            merged[count - 1].1 = merged[count - 1].1.max(end);
        } else {
            merged[count] = (start, end);
            count += 1;
        }
    }
    merged.truncate(count);
    let covered_length = merged
        .iter()
        .map(|&(start, end)| (i128::from(end) - i128::from(start)) as u128)
        .sum();
    Ok(Union {
        intervals: merged,
        covered_length,
    })
}

#[cfg(test)]
mod tests;
