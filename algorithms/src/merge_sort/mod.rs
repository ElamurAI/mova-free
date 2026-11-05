//! Repository-owned stable merge sort from the divide-and-merge idea.

/// Stably sorts values in ascending order.
///
/// Requires a lawful total `Ord` and value-preserving `Clone`. Uses O(n)
/// auxiliary elements, O(log(n + 1)) stack depth and O(n log(n + 1))
/// comparison/clone work. Allocation or user comparison/clone failures are
/// outside the successful-return contract.
pub fn merge_sort<T: Ord + Clone>(values: &mut [T]) {
    if values.len() < 2 {
        return;
    }
    let mut scratch = values.to_vec();
    sort_with_scratch(values, &mut scratch);
}

fn sort_with_scratch<T: Ord + Clone>(values: &mut [T], scratch: &mut [T]) {
    if values.len() < 2 {
        return;
    }
    let mid = values.len() / 2;
    let (left, right) = values.split_at_mut(mid);
    let (scratch_left, scratch_right) = scratch.split_at_mut(mid);
    sort_with_scratch(left, scratch_left);
    sort_with_scratch(right, scratch_right);
    let (mut i, mut j) = (0, mid);
    for slot in scratch.iter_mut() {
        if i < mid && (j == values.len() || values[i] <= values[j]) {
            *slot = values[i].clone();
            i += 1;
        } else {
            *slot = values[j].clone();
            j += 1;
        }
    }
    values.clone_from_slice(scratch);
}

#[cfg(test)]
mod tests;
