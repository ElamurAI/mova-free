//! FISTA with the original t-sequence for the LASSO objective.
//! Beck and Teboulle, SIAM Journal on Imaging Sciences 2(1) (2009), 183–202.
use crate::projected_gradient_box::buffer;
pub use crate::projected_gradient_box::{Error, Report, Status};
use crate::proximal_gradient_lasso::{evaluate, proximal_step, validate};
/// Accelerated proximal minimization from zero with L=||A||_F².
/// Same bounded domain and KKT stopping metric as proximal_gradient_lasso::minimize.
/// Momentum uses extrapolated y, but reports always evaluate the actual x iterate.
/// The objective need not decrease on every step; no single-step stagnation claim.
/// Real-arithmetic O(1/k²) objective theory is not an f64 solution-error guarantee.
pub fn minimize(
    design: &[Vec<f64>],
    response: &[f64],
    lambda: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let (n, scale) = validate(design, response, lambda, tolerance, max_iterations)?;
    let mut x = buffer(n)?;
    let mut extrapolated = buffer(n)?;
    let mut gradient = buffer(n)?;
    let mut t = 1.0_f64;
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
        evaluate(design, response, &extrapolated, lambda, &mut gradient)?;
        let next = proximal_step(&extrapolated, &mut gradient, scale, lambda)?;
        let next_t = (1.0 + (1.0 + 4.0 * t * t).sqrt()) / 2.0;
        let momentum = (t - 1.0) / next_t;
        for i in 0..n {
            extrapolated[i] = next[i] + momentum * (next[i] - x[i]);
            if !extrapolated[i].is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
        x = next;
        t = next_t;
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
