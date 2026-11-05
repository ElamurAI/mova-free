//! Exact Walsh–Hadamard transform and XOR convolution in a prime field.
//! Original butterfly implementation. Classical transform background:
//! J. L. Walsh, "A closed set of normal orthogonal functions", American
//! Journal of Mathematics 45(1), 1923, pp. 5–24.

pub const MODULUS: u64 = 998_244_353;
pub const MAX_LEN: usize = 1 << 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidLength,
    LengthTooLarge,
    AllocationFailed,
}

fn power(mut x: u64, mut e: u64) -> u64 {
    let mut y = 1;
    while e > 0 {
        if e & 1 != 0 {
            y = y * x % MODULUS;
        }
        x = x * x % MODULUS;
        e >>= 1;
    }
    y
}

/// In-place natural binary-index transform H[k,i]=(-1)^popcount(k&i).
/// Validate nonempty power-of-two length first, then MAX_LEN; errors leave
/// the slice unchanged. Normalize arbitrary u64 inputs. Inverse divides by
/// length: H²=nI and n is invertible in the odd prime field. No primitive
/// root or NTT divisibility condition is needed. O(n log n) time, O(1) space.
pub fn transform(values: &mut [u64], inverse: bool) -> Result<(), Error> {
    if !values.len().is_power_of_two() {
        return Err(Error::InvalidLength);
    }
    if values.len() > MAX_LEN {
        return Err(Error::LengthTooLarge);
    }
    for x in values.iter_mut() {
        *x %= MODULUS;
    }
    let mut half = 1;
    while half < values.len() {
        for block in values.chunks_exact_mut(2 * half) {
            for i in 0..half {
                let (a, b) = (block[i], block[i + half]);
                block[i] = (a + b) % MODULUS;
                block[i + half] = (a + MODULUS - b) % MODULUS;
            }
        }
        half *= 2;
    }
    if inverse {
        let scale = power(values.len() as u64, MODULUS - 2);
        for x in values {
            *x = *x * scale % MODULUS;
        }
    }
    Ok(())
}

fn padded(input: &[u64], n: usize) -> Result<Vec<u64>, Error> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    result.extend_from_slice(input);
    result.resize(n, 0);
    Ok(result)
}

/// Return c[k]=sum_{i XOR j=k} a[i]*b[j] modulo MODULUS. Check both
/// input lengths against MAX_LEN before the empty-operand shortcut. Otherwise
/// zero-pad to next_power_of_two(max(a.len(),b.len())); return that full domain,
/// including trailing zeros. This is XOR, not ordinary polynomial convolution.
/// O(n log n + log MODULUS) time and O(n) storage, n the padded length.
/// Arithmetic is exact: additions <2p and products <=(p-1)²<2^60 fit u64;
/// the cap bounds every index, length doubling and next-power calculation.
pub fn xor_convolution(a: &[u64], b: &[u64]) -> Result<Vec<u64>, Error> {
    if a.len() > MAX_LEN || b.len() > MAX_LEN {
        return Err(Error::LengthTooLarge);
    }
    if a.is_empty() || b.is_empty() {
        return Ok(Vec::new());
    }
    let n = a.len().max(b.len()).next_power_of_two();
    let mut x = padded(a, n)?;
    let mut y = padded(b, n)?;
    transform(&mut x, false)?;
    transform(&mut y, false)?;
    for (u, v) in x.iter_mut().zip(y) {
        *u = *u * v % MODULUS;
    }
    transform(&mut x, true)?;
    Ok(x)
}

#[cfg(test)]
mod tests;
