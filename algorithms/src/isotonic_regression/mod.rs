//! Unit-weight isotonic least squares by pool adjacent violators.
//! Reference: Barlow et al., Statistical Inference under Order Restrictions (1972).
//! Exact-real projection model; rounded f64 block means are not certified exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsotonicError {
    NonFinite,
    TooLarge,
    Arithmetic,
    AllocationFailed,
}
/// Maximum vector length; a resource policy, not a mathematical restriction.
pub const MAX_LEN: usize = 1_000_000;
/// Minimize sum (x[i]-input[i])² subject to nondecreasing x.
/// Overflow in block sums is rejected even when an exact mean is representable.
pub fn isotonic_regression(input: &[f64]) -> Result<Vec<f64>, IsotonicError> {
    if input.len() > MAX_LEN {
        return Err(IsotonicError::TooLarge);
    }
    if input.iter().any(|x| !x.is_finite()) {
        return Err(IsotonicError::NonFinite);
    }
    let mut blocks: Vec<(f64, usize)> = Vec::new();
    blocks
        .try_reserve_exact(input.len())
        .map_err(|_| IsotonicError::AllocationFailed)?;
    let mut out = Vec::new();
    out.try_reserve_exact(input.len())
        .map_err(|_| IsotonicError::AllocationFailed)?;
    for &x in input {
        blocks.push((x, 1));
        while blocks.len() >= 2 {
            let n = blocks.len();
            let (a, na) = blocks[n - 2];
            let (b, nb) = blocks[n - 1];
            if a / na as f64 <= b / nb as f64 {
                break;
            }
            let sum = a + b;
            if !sum.is_finite() {
                return Err(IsotonicError::Arithmetic);
            }
            blocks.pop();
            blocks[n - 2] = (sum, na + nb);
        }
    }
    for (sum, n) in blocks {
        out.resize(out.len() + n, sum / n as f64);
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
