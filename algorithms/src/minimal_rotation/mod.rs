//! Linear minimum cyclic rotation by two-candidate prefix elimination.
//!
//! Classical source: Kellogg S. Booth, "Lexicographically least circular
//! substrings", Information Processing Letters 10(4–5), 1980, pp. 240–242.
//! This original Booth-style elimination implementation does not use a doubled
//! input or the failure-table variant's early-return shortcut.

/// Returns the byte index of the lexicographically smallest cyclic rotation.
/// Equal rotations select the smallest index; empty input returns zero.
/// Byte order is unsigned, with no Unicode normalization or character indexing.
/// Costs O(n) time and O(1) auxiliary space, with no allocations.
pub fn minimal_rotation(bytes: &[u8]) -> usize {
    let n = bytes.len();
    if n < 2 {
        return 0;
    }
    let (mut left, mut right, mut matched) = (0, 1, 0);
    while left < n && right < n && matched < n {
        // Modular addition without constructing a potentially overflowing sum.
        let a = bytes[cyclic_offset(left, matched, n)];
        let b = bytes[cyclic_offset(right, matched, n)];
        match a.cmp(&b) {
            std::cmp::Ordering::Equal => matched += 1,
            std::cmp::Ordering::Greater => {
                left = advance(left, matched, n);
                if left == right {
                    left += 1;
                }
                matched = 0;
            }
            std::cmp::Ordering::Less => {
                right = advance(right, matched, n);
                if right == left {
                    right += 1;
                }
                matched = 0;
            }
        }
    }
    left.min(right)
}

fn cyclic_offset(start: usize, offset: usize, n: usize) -> usize {
    if offset >= n - start {
        offset - (n - start)
    } else {
        start + offset
    }
}

fn advance(start: usize, matched: usize, n: usize) -> usize {
    // Any candidate at or beyond n is an equivalent exhausted sentinel.
    start.saturating_add(matched).saturating_add(1).min(n)
}

#[cfg(test)]
mod tests;
