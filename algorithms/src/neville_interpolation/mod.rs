//! Neville's triangular interpolation (Stoer and Bulirsch, Introduction to Numerical Analysis, ch. 2).
//! Finite strictly increasing nodes; extrapolation allowed. No conditioning or accuracy guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}
/// Validates shape before finiteness/order, then evaluates using O(n) storage.
pub fn neville_interpolation(xs: &[f64], ys: &[f64], x: f64) -> Result<f64, Error> {
    if xs.is_empty()
        || xs.len() != ys.len()
        || !x.is_finite()
        || xs.iter().chain(ys).any(|v| !v.is_finite())
        || xs.windows(2).any(|w| w[0] >= w[1])
    {
        return Err(Error::InvalidInput);
    }
    let mut p = ys.to_vec();
    for width in 1..xs.len() {
        for i in 0..xs.len() - width {
            let d = xs[i + width] - xs[i];
            let a = xs[i + width] - x;
            let b = x - xs[i];
            let left = a * p[i];
            let right = b * p[i + 1];
            let sum = left + right;
            if [d, a, b, left, right, sum].iter().any(|v| !v.is_finite()) {
                return Err(Error::NonFiniteArithmetic);
            }
            p[i] = sum / d;
            if !p[i].is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
    }
    Ok(p[0])
}
#[cfg(test)]
mod tests;
