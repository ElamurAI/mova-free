//! Unrestricted unit-cost byte edit distance, including adjacent transposition.
//! Original implementation of the last-occurrence recurrence described by
//! Lowrance and Wagner, "An Extension of the String-to-String Correction Problem"
//! (Journal of the ACM, 1975), following Damerau's 1964 edit operations.

/// Maximum allocated matrix cells, including sentinel rows and columns.
pub const MAX_CELLS: usize = 1_000_000;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    DimensionOverflow,
    CellLimit,
}
/// Returns the unrestricted distance (unlike optimal string alignment).
/// Empty inputs return immediately. Otherwise dimension overflow precedes the
/// cell limit. O((n+2)(m+2)) time/space; allocation failure follows Vec behavior.
pub fn distance(a: &[u8], b: &[u8]) -> Result<usize, Error> {
    if a.is_empty() || b.is_empty() {
        return Ok(a.len().max(b.len()));
    }
    let rows = a.len().checked_add(2).ok_or(Error::DimensionOverflow)?;
    let cols = b.len().checked_add(2).ok_or(Error::DimensionOverflow)?;
    let cells = rows.checked_mul(cols).ok_or(Error::DimensionOverflow)?;
    let sentinel = a
        .len()
        .checked_add(b.len())
        .ok_or(Error::DimensionOverflow)?;
    if cells > MAX_CELLS {
        return Err(Error::CellLimit);
    }
    let mut d = vec![0usize; cells];
    d[0] = sentinel;
    for i in 0..=a.len() {
        d[(i + 1) * cols] = sentinel;
        d[(i + 1) * cols + 1] = i;
    }
    for j in 0..=b.len() {
        d[j + 1] = sentinel;
        d[cols + j + 1] = j;
    }
    let mut last = [0usize; 256];
    for i in 1..=a.len() {
        let mut matched_column = 0;
        for j in 1..=b.len() {
            let row = last[b[j - 1] as usize];
            let column = matched_column;
            let cost = usize::from(a[i - 1] != b[j - 1]);
            if cost == 0 {
                matched_column = j;
            }
            d[(i + 1) * cols + j + 1] = (d[i * cols + j] + cost)
                .min(d[i * cols + j + 1] + 1)
                .min(d[(i + 1) * cols + j] + 1)
                .min(d[row * cols + column] + (i - row - 1) + 1 + (j - column - 1));
        }
        last[a[i - 1] as usize] = i;
    }
    Ok(d[(rows - 1) * cols + cols - 1])
}
#[cfg(test)]
mod tests;
