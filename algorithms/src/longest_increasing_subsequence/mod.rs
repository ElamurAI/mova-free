//! Reconstruct a strictly increasing subsequence through minimum tails.

/// Returns indices of one maximum-length strictly increasing subsequence.
/// Indices and their referenced values both increase strictly.
/// For each length, retain the minimum tail value; replace equal tails with
/// the latest index. Predecessors use the retained shorter tail at insertion.
/// This deterministic policy does not promise lexicographically earliest indices.
/// Time O(n log n), auxiliary space O(n). No Clone bound is required.
/// Ord must define a consistent total order.
pub fn indices<T: Ord>(values: &[T]) -> Vec<usize> {
    let mut tails: Vec<usize> = Vec::new();
    let mut previous = vec![None; values.len()];
    for (i, value) in values.iter().enumerate() {
        let slot = tails.partition_point(|&j| values[j] < *value);
        if slot > 0 {
            previous[i] = Some(tails[slot - 1]);
        }
        if slot == tails.len() {
            tails.push(i);
        } else {
            tails[slot] = i;
        }
    }
    let mut result = Vec::with_capacity(tails.len());
    let mut cursor = tails.last().copied();
    while let Some(i) = cursor {
        result.push(i);
        cursor = previous[i];
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests;
