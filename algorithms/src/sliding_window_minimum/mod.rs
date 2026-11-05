//! Exact fixed-width sliding minima using a monotone deque.
use std::collections::VecDeque;
/// Returns the leftmost minimum's source index for each complete window.
/// Zero or oversized widths return an empty vector, including on empty input.
/// T must obey Ord laws. O(n) comparisons, O(window) auxiliary space and
/// O(n-window+1) output space for valid widths; does not clone or mutate values.
pub fn minimum_indices<T: Ord>(values: &[T], window: usize) -> Vec<usize> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }
    let mut candidates: VecDeque<usize> = VecDeque::new();
    let mut result = Vec::with_capacity(values.len() - window + 1);
    for (index, value) in values.iter().enumerate() {
        if index >= window {
            while candidates.front().is_some_and(|&i| i <= index - window) {
                candidates.pop_front();
            }
        }
        while candidates.back().is_some_and(|&i| values[i] > *value) {
            candidates.pop_back();
        }
        candidates.push_back(index);
        if index >= window - 1 {
            result.push(*candidates.front().expect("current index is a candidate"));
        }
    }
    result
}
#[cfg(test)]
mod tests;
