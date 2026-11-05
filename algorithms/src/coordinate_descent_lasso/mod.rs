//! Cyclic coordinate descent for .5||Ax-y||²+lambda||x||1.
//! Friedman, Hastie and Tibshirani, Journal of Statistical Software 33(1), 2010.
pub use crate::projected_gradient_box::{Error, Report, Status};
use crate::projected_gradient_box::{bounded, buffer};
/// Starts at zero. 1..256 rows, 1..64 columns, finite magnitudes <=1e50.
/// Residual is the maximum recomputed KKT violation, not coordinate error.
/// Zero columns stay zero; cyclic ascending coordinate order is deterministic.
pub fn minimize(
    design: &[Vec<f64>],
    response: &[f64],
    lambda: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let m = design.len();
    let n = design.first().map_or(0, Vec::len);
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
        || max_iterations > 100000
    {
        return Err(Error::InvalidInput);
    }
    let mut x = buffer(n)?;
    let mut residuals = buffer(m)?;
    let mut norms = buffer(n)?;
    for (j, norm) in norms.iter_mut().enumerate() {
        *norm = design.iter().map(|r| r[j] * r[j]).sum();
        if *norm == 0.0 && design.iter().any(|row| row[j] != 0.0) {
            return Err(Error::NonFiniteArithmetic);
        }
    }
    for iterations in 0..=max_iterations {
        // Recompute each sweep to prevent cached-residual drift determining stopping.
        for i in 0..m {
            residuals[i] = response[i] - design[i].iter().zip(&x).map(|(a, b)| a * b).sum::<f64>();
        }
        let value = 0.5 * residuals.iter().map(|r| r * r).sum::<f64>()
            + lambda * x.iter().map(|v| v.abs()).sum::<f64>();
        let mut residual = 0.0_f64;
        for j in 0..n {
            let gradient = -design
                .iter()
                .zip(&residuals)
                .map(|(row, r)| row[j] * r)
                .sum::<f64>();
            let violation = if x[j] > 0.0 {
                (gradient + lambda).abs()
            } else if x[j] < 0.0 {
                (gradient - lambda).abs()
            } else {
                (gradient.abs() - lambda).max(0.0)
            };
            if !violation.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            residual = residual.max(violation);
        }
        if !value.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
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
        let mut changed = false;
        for j in 0..n {
            if norms[j] == 0.0 {
                continue;
            }
            let correlation = design
                .iter()
                .zip(&residuals)
                .map(|(row, r)| row[j] * r)
                .sum::<f64>()
                + norms[j] * x[j];
            if !correlation.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            let threshold = crate::soft_threshold::soft_threshold(&[correlation], lambda).map_err(
                |e| match e {
                    crate::soft_threshold::ThresholdError::AllocationFailed => {
                        Error::AllocationFailed
                    }
                    _ => Error::NonFiniteArithmetic,
                },
            )?;
            let next = threshold[0] / norms[j];
            if !next.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            let delta = next - x[j];
            changed |= next != x[j];
            x[j] = next;
            for i in 0..m {
                residuals[i] -= design[i][j] * delta;
                if !residuals[i].is_finite() {
                    return Err(Error::NonFiniteArithmetic);
                }
            }
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
