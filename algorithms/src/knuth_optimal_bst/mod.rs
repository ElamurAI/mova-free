//! Knuth (1971), Optimum binary search trees. Successful searches only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DimensionOverflow,
    BudgetExceeded,
    AllocationFailed,
    ArithmeticOverflow,
}
/// Minimum sum weight[i]*depth[i] over BSTs in fixed key order, root depth one.
/// Nonnegative u64 weights guarantee Knuth's monotone roots; ties choose leftmost.
/// O(n²) time/space; max_cells limits EACH square table, not their sum.
/// Checks dimension, budget, allocation, then checked u128 arithmetic in order.
/// Empty input costs zero without allocating. No unsuccessful-search frequencies.
pub fn min_cost(weights: &[u64], max_cells: usize) -> Result<u128, Error> {
    let n = weights.len();
    if n == 0 {
        return Ok(0);
    }
    let width = n.checked_add(1).ok_or(Error::DimensionOverflow)?;
    let cells = width.checked_mul(width).ok_or(Error::DimensionOverflow)?;
    if cells > isize::MAX as usize / std::mem::size_of::<u128>() {
        return Err(Error::DimensionOverflow);
    }
    if cells > max_cells {
        return Err(Error::BudgetExceeded);
    }
    let mut dp = Vec::new();
    dp.try_reserve_exact(cells)
        .map_err(|_| Error::AllocationFailed)?;
    dp.resize(cells, 0u128);
    let mut root = Vec::new();
    root.try_reserve_exact(cells)
        .map_err(|_| Error::AllocationFailed)?;
    root.resize(cells, 0usize);
    let mut prefix = Vec::new();
    prefix
        .try_reserve_exact(width)
        .map_err(|_| Error::AllocationFailed)?;
    prefix.push(0u128);
    for &w in weights {
        prefix.push(
            prefix
                .last()
                .unwrap()
                .checked_add(w as u128)
                .ok_or(Error::ArithmeticOverflow)?,
        );
    }
    for i in 0..n {
        root[i * width + i + 1] = i;
        dp[i * width + i + 1] = weights[i] as u128;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len;
            let mut best = None;
            let mut winner = i;
            let lo = root[i * width + j - 1];
            let hi = root[(i + 1) * width + j].min(j - 1);
            for r in lo..=hi {
                let value = dp[i * width + r]
                    .checked_add(dp[(r + 1) * width + j])
                    .and_then(|v| v.checked_add(prefix[j] - prefix[i]))
                    .ok_or(Error::ArithmeticOverflow)?;
                if best.is_none_or(|b| value < b) {
                    best = Some(value);
                    winner = r;
                }
            }
            dp[i * width + j] = best.unwrap();
            root[i * width + j] = winner;
        }
    }
    Ok(dp[n])
}
#[cfg(test)]
mod tests;
