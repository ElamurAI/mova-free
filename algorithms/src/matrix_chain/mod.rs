//! Minimum scalar multiplication cost of a matrix chain.
//! Original interval dynamic program; dimensions describe adjacent matrices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ZeroDimension,
    DimensionOverflow,
    AllocationFailed,
    CostOverflow,
}
/// Empty and one-dimension inputs describe an empty chain and cost zero.
/// Every zero dimension is rejected, including in short inputs.
/// O(n^3) time and O(n^2) fallibly allocated space for n matrices.
/// Overflowing splits are ignored: positive costs cannot later decrease.
/// CostOverflow means every full parenthesization exceeds u128::MAX.
pub fn minimum_cost(dimensions: &[u64]) -> Result<u128, Error> {
    if dimensions.contains(&0) {
        return Err(Error::ZeroDimension);
    }
    let n = dimensions.len().saturating_sub(1);
    if n <= 1 {
        return Ok(0);
    }
    let cells = n.checked_mul(n).ok_or(Error::DimensionOverflow)?;
    let bytes = cells
        .checked_mul(std::mem::size_of::<Option<u128>>())
        .ok_or(Error::DimensionOverflow)?;
    if bytes > isize::MAX as usize {
        return Err(Error::DimensionOverflow);
    }
    let mut best: Vec<Option<u128>> = Vec::new();
    best.try_reserve_exact(cells)
        .map_err(|_| Error::AllocationFailed)?;
    best.resize(cells, None);
    for i in 0..n {
        best[i * n + i] = Some(0);
    }
    for length in 2..=n {
        for i in 0..=n - length {
            let j = i + length - 1;
            for k in i..j {
                let candidate = (|| {
                    let product = u128::from(dimensions[i])
                        .checked_mul(u128::from(dimensions[k + 1]))?
                        .checked_mul(u128::from(dimensions[j + 1]))?;
                    best[i * n + k]?
                        .checked_add(best[(k + 1) * n + j]?)?
                        .checked_add(product)
                })();
                if let Some(cost) = candidate
                    && best[i * n + j].is_none_or(|old| cost < old)
                {
                    best[i * n + j] = Some(cost);
                }
            }
        }
    }
    best[n - 1].ok_or(Error::CostOverflow)
}
#[cfg(test)]
mod tests;
