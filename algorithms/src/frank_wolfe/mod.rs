//! Frank-Wolfe on the unit simplex with exact quadratic line minimization.
//! Frank and Wolfe, Naval Research Logistics Quarterly 3 (1956), 95-110.
pub use crate::projected_gradient_box::{Error, Report, Status};
use crate::projected_gradient_box::{buffer, evaluate, validate};
/// Symmetric diagonally dominant PSD Q, dimension 1..64. Initial nonnegative
/// weights must sum to one within 64*n*EPSILON. Earliest index wins gradient ties.
/// Residual is the nonnegative computed Frank-Wolfe dual gap; floating arithmetic
/// does not provide a rigorous objective-gap or relative-solution certificate.
pub fn minimize(
    matrix: &[Vec<f64>],
    linear: &[f64],
    initial: &[f64],
    tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    validate(matrix, linear, initial, tolerance, max_iterations)?;
    let n = linear.len();
    let mass: f64 = initial.iter().sum();
    let mass_tol = 64.0 * n as f64 * f64::EPSILON;
    if initial.iter().any(|x| *x < 0.0) || (mass - 1.0).abs() > mass_tol {
        return Err(Error::InvalidInput);
    }
    let mut x = buffer(n)?;
    x.copy_from_slice(initial);
    let mut gradient = buffer(n)?;
    let mut direction = buffer(n)?;
    for iterations in 0..=max_iterations {
        let value = evaluate(matrix, linear, &x, &mut gradient)?;
        let mut vertex = 0;
        for i in 1..n {
            if gradient[i] < gradient[vertex] {
                vertex = i;
            }
        }
        for i in 0..n {
            direction[i] = if i == vertex { 1.0 - x[i] } else { -x[i] };
        }
        let gap = -gradient
            .iter()
            .zip(&direction)
            .map(|(g, d)| g * d)
            .sum::<f64>();
        if !gap.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        let residual = gap.max(0.0);
        let status = if residual <= tolerance {
            Some(Status::Converged)
        } else if iterations == max_iterations {
            Some(Status::BudgetExhausted)
        } else {
            None
        };
        if let Some(status) = status {
            return Ok(Report {
                x,
                value,
                residual,
                iterations,
                status,
            });
        }
        let mut curvature = 0.0;
        for i in 0..n {
            curvature += direction[i]
                * matrix[i]
                    .iter()
                    .zip(&direction)
                    .map(|(q, d)| q * d)
                    .sum::<f64>();
        }
        if !curvature.is_finite() || curvature < 0.0 {
            return Err(Error::NonFiniteArithmetic);
        }
        let alpha = if curvature == 0.0 {
            1.0
        } else {
            (gap / curvature).min(1.0)
        };
        let mut changed = false;
        for (i, weight) in x.iter_mut().enumerate() {
            let next = (1.0 - alpha) * *weight + if i == vertex { alpha } else { 0.0 };
            if !next.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            changed |= next != *weight;
            *weight = next;
        }
        if (x.iter().sum::<f64>() - 1.0).abs() > mass_tol {
            return Err(Error::NonFiniteArithmetic);
        }
        if !changed {
            return Ok(Report {
                x,
                value,
                residual,
                iterations: iterations + 1,
                status: Status::Stagnated,
            });
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
