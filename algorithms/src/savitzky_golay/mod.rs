//! Local polynomial smoothing; Savitzky and Golay, Analytical Chemistry 36 (1964).
use crate::householder_qr;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    LeastSquares(householder_qr::Error),
    NonFiniteArithmetic,
}
/// Unit spaced grid; each full odd window shifts inside the array at boundaries (no padding).
/// n<=4096; odd window<=31; degree<=6 and degree<window<=n; finite |x|<=1e50.
/// Empty input is invalid. Scaled offsets (j-i)/window; evaluate fitted polynomial at zero.
/// Reuses Householder QR. O(n*window*(degree+1)^2) work, O(n+window*degree) space.
/// Standard allocations may abort; QR allocation errors propagate. Fit is not a signal-error bound.
pub fn smooth(input: &[f64], window: usize, degree: usize) -> Result<Vec<f64>, Error> {
    let n = input.len();
    if n > 4096
        || window == 0
        || window > 31
        || window.is_multiple_of(2)
        || window > n
        || degree > 6
        || degree >= window
        || input.iter().any(|x| !x.is_finite() || x.abs() > 1e50)
    {
        return Err(Error::InvalidInput);
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let start = i.saturating_sub(window / 2).min(n - window);
        let mut a = Vec::with_capacity(window);
        for j in start..start + window {
            let t = (j as f64 - i as f64) / window as f64;
            let mut row = vec![1.0; degree + 1];
            for k in 1..=degree {
                row[k] = row[k - 1] * t;
            }
            a.push(row);
        }
        let fit = householder_qr::least_squares(&a, &input[start..start + window])
            .map_err(Error::LeastSquares)?;
        let value = fit.x[0];
        if !value.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        out.push(value);
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
