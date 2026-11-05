//! Armijo backtracking (Armijo, 1966). Callback evaluates the supplied search ray.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteEvaluation,
    NonFiniteArithmetic,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Converged,
    BudgetExhausted,
    Stagnated,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Report {
    pub step: f64,
    pub value: f64,
    pub iterations: usize,
    pub evaluations: usize,
    pub status: Status,
}
/// Caller supplies the true value and negative directional derivative at zero,
/// and a deterministic callback. Converged means sampled sufficient decrease.
/// Budget counts contractions; at most budget+1 evaluations, including budget zero.
pub fn search<F: FnMut(f64) -> f64>(
    mut f: F,
    value_at_zero: f64,
    directional_derivative: f64,
    initial_step: f64,
    contraction: f64,
    sufficient_decrease: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    if !value_at_zero.is_finite()
        || !directional_derivative.is_finite()
        || directional_derivative >= 0.0
        || !initial_step.is_finite()
        || initial_step <= 0.0
        || !contraction.is_finite()
        || contraction <= 0.0
        || contraction >= 1.0
        || !sufficient_decrease.is_finite()
        || sufficient_decrease <= 0.0
        || sufficient_decrease >= 1.0
        || max_iterations > 100000
    {
        return Err(Error::InvalidInput);
    }
    let mut step = initial_step;
    for iterations in 0..=max_iterations {
        let decrement = sufficient_decrease * step * directional_derivative;
        let bound = value_at_zero + decrement;
        if !bound.is_finite() || !decrement.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        let value = f(step);
        if !value.is_finite() {
            return Err(Error::NonFiniteEvaluation);
        }
        let next = step * contraction;
        let status = if value <= bound && value < value_at_zero {
            Some(Status::Converged)
        } else if iterations == max_iterations {
            Some(Status::BudgetExhausted)
        } else if next == 0.0 || next == step {
            Some(Status::Stagnated)
        } else {
            None
        };
        if let Some(status) = status {
            return Ok(Report {
                step,
                value,
                iterations,
                evaluations: iterations + 1,
                status,
            });
        }
        step = next;
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
