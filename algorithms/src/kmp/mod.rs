//! Exact byte matching with Knuth-Morris-Pratt prefix fallback.

/// Returns all matching byte offsets, including overlapping matches.
/// An empty needle matches every boundary, including the final boundary.
/// Takes O(text.len() + needle.len()) time and O(needle.len()) auxiliary
/// space, in addition to the returned offsets.
pub fn find_all(text: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() {
        return (0..=text.len()).collect();
    }
    let mut prefix = vec![0; needle.len()];
    let mut matched = 0;
    for i in 1..needle.len() {
        while matched > 0 && needle[i] != needle[matched] {
            matched = prefix[matched - 1];
        }
        if needle[i] == needle[matched] {
            matched += 1;
        }
        prefix[i] = matched;
    }
    let mut result = Vec::new();
    matched = 0;
    for (i, &byte) in text.iter().enumerate() {
        while matched > 0 && byte != needle[matched] {
            matched = prefix[matched - 1];
        }
        if byte == needle[matched] {
            matched += 1;
        }
        if matched == needle.len() {
            result.push(i + 1 - needle.len());
            matched = prefix[matched - 1];
        }
    }
    result
}

#[cfg(test)]
mod tests;
