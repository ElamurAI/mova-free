//! Exact inversion count through merge sorting indices; original implementation.
//! Merge-count method: Cormen, Leiserson, Rivest and Stein, Introduction to
//! Algorithms, 3rd ed. (2009), problem 2-4 (Inversions).
pub const MAX_LEN: usize = 1 << 20;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooLong,
    AllocationFailed,
}
fn merge_count<T: Ord>(values: &[T], indices: &mut [usize], scratch: &mut [usize]) -> u128 {
    if indices.len() < 2 {
        return 0;
    }
    let mid = indices.len() / 2;
    let (left, right) = indices.split_at_mut(mid);
    let (a, b) = scratch.split_at_mut(mid);
    let mut total = merge_count(values, left, a) + merge_count(values, right, b);
    let (mut i, mut j) = (0, 0);
    for slot in scratch.iter_mut() {
        if j == right.len() || (i < left.len() && values[left[i]] <= values[right[j]]) {
            *slot = left[i];
            i += 1;
        } else {
            *slot = right[j];
            j += 1;
            total += (left.len() - i) as u128;
        }
    }
    indices.copy_from_slice(scratch);
    total
}
/// Count i<j with values[i]>values[j]. Ties are not inversions.
/// Preserves input and requires lawful Ord, without requiring Clone.
/// O(n log n) comparisons, O(n) indices, O(log n) stack. Length validation
/// precedes allocation; n<=2^20 makes the pair total fit u128 exactly.
pub fn count_inversions<T: Ord>(values: &[T]) -> Result<u128, Error> {
    let n = values.len();
    if n > MAX_LEN {
        return Err(Error::TooLong);
    }
    let mut indices = Vec::new();
    indices
        .try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    indices.extend(0..n);
    let mut scratch = Vec::new();
    scratch
        .try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    scratch.resize(n, 0);
    Ok(merge_count(values, &mut indices, &mut scratch))
}
#[cfg(test)]
#[path = "tests.rs"]
mod wave14_models;
