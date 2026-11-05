//! Thresholded byte edit distance using a diagonal dynamic-programming band.
//! Reference: Esko Ukkonen, "Algorithms for approximate string matching",
//! Information and Control 64 (1985), 100–118. Original implementation.

/// Maximum number of cells in either rolling band; two such rows are stored.
pub const MAX_BAND_CELLS: usize = 4096;

/// The required band exceeds the explicit storage budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BandTooWide {
    /// Required cells per row.
    pub cells: usize,
}
impl std::fmt::Display for BandTooWide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "band requires {} cells; limit is {}",
            self.cells, MAX_BAND_CELLS
        )
    }
}
impl std::error::Error for BandTooWide {}

/// Exact unit-cost byte Levenshtein distance when at most `threshold`.
///
/// Returns `Ok(None)` otherwise. Length-difference pruning and empty inputs
/// need no band and precede budget validation. Thresholds including usize::MAX
/// are safe. Uses O(max(n,m) * min(min(n,m)+1, 2k+1)) time and
/// O(min(min(n,m)+1, 2k+1)) space, where k=min(threshold,max(n,m)).
/// At most two rows of MAX_BAND_CELLS are allocated; allocator failure follows
/// the standard allocator behavior. Inputs are bytes, not Unicode characters.
pub fn distance(a: &[u8], b: &[u8], threshold: usize) -> Result<Option<usize>, BandTooWide> {
    let (rows, columns) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let (n, m) = (rows.len(), columns.len());
    if n - m > threshold {
        return Ok(None);
    }
    if m == 0 {
        return Ok(Some(n));
    }
    let k = threshold.min(n);
    let width = m
        .saturating_add(1)
        .min(k.saturating_mul(2).saturating_add(1));
    if width > MAX_BAND_CELLS {
        return Err(BandTooWide { cells: width });
    }
    let infinity = k.saturating_add(1);
    let mut previous = vec![infinity; width];
    let mut current = vec![infinity; width];
    let mut previous_start = 0;
    let mut previous_end = m.min(k);
    for (j, cell) in previous.iter_mut().enumerate().take(previous_end + 1) {
        *cell = j;
    }
    for i in 1..=n {
        let start = i.saturating_sub(k);
        let end = m.min(i.saturating_add(k));
        current.fill(infinity);
        let lookup = |j: usize| {
            if j >= previous_start && j <= previous_end {
                previous[j - previous_start]
            } else {
                infinity
            }
        };
        for j in start..=end {
            let value = if j == 0 {
                i
            } else {
                let deletion = lookup(j).saturating_add(1);
                let insertion = if j > start {
                    current[j - start - 1].saturating_add(1)
                } else {
                    infinity
                };
                let substitution =
                    lookup(j - 1).saturating_add(usize::from(rows[i - 1] != columns[j - 1]));
                deletion.min(insertion).min(substitution).min(infinity)
            };
            current[j - start] = value;
        }
        std::mem::swap(&mut previous, &mut current);
        previous_start = start;
        previous_end = end;
    }
    let result = previous[m - previous_start];
    Ok((result <= threshold).then_some(result))
}

#[cfg(test)]
mod tests;
