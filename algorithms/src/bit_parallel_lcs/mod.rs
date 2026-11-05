//! Single-word bit-parallel longest common subsequence length over bytes.
//! Reference: L. Allison and T. I. Dix, "A Bit-String Longest-Common-Subsequence
//! Algorithm", Information Processing Letters 23 (1986), 305–310.
//! Original implementation of the bit-vector row-difference recurrence.

/// Both inputs exceed the supported 64-byte short side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortSideTooLong {
    /// Length of the shorter input.
    pub length: usize,
}
impl std::fmt::Display for ShortSideTooLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shorter input length {} exceeds 64 bytes", self.length)
    }
}
impl std::error::Error for ShortSideTooLong {}

/// Returns the exact byte LCS length if the shorter input fits in 64 bits.
/// Empty inputs return zero, even when the other input is arbitrarily long.
/// Time O(256+n+m), auxiliary space O(256) words, with no allocation.
/// Unsigned wrapping subtraction deliberately propagates borrow modulo 2^64;
/// the top bit and a full 64-bit matching state require no signed arithmetic.
pub fn length(a: &[u8], b: &[u8]) -> Result<usize, ShortSideTooLong> {
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if short.len() > 64 {
        return Err(ShortSideTooLong {
            length: short.len(),
        });
    }
    let mut masks = [0_u64; 256];
    for (bit, &byte) in short.iter().enumerate() {
        masks[usize::from(byte)] |= 1_u64 << bit;
    }
    let mut state = 0_u64;
    for &byte in long {
        let x = masks[usize::from(byte)] | state;
        let y = (state << 1) | 1;
        state = x & !x.wrapping_sub(y);
    }
    Ok(state.count_ones() as usize)
}

#[cfg(test)]
mod tests;
