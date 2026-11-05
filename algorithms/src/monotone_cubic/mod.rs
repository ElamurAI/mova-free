//! Shape-preserving cubic Hermite interpolation, Fritsch and Butland (1984),
//! SIAM J. Sci. Stat. Comput. 5, 300–304; one-sided endpoint limiter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Size,
    NonFinite,
    Domain,
    Spacing,
    Arithmetic,
}
fn endpoint(h: f64, k: f64, d: f64, e: f64) -> f64 {
    let m = ((2.0 * h + k) * d - h * e) / (h + k);
    if m.signum() != d.signum() {
        0.0
    } else if d.signum() != e.signum() && m.abs() > 3.0 * d.abs() {
        3.0 * d
    } else {
        m
    }
}
fn slope(p: &[(f64, f64)], i: usize) -> f64 {
    let n = p.len();
    let delta = |j: usize| (p[j + 1].1 - p[j].1) / (p[j + 1].0 - p[j].0);
    let width = |j: usize| p[j + 1].0 - p[j].0;
    if n == 2 {
        return delta(0);
    }
    if i == 0 {
        return endpoint(width(0), width(1), delta(0), delta(1));
    }
    if i == n - 1 {
        return endpoint(width(n - 2), width(n - 3), delta(n - 2), delta(n - 3));
    }
    let a = delta(i - 1);
    let b = delta(i);
    if a == 0.0 || b == 0.0 || a.signum() != b.signum() {
        return 0.0;
    }
    let w1 = 2.0 * width(i) + width(i - 1);
    let w2 = width(i) + 2.0 * width(i - 1);
    (w1 + w2) / (w1 / a + w2 / b)
}
/// 2..=64 sorted nodes in [-1,1], spacing >=1e-8, |y|<=1e100.
/// In-domain query only. O(n) time and O(1) auxiliary space, no allocation.
/// Shape preservation is a real-arithmetic property, not an error certificate.
pub fn interpolate(nodes: &[(f64, f64)], x: f64) -> Result<f64, Error> {
    if nodes.len() < 2 || nodes.len() > 64 {
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
    let i = nodes.partition_point(|&(a, _)| a < x) - 1;
    let h = nodes[i + 1].0 - nodes[i].0;
    let t = (x - nodes[i].0) / h;
    let value = (2.0 * t - 3.0) * t * t * nodes[i].1
        + nodes[i].1
        + (-2.0 * t + 3.0) * t * t * nodes[i + 1].1
        + ((t - 2.0) * t + 1.0) * t * h * slope(nodes, i)
        + (t - 1.0) * t * t * h * slope(nodes, i + 1);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::Arithmetic)
    }
}
#[cfg(test)]
mod tests;
