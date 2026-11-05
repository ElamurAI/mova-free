//! Full-coefficient Chebyshev series via Clenshaw (1955), MTAC 9, 118–120.
//! Real recurrence models sum c[k] T_k(x); f64 results have no universal relative bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}
/// Evaluate on [-1,1]; empty series is zero; constant coefficient is not halved.
pub fn evaluate(coefficients: &[f64], x: f64) -> Result<f64, Error> {
    if !x.is_finite() || x.abs() > 1.0 || coefficients.iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput);
    }
    let (mut b1, mut b2) = (0.0, 0.0);
    for &c in coefficients.iter().skip(1).rev() {
        let b = 2.0 * x * b1 - b2 + c;
        if !b.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        b2 = b1;
        b1 = b;
    }
    let value = x * b1 - b2 + coefficients.first().copied().unwrap_or(0.0);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::NonFiniteArithmetic)
    }
}
#[cfg(test)]
mod tests;
