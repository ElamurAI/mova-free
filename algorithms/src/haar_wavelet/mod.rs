//! Full orthonormal Haar transform; Haar, "Zur Theorie der orthogonalen Funktionensysteme" (1910).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}

/// Power-of-two length 1..=65536, finite |x|<=1e50; no padding or extension.
/// Packed [coarsest average, coarsest detail, ..., finest details], left-minus-right.
/// Inverse consumes this layout. O(n) time/space. Standard allocator policy; no relative error bound.
pub fn transform(input: &[f64], inverse: bool) -> Result<Vec<f64>, Error> {
    let n = input.len();
    if !n.is_power_of_two() || n > 65536 || input.iter().any(|x| !x.is_finite() || x.abs() > 1e50) {
        return Err(Error::InvalidInput);
    }
    let mut out = input.to_vec();
    let mut scratch = vec![0.0; n];
    let scale = std::f64::consts::FRAC_1_SQRT_2;
    if inverse {
        let mut width = 2;
        while width <= n {
            for j in 0..width / 2 {
                scratch[2 * j] = (out[j] + out[width / 2 + j]) * scale;
                scratch[2 * j + 1] = (out[j] - out[width / 2 + j]) * scale;
            }
            out[..width].copy_from_slice(&scratch[..width]);
            width *= 2;
        }
    } else {
        let mut width = n;
        while width > 1 {
            for j in 0..width / 2 {
                scratch[j] = (out[2 * j] + out[2 * j + 1]) * scale;
                scratch[width / 2 + j] = (out[2 * j] - out[2 * j + 1]) * scale;
            }
            out[..width].copy_from_slice(&scratch[..width]);
            width /= 2;
        }
    }
    if out.iter().any(|x| !x.is_finite()) {
        return Err(Error::NonFiniteArithmetic);
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
