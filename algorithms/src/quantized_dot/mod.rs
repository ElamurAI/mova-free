//! Integer quantized dot product with exact residual certificate.
//! Error decomposition follows standard perturbation analysis (Higham,
//! Accuracy and Stability of Numerical Algorithms, dot products).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    LengthMismatch,
    ZeroStep,
    UnsupportedPercent,
    Overflow,
    Uncertified,
    WorkBudget,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Certificate {
    pub value: i128,
    pub absolute_error: u128,
    /// Sum |a_i*b_i|, NOT absolute value of the sum.
    pub l1_products: u128,
    pub percent: u8,
}
/// Truncate each input toward zero to a multiple of step.
/// Certify |approx-exact| <= absolute_error <= percent*l1_products/100.
/// A zero L1 requires a zero certificate. Cancellation does not invalidate
/// this metric. Unsupported/uncertified calls fail without exact fallback.
/// O(n) time, O(1) space; certification is not a speedup claim.
pub fn dot(
    a: &[i64],
    b: &[i64],
    step: u64,
    percent: u8,
    work_budget: usize,
) -> Result<Certificate, Error> {
    if a.len() != b.len() {
        return Err(Error::LengthMismatch);
    }
    if step == 0 {
        return Err(Error::ZeroStep);
    }
    if !matches!(percent, 1 | 2 | 5 | 10) {
        return Err(Error::UnsupportedPercent);
    }
    if a.len() > work_budget {
        return Err(Error::WorkBudget);
    }
    let s = i128::from(step);
    let mut value = 0i128;
    let mut error = 0u128;
    let mut l1 = 0u128;
    for (&a, &b) in a.iter().zip(b) {
        let a = i128::from(a);
        let b = i128::from(b);
        let qa = (a / s) * s;
        let qb = (b / s) * s;
        let ra = (a - qa).unsigned_abs();
        let rb = (b - qb).unsigned_abs();
        value = value
            .checked_add(qa.checked_mul(qb).ok_or(Error::Overflow)?)
            .ok_or(Error::Overflow)?;
        let e = qa
            .unsigned_abs()
            .checked_mul(rb)
            .and_then(|v| {
                qb.unsigned_abs()
                    .checked_mul(ra)
                    .and_then(|w| v.checked_add(w))
            })
            .and_then(|v| ra.checked_mul(rb).and_then(|w| v.checked_add(w)))
            .ok_or(Error::Overflow)?;
        error = error.checked_add(e).ok_or(Error::Overflow)?;
        l1 = l1
            .checked_add((a * b).unsigned_abs())
            .ok_or(Error::Overflow)?;
    }
    // Quotient/remainder comparison avoids multiplying potentially large sums.
    let allowed = (l1 / 100)
        .checked_mul(u128::from(percent))
        .and_then(|v| v.checked_add((l1 % 100) * u128::from(percent) / 100))
        .ok_or(Error::Overflow)?;
    if error > allowed {
        return Err(Error::Uncertified);
    }
    Ok(Certificate {
        value,
        absolute_error: error,
        l1_products: l1,
        percent,
    })
}
#[cfg(test)]
mod tests;
