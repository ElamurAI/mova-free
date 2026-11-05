//! Single-word Shift-And matching over bytes.
//!
//! Classical source: Ricardo Baeza-Yates and Gaston H. Gonnet,
//! "A New Approach to Text Searching", Communications of the ACM 35(10),
//! 1992, pp. 74–82. This implementation is original.

/// A pattern exceeds the single 64-bit state word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternTooLong {
    /// Rejected pattern length in bytes; the supported maximum is 64.
    pub length: usize,
}

impl std::fmt::Display for PatternTooLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pattern length {} exceeds 64 bytes", self.length)
    }
}

impl std::error::Error for PatternTooLong {}

/// Returns every exact match's byte offset, including overlaps.
/// Empty patterns match every boundary, including the final boundary.
/// Patterns longer than 64 bytes are rejected before scanning the text.
/// Costs O(256 + pattern.len() + text.len() + output.len()) time and
/// O(256) auxiliary words, excluding the returned vector. Allocation failure
/// follows the standard allocator behavior; there is no arbitrary text cap.
pub fn find_all(text: &[u8], pattern: &[u8]) -> Result<Vec<usize>, PatternTooLong> {
    if pattern.len() > 64 {
        return Err(PatternTooLong {
            length: pattern.len(),
        });
    }
    if pattern.is_empty() {
        return Ok((0..=text.len()).collect());
    }
    let mut masks = [0_u64; 256];
    for (position, &byte) in pattern.iter().enumerate() {
        masks[usize::from(byte)] |= 1_u64 << position;
    }
    let accepting = 1_u64 << (pattern.len() - 1);
    let mut state = 0_u64;
    let mut matches = Vec::new();
    for (position, &byte) in text.iter().enumerate() {
        state = ((state << 1) | 1) & masks[usize::from(byte)];
        if state & accepting != 0 {
            matches.push(position + 1 - pattern.len());
        }
    }
    Ok(matches)
}

#[cfg(test)]
mod tests;
