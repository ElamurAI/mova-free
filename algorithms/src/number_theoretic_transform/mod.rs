//! Exact polynomial convolution over the prime field with 998244353 elements.
//! Original radix-two implementation. Classical transform reference:
//! J. M. Pollard, "The fast Fourier transform in a finite field",
//! Mathematics of Computation 25 (1971), 365–374.

pub const MODULUS: u64 = 998_244_353;
/// Largest power of two dividing MODULUS - 1.
pub const MAX_TRANSFORM_LEN: usize = 1 << 23;
const ROOT: u64 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    LengthOverflow,
    TransformTooLarge,
    AllocationFailed,
}

fn pow(mut base: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = result * base % MODULUS;
        }
        base = base * base % MODULUS;
        exponent >>= 1;
    }
    result
}

fn transform(values: &mut [u64], inverse: bool) {
    let n = values.len();
    debug_assert!(n.is_power_of_two() && n <= MAX_TRANSFORM_LEN);
    debug_assert!(values.iter().all(|&x| x < MODULUS));
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            values.swap(i, j);
        }
    }
    let mut width = 2;
    while width <= n {
        let mut root = pow(ROOT, (MODULUS - 1) / width as u64);
        if inverse {
            root = pow(root, MODULUS - 2);
        }
        for block in values.chunks_exact_mut(width) {
            let mut weight = 1;
            let (left, right) = block.split_at_mut(width / 2);
            for (a, b) in left.iter_mut().zip(right) {
                let u = *a;
                let v = *b * weight % MODULUS;
                *a = (u + v) % MODULUS;
                *b = (u + MODULUS - v) % MODULUS;
                weight = weight * root % MODULUS;
            }
        }
        width *= 2;
    }
    if inverse {
        let reciprocal = pow(n as u64, MODULUS - 2);
        for value in values {
            *value = *value * reciprocal % MODULUS;
        }
    }
}

fn lengths(a: usize, b: usize) -> Result<(usize, usize), Error> {
    let output = a.checked_add(b - 1).ok_or(Error::LengthOverflow)?;
    let transform = output
        .checked_next_power_of_two()
        .ok_or(Error::LengthOverflow)?;
    if transform > MAX_TRANSFORM_LEN {
        return Err(Error::TransformTooLarge);
    }
    Ok((output, transform))
}

fn padded(input: &[u64], len: usize) -> Result<Vec<u64>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    values.extend(input.iter().map(|&x| x % MODULUS));
    values.resize(len, 0);
    Ok(values)
}

/// Coefficients in ascending degree order; arbitrary u64 inputs are normalized.
/// Empty input returns an empty polynomial before length checks. For nonempty
/// inputs, length arithmetic precedes the transform cap, then allocation.
/// O(L log L) time and O(L) auxiliary words, L=next_power_of_two(a.len+b.len-1).
/// Exact residues, not integer convolution or floating-point approximations.
pub fn convolution(a: &[u64], b: &[u64]) -> Result<Vec<u64>, Error> {
    if a.is_empty() || b.is_empty() {
        return Ok(Vec::new());
    }
    let (output, len) = lengths(a.len(), b.len())?;
    let mut left = padded(a, len)?;
    let mut right = padded(b, len)?;
    transform(&mut left, false);
    transform(&mut right, false);
    for (a, b) in left.iter_mut().zip(right) {
        *a = *a * b % MODULUS;
    }
    transform(&mut left, true);
    left.truncate(output);
    Ok(left)
}

#[cfg(test)]
mod tests;
