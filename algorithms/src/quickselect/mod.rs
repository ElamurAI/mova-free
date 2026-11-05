//! Deterministic in-place order statistic selection.

/// Returns the element of zero-based sorted rank index.
/// Invalid indices return None without changing the input. On success every
/// preceding element is <= the selected element and every later element >= it.
/// Swaps preserve the multiset; relative order of equal elements is unspecified.
/// Uses a midpoint pivot and O(1) auxiliary space. Worst-case time is O(n²),
/// including equal-element inputs; no randomized expected-time claim is made.
/// Ord must define a consistent total order.
pub fn select_nth<T: Ord>(values: &mut [T], index: usize) -> Option<&T> {
    if index >= values.len() {
        return None;
    }
    let mut low = 0;
    let mut high = values.len();
    loop {
        if high - low == 1 {
            return Some(&values[index]);
        }
        let pivot = low + (high - low) / 2;
        let last = high - 1;
        values.swap(pivot, last);
        let mut boundary = low;
        for scan in low..last {
            if values[scan] < values[last] {
                values.swap(scan, boundary);
                boundary += 1;
            }
        }
        values.swap(boundary, last);
        if index == boundary {
            return Some(&values[index]);
        }
        if index < boundary {
            high = boundary;
        } else {
            low = boundary + 1;
        }
    }
}

#[cfg(test)]
mod tests;
