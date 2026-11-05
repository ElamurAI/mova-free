//! Proximal map of lambda times the L1 norm.
//! Reference: Parikh and Boyd, Proximal Algorithms, Foundations and Trends
//! in Optimization 1(3), 2014, section 6.5.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdError {
    NonFinite,
    NegativeLambda,
    TooLarge,
    AllocationFailed,
}
pub const MAX_LEN: usize = 1_000_000;
/// Coordinate minimizer of 0.5*||x-input||² + lambda*||x||1.
/// Finite operands yield finite values; subtraction and underflow are rounded.
pub fn soft_threshold(input: &[f64], lambda: f64) -> Result<Vec<f64>, ThresholdError> {
    if input.len() > MAX_LEN {
        return Err(ThresholdError::TooLarge);
    }
    if !lambda.is_finite() || input.iter().any(|x| !x.is_finite()) {
        return Err(ThresholdError::NonFinite);
    }
    if lambda < 0.0 {
        return Err(ThresholdError::NegativeLambda);
    }
    let mut out = Vec::new();
    out.try_reserve_exact(input.len())
        .map_err(|_| ThresholdError::AllocationFailed)?;
    for &x in input {
        out.push((x.abs() - lambda).max(0.0).copysign(x));
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
