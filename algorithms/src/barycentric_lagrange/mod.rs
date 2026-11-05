//! Barycentric polynomial interpolation. Berrut and Trefethen (2004),
//! SIAM Review 46(3), 501–517. Original implementation of the second formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Size,
    NonFinite,
    Domain,
    Spacing,
    Arithmetic,
}
/// 1..=64 sorted nodes on [-1,1], spacing >=1e-8, |y|<=1e100.
/// Query must lie in the node hull. O(n²) time, fixed 64-value stack.
/// Real polynomial reproduction does not certify floating accuracy.
pub fn interpolate(nodes: &[(f64, f64)], x: f64) -> Result<f64, Error> {
    if nodes.is_empty() || nodes.len() > 64 {
        return Err(Error::Size);
    }
    if !x.is_finite() || nodes.iter().any(|&(a, b)| !a.is_finite() || !b.is_finite()) {
        return Err(Error::NonFinite);
    }
    if nodes.iter().any(|&(a, b)| a.abs() > 1.0 || b.abs() > 1e100)
        || x < nodes[0].0
        || x > nodes[nodes.len() - 1].0
    {
        return Err(Error::Domain);
    }
    if nodes.windows(2).any(|w| w[1].0 - w[0].0 < 1e-8) {
        return Err(Error::Spacing);
    }
    if let Some(&(_, y)) = nodes.iter().find(|&&(a, _)| a == x) {
        return Ok(y);
    }
    let mut terms = [0.0; 64];
    let mut scale: f64 = 0.0;
    for (i, &(a, _)) in nodes.iter().enumerate() {
        let mut product = 1.0;
        for (j, &(b, _)) in nodes.iter().enumerate() {
            if i != j {
                product *= a - b;
            }
        }
        terms[i] = 1.0 / (product * (x - a));
        if !terms[i].is_finite() {
            return Err(Error::Arithmetic);
        }
        scale = scale.max(terms[i].abs());
    }
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (i, &(_, y)) in nodes.iter().enumerate() {
        let t = terms[i] / scale;
        numerator += t * y;
        denominator += t;
    }
    let value = numerator / denominator;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::Arithmetic)
    }
}
#[cfg(test)]
mod tests;
