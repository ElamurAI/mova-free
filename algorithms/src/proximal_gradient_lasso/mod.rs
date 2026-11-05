//! Proximal gradient (ISTA) for the LASSO objective.
//! Beck and Teboulle, SIAM Journal on Imaging Sciences 2(1) (2009), 183–202.
use crate::projected_gradient_box::buffer;
pub use crate::projected_gradient_box::{Error, Report, Status};

pub(crate) fn validate(
    design: &[Vec<f64>],
    response: &[f64],
    lambda: f64,
    tolerance: f64,
    budget: usize,
) -> Result<(usize, f64), Error> {
    let m = design.len();
    let n = design.first().map_or(0, Vec::len);
    let bounded = |v: f64| v.is_finite() && v.abs() <= 1e6;
    if m == 0
        || m > 256
        || n == 0
        || n > 64
        || response.len() != m
        || design
            .iter()
            .any(|r| r.len() != n || r.iter().any(|&v| !bounded(v)))
        || response.iter().any(|&v| !bounded(v))
        || !bounded(lambda)
        || lambda < 0.0
        || !tolerance.is_finite()
        || tolerance <= 0.0
        || budget > 100000
    {
        return Err(Error::InvalidInput);
    }
    let scale: f64 = design.iter().flatten().map(|v| v * v).sum();
    let nonzero = design.iter().flatten().any(|v| *v != 0.0);
    if nonzero && scale < 1e-12 {
        return Err(Error::InvalidInput);
    }
    Ok((n, scale))
}
/// Recompute smooth gradient and objective; no cached residual decides stopping.
pub(crate) fn evaluate(
    design: &[Vec<f64>],
    response: &[f64],
    x: &[f64],
    lambda: f64,
    gradient: &mut [f64],
) -> Result<(f64, f64), Error> {
    gradient.fill(0.0);
    let mut value = 0.0;
    for (row, &b) in design.iter().zip(response) {
        let residual = row.iter().zip(x).map(|(a, x)| a * x).sum::<f64>() - b;
        value += 0.5 * residual * residual;
        for (g, a) in gradient.iter_mut().zip(row) {
            *g += a * residual;
        }
    }
    value += lambda * x.iter().map(|v| v.abs()).sum::<f64>();
    let mut violation = 0.0_f64;
    for (&g, &v) in gradient.iter().zip(x) {
        if !g.is_finite() || !v.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        let kkt = if v > 0.0 {
            (g + lambda).abs()
        } else if v < 0.0 {
            (g - lambda).abs()
        } else {
            (g.abs() - lambda).max(0.0)
        };
        violation = violation.max(kkt);
    }
    if !value.is_finite() || !violation.is_finite() {
        return Err(Error::NonFiniteArithmetic);
    }
    Ok((value, violation))
}
pub(crate) fn proximal_step(
    point: &[f64],
    gradient: &mut [f64],
    scale: f64,
    lambda: f64,
) -> Result<Vec<f64>, Error> {
    for (g, &v) in gradient.iter_mut().zip(point) {
        *g = v - *g / scale;
    }
    crate::soft_threshold::soft_threshold(gradient, lambda / scale).map_err(|e| match e {
        crate::soft_threshold::ThresholdError::AllocationFailed => Error::AllocationFailed,
        _ => Error::NonFiniteArithmetic,
    })
}
/// Minimize `0.5*||Ax-response||² + lambda*||x||1`, starting at zero.
/// Rows 1..256, columns 1..64, finite magnitudes <=1e6; lambda >=0.
/// Nonzero designs require squared Frobenius norm >=1e-12. Step is its reciprocal.
/// Positive finite tolerance; at most 100000 updates, zero permitted.
/// Residual is recomputed KKT infinity violation, not coordinate error.
/// Convergence wins ties with the budget; zero designs return zero immediately.
pub fn minimize(
    design: &[Vec<f64>],
    response: &[f64],
    lambda: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let (n, scale) = validate(design, response, lambda, tolerance, max_iterations)?;
    let mut x = buffer(n)?;
    let mut gradient = buffer(n)?;
    for iterations in 0..=max_iterations {
        let (value, residual) = evaluate(design, response, &x, lambda, &mut gradient)?;
        if residual <= tolerance || iterations == max_iterations {
            let status = if residual <= tolerance {
                Status::Converged
            } else {
                Status::BudgetExhausted
            };
            return Ok(Report {
                x,
                value,
                residual,
                iterations,
                status,
            });
        }
        x = proximal_step(&x, &mut gradient, scale, lambda)?;
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
