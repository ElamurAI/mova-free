//! De Casteljau triangular evaluation; Farin, Curves and Surfaces for CAGD,
//! 5th ed., chapter 4. Original scalar Bernstein implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Size,
    NonFinite,
    Domain,
    Arithmetic,
}
/// 1..=128 finite controls |c|<=1e100 and t in [0,1].
/// O(n²) time, fixed 128-value stack, no allocation. Real convex-hull property;
/// no universal machine relative-error bound near cancellation or zero.
pub fn evaluate(controls: &[f64], t: f64) -> Result<f64, Error> {
    if controls.is_empty() || controls.len() > 128 {
        return Err(Error::Size);
    }
    if !t.is_finite() || controls.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if !(0.0..=1.0).contains(&t) || controls.iter().any(|v| v.abs() > 1e100) {
        return Err(Error::Domain);
    }
    if t == 0.0 {
        return Ok(controls[0]);
    }
    if t == 1.0 {
        return Ok(controls[controls.len() - 1]);
    }
    let mut row = [0.0; 128];
    row[..controls.len()].copy_from_slice(controls);
    for width in (1..controls.len()).rev() {
        for i in 0..width {
            row[i] = (1.0 - t) * row[i] + t * row[i + 1];
        }
    }
    if row[0].is_finite() {
        Ok(row[0])
    } else {
        Err(Error::Arithmetic)
    }
}
#[cfg(test)]
mod tests;
