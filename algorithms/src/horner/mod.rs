//! Simultaneous polynomial value and derivative, Horner (1819), Phil. Trans. 109.
//! Ascending coefficients. Rounded f64 evaluation, not a certified relative approximation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}
/// Returns (p(x), p'(x)); an empty polynomial returns (0,0).
pub fn evaluate(coefficients: &[f64], x: f64) -> Result<(f64, f64), Error> {
    if !x.is_finite() || coefficients.iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput);
    }
    let (mut value, mut derivative) = (0.0, 0.0);
    for &coefficient in coefficients.iter().rev() {
        derivative = derivative * x + value;
        value = value * x + coefficient;
    }
    // With finite inputs, a nonfinite accumulator cannot recover through these
    // IEEE-754 recurrences. One final gate preserves the Result observation;
    // rejected inputs may perform more work than an early per-step gate.
    if !value.is_finite() || !derivative.is_finite() {
        return Err(Error::NonFiniteArithmetic);
    }
    Ok((value, derivative))
}
#[cfg(test)]
mod tests;
