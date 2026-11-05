//! Bounded bisection; Burden and Faires, Numerical Analysis, section on bisection.
//! Caller supplies a deterministic continuous function. Finite checks cannot establish continuity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NotBracketed,
    NonFiniteEvaluation,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Converged,
    BudgetExhausted,
    Stagnated,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootReport {
    pub left: f64,
    pub right: f64,
    pub estimate: f64,
    pub value: f64,
    pub iterations: usize,
    pub status: Status,
}
/// Absolute bracket-width stopping, or a computed zero. At most `max_iterations + 2` evaluations.
/// Endpoints must be ordered, finite and bounded by 1e100; budget at most 100000.
pub fn solve<F: FnMut(f64) -> f64>(
    mut f: F,
    mut left: f64,
    mut right: f64,
    x_tolerance: f64,
    max_iterations: usize,
) -> Result<RootReport, Error> {
    if !left.is_finite()
        || !right.is_finite()
        || left.abs() > 1e100
        || right.abs() > 1e100
        || left >= right
        || !x_tolerance.is_finite()
        || x_tolerance <= 0.0
        || max_iterations > 100000
    {
        return Err(Error::InvalidInput);
    }
    let mut fl = f(left);
    let fr = f(right);
    if !fl.is_finite() || !fr.is_finite() {
        return Err(Error::NonFiniteEvaluation);
    }
    if fl == 0.0 || fr == 0.0 {
        let (estimate, value) = if fl == 0.0 { (left, fl) } else { (right, fr) };
        return Ok(RootReport {
            left: estimate,
            right: estimate,
            estimate,
            value,
            iterations: 0,
            status: Status::Converged,
        });
    }
    if fl.is_sign_positive() == fr.is_sign_positive() {
        return Err(Error::NotBracketed);
    }
    let mut estimate = left;
    let mut value = fl;
    for iterations in 0..=max_iterations {
        let status = if right - left <= x_tolerance {
            Some(Status::Converged)
        } else if iterations == max_iterations {
            Some(Status::BudgetExhausted)
        } else {
            None
        };
        if let Some(status) = status {
            return Ok(RootReport {
                left,
                right,
                estimate,
                value,
                iterations,
                status,
            });
        }
        let mid = left + (right - left) * 0.5;
        if mid == left || mid == right {
            return Ok(RootReport {
                left,
                right,
                estimate,
                value,
                iterations,
                status: Status::Stagnated,
            });
        }
        estimate = mid;
        value = f(mid);
        if !value.is_finite() {
            return Err(Error::NonFiniteEvaluation);
        }
        if value == 0.0 {
            return Ok(RootReport {
                left: mid,
                right: mid,
                estimate,
                value,
                iterations: iterations + 1,
                status: Status::Converged,
            });
        }
        if value.is_sign_positive() == fl.is_sign_positive() {
            left = mid;
            fl = value;
        } else {
            right = mid;
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
