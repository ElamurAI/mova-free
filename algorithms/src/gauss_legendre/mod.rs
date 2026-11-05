//! Fixed-order Gauss–Legendre quadrature. Davis and Rabinowitz, Methods of
//! Numerical Integration, 2nd ed., Gaussian quadrature. Constants rounded f64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Order,
    NonFinite,
    Domain,
    Callback,
    Arithmetic,
}
/// Orders 1..=5; -1e100<=a<=b<=1e100. Equal endpoints return zero without calls.
/// Otherwise exactly order calls in increasing node order. Callback must be
/// deterministic, finite, |f|<=1e100; panics propagate. No allocations.
/// Real exactness through degree 2*order-1 excludes table and arithmetic error.
pub fn integrate<F: FnMut(f64) -> f64>(
    mut f: F,
    a: f64,
    b: f64,
    order: usize,
) -> Result<f64, Error> {
    if !(1..=5).contains(&order) {
        return Err(Error::Order);
    }
    if !a.is_finite() || !b.is_finite() {
        return Err(Error::NonFinite);
    }
    if a > b || a.abs() > 1e100 || b.abs() > 1e100 {
        return Err(Error::Domain);
    }
    if a == b {
        return Ok(0.0);
    }
    let table: &[(f64, f64)] = match order {
        1 => &[(0.0, 2.0)],
        2 => &[(-0.5773502691896257, 1.0), (0.5773502691896257, 1.0)],
        3 => &[
            (-0.7745966692414834, 0.5555555555555556),
            (0.0, 0.8888888888888888),
            (0.7745966692414834, 0.5555555555555556),
        ],
        4 => &[
            (-0.8611363115940526, 0.3478548451374538),
            (-0.3399810435848563, 0.6521451548625461),
            (0.3399810435848563, 0.6521451548625461),
            (0.8611363115940526, 0.3478548451374538),
        ],
        _ => &[
            (-0.906179845938664, 0.2369268850561891),
            (-0.5384693101056831, 0.4786286704993665),
            (0.0, 0.5688888888888889),
            (0.5384693101056831, 0.4786286704993665),
            (0.906179845938664, 0.2369268850561891),
        ],
    };
    let midpoint = a + (b - a) * 0.5;
    let half = (b - a) * 0.5;
    let mut sum = 0.0;
    for &(x, w) in table {
        let y = f(midpoint + half * x);
        if !y.is_finite() || y.abs() > 1e100 {
            return Err(Error::Callback);
        }
        sum += w * y;
    }
    let value = half * sum;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::Arithmetic)
    }
}
#[cfg(test)]
mod tests;
