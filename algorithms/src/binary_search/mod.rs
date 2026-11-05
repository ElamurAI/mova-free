//! Repository-owned binary search over sorted slices.

/// Returns the first index whose value is at least `target`, or `values.len()`.
///
/// Requires a nondecreasing slice and a lawful total order. Does not validate sorting.
/// Uses O(log(n + 1)) comparisons and O(1) auxiliary space.
pub fn lower_bound<T: Ord>(values: &[T], target: &T) -> usize {
    let (mut lo, mut hi) = (0, values.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if values[mid] < *target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Returns the first equal element's index, or `None`.
/// Has the same sorted-input precondition as `lower_bound`.
pub fn binary_search_first<T: Ord>(values: &[T], target: &T) -> Option<usize> {
    let index = lower_bound(values, target);
    (index < values.len() && values[index] == *target).then_some(index)
}

#[cfg(test)]
mod tests;
