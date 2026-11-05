//! Selected DFT bins; Goertzel, "An Algorithm for the Evaluation of Finite Trigonometric Series" (1958).
use crate::fast_fourier_transform::Complex;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}

/// Real samples on unit grid; negative-phase unnormalized DFT. Bins preserve order/duplicates.
/// n<=65536, bin count<=65536, every bin<n, finite |x|<=1e50. Empty samples require empty bins.
/// O(n*bin_count) time/O(bin_count) output; standard allocator policy.
/// Endpoint cancellation can amplify relative error; no accuracy certificate.
pub fn bins(input: &[f64], indices: &[usize]) -> Result<Vec<Complex>, Error> {
    let n = input.len();
    if n > 65536
        || indices.len() > 65536
        || indices.iter().any(|&k| k >= n)
        || input.iter().any(|x| !x.is_finite() || x.abs() > 1e50)
    {
        return Err(Error::InvalidInput);
    }
    let mut result = Vec::with_capacity(indices.len());
    for &k in indices {
        let angle = 2.0 * std::f64::consts::PI * k as f64 / n as f64;
        let (s, c) = angle.sin_cos();
        let (mut p, mut q) = (0.0, 0.0);
        for &x in input {
            let next = x + 2.0 * c * p - q;
            q = p;
            p = next;
        }
        // Integer bins supply exp(-i*n*angle)=1: rotate the usual terminal state.
        let z = Complex {
            re: c * p - q,
            im: s * p,
        };
        if !z.re.is_finite() || !z.im.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        result.push(z);
    }
    Ok(result)
}
#[cfg(test)]
mod tests;
