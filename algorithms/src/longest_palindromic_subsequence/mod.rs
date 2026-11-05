//! Longest palindromic subsequence via direct interval dynamic programming.
/// Maximum length of a subsequence equal to its reversal. Empty gives zero.
/// O(n²) equality calls and O(n) auxiliary words; equality cost is additional.
pub fn length<T: Eq>(input: &[T]) -> usize {
    let n = input.len();
    let mut row = vec![0; n];
    for left in (0..n).rev() {
        let mut diagonal = 0;
        row[left] = 1;
        for right in left + 1..n {
            let below = row[right];
            row[right] = if input[left] == input[right] {
                diagonal + 2
            } else {
                row[right].max(row[right - 1])
            };
            diagonal = below;
        }
    }
    row.last().copied().unwrap_or(0)
}
#[cfg(test)]
mod tests;
