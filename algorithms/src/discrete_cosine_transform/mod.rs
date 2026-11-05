//! Orthonormal DCT-II and its transpose inverse.
//! Ahmed, Natarajan and Rao, "Discrete Cosine Transform", IEEE Trans. Computers (1974).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}

/// Unit sample grid; no extension. Empty maps to empty; n<=2048, |x|<=1e50 finite.
/// DC weight sqrt(1/n), other weights sqrt(2/n). O(n²) time, O(n) space.
/// Allocator exhaustion follows standard policy; no relative roundoff guarantee.
pub fn transform(input: &[f64], inverse: bool) -> Result<Vec<f64>, Error> {
    let n = input.len();
    if n > 2048 || input.iter().any(|x| !x.is_finite() || x.abs() > 1e50) {
        return Err(Error::InvalidInput);
    }
    let mut out = vec![0.0; n];
    for (k, y) in out.iter_mut().enumerate() {
        for (j, &x) in input.iter().enumerate() {
            let (sample, frequency) = if inverse { (k, j) } else { (j, k) };
            let weight = if frequency == 0 {
                (1.0 / n as f64).sqrt()
            } else {
                (2.0 / n as f64).sqrt()
            };
            *y += x
                * weight
                * (std::f64::consts::PI * (sample as f64 + 0.5) * frequency as f64 / n as f64)
                    .cos();
        }
        if !y.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
