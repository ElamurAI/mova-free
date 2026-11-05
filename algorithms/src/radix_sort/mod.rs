//! Exact stable least-significant-digit sorting of unsigned 64-bit values.

use std::collections::TryReserveError;

/// Sort ascending using eight stable byte passes, independent of host endianness.
///
/// Equal values retain their order in each pass. The scalar API does not expose
/// attached records. Allocation failure leaves the slice unchanged.
/// Time O(8(n + 256)); auxiliary space O(n + 256).
pub fn radix_sort(values: &mut [u64]) -> Result<(), TryReserveError> {
    if values.len() < 2 {
        return Ok(());
    }
    let mut scratch = Vec::new();
    scratch.try_reserve_exact(values.len())?;
    scratch.resize(values.len(), 0);
    for byte in 0..8 {
        byte_pass(values, &mut scratch, byte);
        values.copy_from_slice(&scratch);
    }
    Ok(())
}

// Traversing source left-to-right with increasing bucket cursors makes a pass
// stable: lower bytes already sorted by previous passes keep their order.
fn byte_pass(source: &[u64], destination: &mut [u64], byte: usize) {
    let mut next = [0usize; 256];
    for &value in source {
        next[value.to_le_bytes()[byte] as usize] += 1;
    }
    let mut prefix = 0;
    for count in &mut next {
        let frequency = *count;
        *count = prefix;
        prefix += frequency;
    }
    for &value in source {
        let digit = value.to_le_bytes()[byte] as usize;
        destination[next[digit]] = value;
        next[digit] += 1;
    }
}

#[cfg(test)]
mod tests;
