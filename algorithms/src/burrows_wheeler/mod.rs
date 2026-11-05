//! Burrows–Wheeler byte transform with a canonical primary row.
//!
//! Original cyclic prefix-doubling implementation. Transform attribution:
//! M. Burrows and D. J. Wheeler, "A Block-sorting Lossless Data Compression
//! Algorithm", Digital Systems Research Center Research Report 124 (1994).

/// Invalid metadata or an unreachable canonical transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BwtError {
    /// Empty columns require zero; otherwise the primary row must be in bounds.
    InvalidPrimaryIndex,
    /// The column and row do not encode a canonical transform of any byte text.
    InvalidTransform,
}

fn shifted(i: usize, width: usize, n: usize) -> usize {
    if i >= n - width {
        i - (n - width)
    } else {
        i + width
    }
}

/// Return (last column, primary row), with no sentinel or alphabet restriction.
///
/// Rotations sort lexicographically, breaking equal periodic rotations by
/// ascending original offset. Empty input maps to ([], 0).
/// Comparison-sort cyclic doubling costs O(n log² n) time, O(n) space.
/// Allocation failure may panic/abort.
pub fn transform(text: &[u8]) -> (Vec<u8>, usize) {
    let n = text.len();
    if n == 0 {
        return (Vec::new(), 0);
    }
    let mut order: Vec<usize> = (0..n).collect();
    let mut ranks: Vec<usize> = text.iter().map(|&b| usize::from(b)).collect();
    let mut next = vec![0; n];
    let mut width = 1;
    while width < n {
        let key = |i: usize| (ranks[i], ranks[shifted(i, width, n)]);
        order.sort_unstable_by_key(|&i| key(i));
        next[order[0]] = 0;
        for pair in order.windows(2) {
            next[pair[1]] = next[pair[0]] + usize::from(key(pair[0]) != key(pair[1]));
        }
        std::mem::swap(&mut ranks, &mut next);
        width = width.saturating_mul(2);
    }
    order.sort_unstable_by_key(|&i| (ranks[i], i));
    let primary = order.iter().position(|&i| i == 0).unwrap();
    let column = order
        .into_iter()
        .map(|i| text[if i == 0 { n - 1 } else { i - 1 }])
        .collect();
    (column, primary)
}

/// Invert a canonical transform, checking index validity before reachability.
///
/// Stable occurrence ranks construct the LF permutation in O(n+256) time.
/// Retransformation rejects unreachable columns and noncanonical periodic primary
/// rows. Overall O(n log² n + 256) time and O(n+256) storage. In particular an
/// arbitrary in-range row is not necessarily accepted. Allocation may panic/abort.
pub fn inverse(column: &[u8], primary: usize) -> Result<Vec<u8>, BwtError> {
    let n = column.len();
    if (n == 0 && primary != 0) || (n != 0 && primary >= n) {
        return Err(BwtError::InvalidPrimaryIndex);
    }
    if n == 0 {
        return Ok(Vec::new());
    }
    let mut counts = [0usize; 256];
    for &b in column {
        counts[usize::from(b)] += 1;
    }
    let mut total = 0;
    for count in &mut counts {
        let frequency = *count;
        *count = total;
        total += frequency;
    }
    let mut lf = Vec::with_capacity(n);
    for &b in column {
        let count = &mut counts[usize::from(b)];
        lf.push(*count);
        *count += 1;
    }
    let mut text = vec![0; n];
    let mut row = primary;
    for byte in text.iter_mut().rev() {
        *byte = column[row];
        row = lf[row];
    }
    let (check, row) = transform(&text);
    if check != column || row != primary {
        return Err(BwtError::InvalidTransform);
    }
    Ok(text)
}

#[cfg(test)]
mod tests;
