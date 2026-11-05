//! Rational trimmed-list subset sum, following CLRS, Introduction to Algorithms,
//! approximation algorithms / APPROX-SUBSET-SUM. Original implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnsupportedPercent,
    Overflow,
    WorkBudget,
    Allocation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Certificate {
    pub sum: u64,
    pub percent: u8,
    /// Input elements plus generated merge candidates; sorting is additional work.
    pub generated_states: usize,
}
/// Feasible sum >= (1-percent/100)*OPT. OPT=0 yields exact zero.
/// Trim uses delta=percent/(100*n), with exact integer comparisons.
/// Budget bounds cumulative candidates; sorting costs O(B log B) total.
/// Peak storage is O(B). Trim length O(n log(target)/epsilon).
pub fn max_sum(
    values: &[u64],
    target: u64,
    percent: u8,
    work_budget: usize,
) -> Result<Certificate, Error> {
    if !matches!(percent, 1 | 2 | 5 | 10) {
        return Err(Error::UnsupportedPercent);
    }
    let mut work = values.len();
    if work > work_budget {
        return Err(Error::WorkBudget);
    }
    let denominator = (values.len() as u128)
        .checked_mul(100)
        .ok_or(Error::Overflow)?;
    let mut list = Vec::new();
    list.try_reserve_exact(1).map_err(|_| Error::Allocation)?;
    list.push(0u64);
    for &x in values {
        if x == 0 || x > target {
            continue;
        }
        let cells = list.len().checked_mul(2).ok_or(Error::Overflow)?;
        work = work.checked_add(cells).ok_or(Error::Overflow)?;
        if work > work_budget {
            return Err(Error::WorkBudget);
        }
        let mut merged = Vec::new();
        merged
            .try_reserve_exact(cells)
            .map_err(|_| Error::Allocation)?;
        merged.extend_from_slice(&list);
        for &y in &list {
            if x <= target - y {
                merged.push(x + y);
            }
        }
        merged.sort_unstable();
        list.clear();
        list.try_reserve_exact(merged.len())
            .map_err(|_| Error::Allocation)?;
        list.push(0);
        let mut last = 0u64;
        for y in merged {
            let lhs = u128::from(y)
                .checked_mul(denominator)
                .ok_or(Error::Overflow)?;
            let rhs = u128::from(last)
                .checked_mul(denominator + u128::from(percent))
                .ok_or(Error::Overflow)?;
            if lhs > rhs {
                list.push(y);
                last = y;
            }
        }
    }
    Ok(Certificate {
        sum: *list.last().unwrap(),
        percent,
        generated_states: work,
    })
}
#[cfg(test)]
mod tests;
