//! Euclidean simplex projection; Duchi et al., Efficient Projections onto the
//! l1-Ball for Learning in High Dimensions, ICML (2008).
//! Shift by the maximum before sorting; f64 rounding remains observable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionError {
    NonFinite,
    NegativeRadius,
    EmptySimplex,
    TooLarge,
    Arithmetic,
    AllocationFailed,
}
pub const MAX_LEN: usize = 1_000_000;
/// Project onto {x>=0: sum x=radius}. Empty input requires radius zero.
/// Reject nonfinite intermediate arithmetic and failed mass checks. Successful
/// output has a checked relative mass residual <= 64*n*EPSILON; this is not a
/// universal objective-error bound. Underflow can cause Arithmetic.
pub fn project_simplex(input: &[f64], radius: f64) -> Result<Vec<f64>, ProjectionError> {
    validate(input, radius)?;
    if input.is_empty() && radius > 0.0 {
        return Err(ProjectionError::EmptySimplex);
    }
    let mut sorted = Vec::new();
    sorted
        .try_reserve_exact(input.len())
        .map_err(|_| ProjectionError::AllocationFailed)?;
    if radius == 0.0 {
        sorted.resize(input.len(), 0.0);
        return Ok(sorted);
    }
    let max = input.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    for &x in input {
        let y = x - max;
        if !y.is_finite() {
            return Err(ProjectionError::Arithmetic);
        }
        sorted.push(y);
    }
    sorted.sort_unstable_by(|a, b| b.total_cmp(a));
    let mut sum = 0.0;
    let mut theta = 0.0;
    for (i, &x) in sorted.iter().enumerate() {
        sum += x;
        let candidate = (sum - radius) / (i + 1) as f64;
        if !candidate.is_finite() {
            return Err(ProjectionError::Arithmetic);
        }
        if x > candidate {
            theta = candidate;
        }
    }
    for (out, &x) in sorted.iter_mut().zip(input) {
        *out = ((x - max) - theta).max(0.0);
        if !out.is_finite() {
            return Err(ProjectionError::Arithmetic);
        }
    }
    let mass: f64 = sorted.iter().sum();
    if !mass.is_finite()
        || ((mass - radius) / radius).abs() > 64.0 * input.len() as f64 * f64::EPSILON
    {
        return Err(ProjectionError::Arithmetic);
    }
    Ok(sorted)
}
pub(crate) fn validate(input: &[f64], radius: f64) -> Result<(), ProjectionError> {
    if input.len() > MAX_LEN {
        return Err(ProjectionError::TooLarge);
    }
    if !radius.is_finite() || input.iter().any(|x| !x.is_finite()) {
        return Err(ProjectionError::NonFinite);
    }
    if radius < 0.0 {
        return Err(ProjectionError::NegativeRadius);
    }
    Ok(())
}
#[cfg(test)]
mod tests;
