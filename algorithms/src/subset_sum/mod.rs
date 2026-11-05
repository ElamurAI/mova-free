//! Exact nonnegative subset sum with an index witness.
//! Original implementation of the subset dynamic program; see Bellman,
//! Dynamic Programming (1957), for the general principle of optimality.

/// Maximum target accepted before allocation, including empty inputs.
pub const MAX_TARGET: usize = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TargetTooLarge,
    AllocationFailed,
}

/// Returns increasing, distinct input indices summing to `target`, or `None`.
/// Zero target has the empty witness. Zero-valued items are never selected.
/// Ties retain the first discovery while processing indices in input order
/// and sums in descending order; no minimum-cardinality promise is made.
/// O(values.len() * (target + 1)) time, O(target + 1) auxiliary space.
pub fn subset_sum(values: &[usize], target: usize) -> Result<Option<Vec<usize>>, Error> {
    if target > MAX_TARGET {
        return Err(Error::TargetTooLarge);
    }
    if target == 0 {
        return Ok(Some(Vec::new()));
    }
    let mut parent = Vec::new();
    parent
        .try_reserve_exact(target + 1)
        .map_err(|_| Error::AllocationFailed)?;
    parent.resize(target + 1, None);
    for (index, &value) in values.iter().enumerate() {
        if value == 0 || value > target {
            continue;
        }
        for sum in (value..=target).rev() {
            if parent[sum].is_none() && (sum == value || parent[sum - value].is_some()) {
                parent[sum] = Some(index);
            }
        }
        if parent[target].is_some() {
            break;
        }
    }
    if parent[target].is_none() {
        return Ok(None);
    }
    let mut witness = Vec::new();
    witness
        .try_reserve_exact(values.len().min(target))
        .map_err(|_| Error::AllocationFailed)?;
    let mut sum = target;
    while sum != 0 {
        let index = parent[sum].expect("reachable positive sum has a predecessor");
        witness.push(index);
        sum -= values[index];
    }
    witness.reverse();
    Ok(Some(witness))
}

#[cfg(test)]
mod tests;
