//! In-place comparison sorting by a max heap.

/// Sorts ascending according to Ord, without allocating or requiring Clone.
/// This sort is unstable. Worst-case time O(n log n), auxiliary space O(1).
/// Ord must define a consistent total order.
pub fn heap_sort<T: Ord>(values: &mut [T]) {
    let n = values.len();
    for root in (0..n / 2).rev() {
        sift_down(values, root, n);
    }
    for end in (1..n).rev() {
        values.swap(0, end);
        sift_down(values, 0, end);
    }
}

fn sift_down<T: Ord>(values: &mut [T], mut root: usize, end: usize) {
    // root < end/2 implies that 2*root+1 is a valid, nonoverflowing child.
    while root < end / 2 {
        let mut child = root * 2 + 1;
        if child + 1 < end && values[child] < values[child + 1] {
            child += 1;
        }
        if values[root] >= values[child] {
            break;
        }
        values.swap(root, child);
        root = child;
    }
}

#[cfg(test)]
mod tests;
