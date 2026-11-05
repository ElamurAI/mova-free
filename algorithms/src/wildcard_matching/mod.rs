//! Exact full-string byte wildcard matching by a dynamic-programming frontier.
//!
//! Uses the classical dynamic-programming principle (R. Bellman, Dynamic
//! Programming, Princeton University Press, 1957); this recurrence and code
//! are original. This is not a regular-expression or filesystem glob engine.

/// Match all text bytes against all pattern bytes.
///
/// '?' consumes exactly one byte; '*' consumes any number, including zero.
/// Every other byte, including backslash, is literal: there is no escape syntax.
/// UTF-8 is not decoded. O((n+1)(m+1)) time and O(m+1) storage for text length n
/// and pattern length m. Allocation failure may panic/abort.
pub fn wildcard_match(text: &[u8], pattern: &[u8]) -> bool {
    let mut row = vec![false; pattern.len() + 1];
    row[0] = true;
    for (j, &token) in pattern.iter().enumerate() {
        row[j + 1] = row[j] && token == b'*';
    }
    for &byte in text {
        let mut diagonal = row[0];
        row[0] = false;
        for (j, &token) in pattern.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = if token == b'*' {
                row[j] || above
            } else {
                diagonal && (token == b'?' || token == byte)
            };
            diagonal = above;
        }
    }
    row[pattern.len()]
}

#[cfg(test)]
mod tests;
