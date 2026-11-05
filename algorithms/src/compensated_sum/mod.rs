//! Neumaier compensated summation: Neumaier (1974), ZAMM 54, 39–51.
//! Finite f64 arithmetic; compensation improves many cancellations but is not exact generally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}
/// Sum in input order using a correction; rejects nonfinite inputs and intermediates.
pub fn sum(values: &[f64]) -> Result<f64, Error> {
    if values.iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput);
    }
    let (mut total, mut correction): (f64, f64) = (0.0, 0.0);
    for &value in values {
        let next = total + value;
        correction += if total.abs() >= value.abs() {
            (total - next) + value
        } else {
            (value - next) + total
        };
        total = next;
        if !total.is_finite() || !correction.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
    }
    let result = total + correction;
    if result.is_finite() {
        Ok(result)
    } else {
        Err(Error::NonFiniteArithmetic)
    }
}
#[cfg(test)]
mod tests;
