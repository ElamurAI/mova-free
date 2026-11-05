//! Horowitz and Sahni (1974), Computing partitions with applications to the knapsack problem.
use crate::knapsack::Item;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DimensionOverflow,
    BudgetExceeded,
    AllocationFailed,
    ArithmeticOverflow,
}
fn states(items: &[Item], capacity: usize, count: usize) -> Result<Vec<(usize, u128)>, Error> {
    let mut out = Vec::new();
    out.try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    out.push((0usize, 0u128));
    for item in items {
        let len = out.len();
        for i in 0..len {
            let (w, v) = out[i];
            if item.weight <= capacity - w {
                out.push((
                    w + item.weight,
                    v.checked_add(item.value as u128)
                        .ok_or(Error::ArithmeticOverflow)?,
                ));
            }
        }
    }
    Ok(out)
}
/// Exact 0/1 objective using two half enumerations and a dominance envelope.
/// max_states bounds the sum of worst-case half counts, including empty subsets.
/// O(n*2^ceil(n/2)) time, O(2^ceil(n/2)) space; independent of capacity.
/// Dimension then budget checks precede allocation; weight sums never exceed capacity.
pub fn max_value(items: &[Item], capacity: usize, max_states: usize) -> Result<u128, Error> {
    let mid = items.len() / 2;
    let count = |n: usize| {
        1usize
            .checked_shl(u32::try_from(n).map_err(|_| Error::DimensionOverflow)?)
            .ok_or(Error::DimensionOverflow)
    };
    let (a, b) = (count(mid)?, count(items.len() - mid)?);
    let total = a.checked_add(b).ok_or(Error::DimensionOverflow)?;
    if total > isize::MAX as usize / std::mem::size_of::<(usize, u128)>() {
        return Err(Error::DimensionOverflow);
    }
    if total > max_states {
        return Err(Error::BudgetExceeded);
    }
    let left = states(&items[..mid], capacity, a)?;
    let mut right = states(&items[mid..], capacity, b)?;
    right.sort_unstable();
    for i in 1..right.len() {
        right[i].1 = right[i].1.max(right[i - 1].1);
    }
    let mut best = 0;
    for (w, v) in left {
        let end = right.partition_point(|&(rw, _)| rw <= capacity - w);
        best = best.max(
            v.checked_add(right[end - 1].1)
                .ok_or(Error::ArithmeticOverflow)?,
        );
    }
    Ok(best)
}
#[cfg(test)]
mod tests;
