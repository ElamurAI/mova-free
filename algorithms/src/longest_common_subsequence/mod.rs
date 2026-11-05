//! Classical longest-common-subsequence length with one rolling row.
//! Original implementation; does not reconstruct a witness sequence.

/// Maximum length of a common subsequence.
/// O(a.len()*b.len()) equality calls and O(min(a.len(),b.len())) words.
/// Eq must satisfy its contract. Empty inputs return immediately.
/// Row dimension overflow panics explicitly; allocation follows Vec panic/abort
/// behavior, and a caller-defined Eq implementation may panic.
pub fn length<T: Eq>(a: &[T], b: &[T]) -> usize {
    let (rows, columns) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    if columns.is_empty() {
        return 0;
    }
    let width = columns
        .len()
        .checked_add(1)
        .expect("LCS row dimension overflow");
    let mut row = vec![0usize; width];
    for x in rows {
        let mut diagonal = 0;
        for (j, y) in columns.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = if x == y {
                diagonal + 1
            } else {
                above.max(row[j])
            };
            diagonal = above;
        }
    }
    row[columns.len()]
}

#[cfg(test)]
mod tests;
