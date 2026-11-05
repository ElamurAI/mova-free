//! Lexicographic suffix order for arbitrary bytes.
//!
//! Original prefix-doubling implementation inspired by U. Manber and G. Myers,
//! "Suffix Arrays: A New Method for On-Line String Searches", SIAM Journal on
//! Computing 22(5), 935–948 (1993), doi:10.1137/0222058.
//! This comparison-sort variant takes O(n log² n), not their optimized bound.

/// Return each byte offset once, ordered by its nonempty suffix.
///
/// The empty suffix is excluded. Bytes use unsigned order; UTF-8 is not decoded.
/// O(n log² n) time and O(n) auxiliary space. Allocation failure may panic/abort.
pub fn suffix_array(text: &[u8]) -> Vec<usize> {
    let n = text.len();
    let mut order: Vec<usize> = (0..n).collect();
    if n < 2 {
        return order;
    }
    let mut ranks: Vec<usize> = text.iter().map(|&byte| usize::from(byte)).collect();
    let mut next = vec![0; n];
    let mut width = 1;
    loop {
        // None is the end sentinel, smaller than every actual rank.
        let key = |i: usize| {
            (
                ranks[i],
                i.checked_add(width).and_then(|j| ranks.get(j)).copied(),
            )
        };
        order.sort_unstable_by_key(|&i| key(i));
        next[order[0]] = 0;
        for pair in order.windows(2) {
            next[pair[1]] = next[pair[0]] + usize::from(key(pair[0]) != key(pair[1]));
        }
        std::mem::swap(&mut ranks, &mut next);
        if ranks[order[n - 1]] == n - 1 {
            return order;
        }
        // Once width covers n, every distinct suffix has a distinct key.
        width = width.saturating_mul(2);
    }
}

#[cfg(test)]
mod tests;
