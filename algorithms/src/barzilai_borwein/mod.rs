//! barzilai borwein for positive diagonal quadratics.
//! Barzilai and Borwein, IMA Journal of Numerical Analysis 8 (1988), 141–148.
pub use crate::gradient_descent_quadratic::{Error, Report, Status};
fn buffer(n: usize) -> Result<Vec<f64>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(n, 0.0);
    Ok(v)
}
/// Minimize `0.5*x'Dx-linear'x`.
/// Dimension 1..256, diagonal [1e-6,1e6], other magnitudes <=1e6.
/// Residual is the absolute Euclidean gradient norm, not solution error.
/// Convergence is checked before the budget; zero budget evaluates the initial point.
/// At most 100000 updates. No stagnation claim is made from a single unchanged iterate.
pub fn minimize_diagonal(
    diagonal: &[f64],
    linear: &[f64],
    initial: &[f64],
    gradient_tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let n = diagonal.len();
    if n == 0
        || n > 256
        || linear.len() != n
        || initial.len() != n
        || diagonal
            .iter()
            .any(|d| !d.is_finite() || *d < 1e-6 || *d > 1e6)
        || linear
            .iter()
            .chain(initial)
            .any(|v| !v.is_finite() || v.abs() > 1e6)
        || !gradient_tolerance.is_finite()
        || gradient_tolerance <= 0.0
        || max_iterations > 100000
    {
        return Err(Error::InvalidInput);
    }
    let low = diagonal.iter().copied().fold(f64::INFINITY, f64::min);
    let high = diagonal.iter().copied().fold(0.0, f64::max);
    let mut x = buffer(n)?;
    x.copy_from_slice(initial);
    let mut previous = buffer(n)?;
    previous.copy_from_slice(initial);
    let mut step = 1.0 / high;
    for iterations in 0..=max_iterations {
        let mut residual_norm = 0.0_f64;
        for i in 0..n {
            let gradient = diagonal[i] * x[i] - linear[i];
            if !gradient.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            residual_norm = residual_norm.hypot(gradient);
        }
        if !residual_norm.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        if residual_norm <= gradient_tolerance || iterations == max_iterations {
            let status = if residual_norm <= gradient_tolerance {
                Status::Converged
            } else {
                Status::BudgetExhausted
            };
            return Ok(Report {
                x,
                residual_norm,
                iterations,
                status,
            });
        }
        if iterations > 0 {
            let mut ss = 0.0;
            let mut sy = 0.0;
            for i in 0..n {
                let s = x[i] - previous[i];
                ss += s * s;
                sy += diagonal[i] * s * s;
            }
            if !ss.is_finite() || !sy.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            step = if sy > 0.0 {
                (ss / sy).clamp(1.0 / high, 1.0 / low)
            } else {
                1.0 / high
            };
        }
        for i in 0..n {
            let next = x[i] - step * (diagonal[i] * x[i] - linear[i]);
            if !next.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            previous[i] = x[i];
            x[i] = next;
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
