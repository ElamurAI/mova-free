//! Word-parallel 0/1 subset sum. See Bellman, Dynamic Programming (1957).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DimensionOverflow,
    BudgetExceeded,
    AllocationFailed,
}
/// Exact reachability. Zero values are harmless; words beyond target are discarded.
/// O(values.len() * (target/64+1)) time and O(target/64+1) words.
/// Dimensions, budget, then fallible allocation are checked in that order.
pub fn reachable(values: &[usize], target: usize, max_words: usize) -> Result<bool, Error> {
    let words = target / 64 + 1;
    if words > isize::MAX as usize / 8 {
        return Err(Error::DimensionOverflow);
    }
    if words > max_words {
        return Err(Error::BudgetExceeded);
    }
    let mut bits = Vec::new();
    bits.try_reserve_exact(words)
        .map_err(|_| Error::AllocationFailed)?;
    bits.resize(words, 0u64);
    bits[0] = 1;
    for &value in values {
        if value > target {
            continue;
        }
        let (whole, shift) = (value / 64, value % 64);
        for dst in (whole..words).rev() {
            let mut moved = bits[dst - whole] << shift;
            if shift != 0 && dst > whole {
                moved |= bits[dst - whole - 1] >> (64 - shift);
            }
            bits[dst] |= moved;
        }
    }
    Ok(bits[target / 64] & (1u64 << (target % 64)) != 0)
}
#[cfg(test)]
mod tests;
