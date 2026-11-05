//! Chen-Fox-Lyndon factorization using Duval's linear scan.
//! J.-P. Duval, "Factorizing words over an ordered alphabet" (1983).
use std::ops::Range;

/// Unique nonincreasing sequence of nonempty Lyndon factors, as byte ranges.
/// Bytes use unsigned lexicographic order; no Unicode normalization occurs.
/// O(n) byte comparisons, O(k) output storage, O(1) auxiliary storage.
/// Empty input has no factors. Allocation failure follows Vec behavior.
pub fn factorize(bytes: &[u8]) -> Vec<Range<usize>> {
    let mut factors = Vec::new();
    let mut start = 0;
    while start < bytes.len() {
        let mut scan = start + 1;
        let mut matched = start;
        while scan < bytes.len() && bytes[matched] <= bytes[scan] {
            matched = if bytes[matched] < bytes[scan] {
                start
            } else {
                matched + 1
            };
            scan += 1;
        }
        let period = scan - matched;
        while start <= matched {
            factors.push(start..start + period);
            start += period;
        }
    }
    factors
}
#[cfg(test)]
mod tests;
