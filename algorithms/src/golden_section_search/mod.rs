//! Golden-section minimization; Kiefer (1953), Sequential minimax search for a maximum.
//! Caller assumes deterministic strict unimodality; finite sampling cannot check it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteEvaluation,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Converged,
    BudgetExhausted,
    Stagnated,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SearchReport {
    pub left: f64,
    pub right: f64,
    pub estimate: f64,
    pub value: f64,
    pub iterations: usize,
    pub status: Status,
}
/// Absolute interval width is the stopping metric, not an objective-error guarantee.
/// Finite endpoints bounded by 1e100; budget <=100000; at most budget+2 evaluations.
pub fn minimize<F: FnMut(f64) -> f64>(
    mut f: F,
    mut left: f64,
    mut right: f64,
    x_tolerance: f64,
    max_iterations: usize,
) -> Result<SearchReport, Error> {
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
    let ratio = (5.0_f64.sqrt() - 1.0) * 0.5;
    let mut c = right - ratio * (right - left);
    let mut d = left + ratio * (right - left);
    let mut fc = f(c);
    let mut fd = f(d);
    if !fc.is_finite() || !fd.is_finite() {
        return Err(Error::NonFiniteEvaluation);
    }
    for iterations in 0..=max_iterations {
        let (estimate, value) = if fc <= fd { (c, fc) } else { (d, fd) };
        let status = if right - left <= x_tolerance {
            Some(Status::Converged)
        } else if iterations == max_iterations {
            Some(Status::BudgetExhausted)
        } else if !(left < c && c < d && d < right) {
            Some(Status::Stagnated)
        } else {
            None
        };
        if let Some(status) = status {
            return Ok(SearchReport {
                left,
                right,
                estimate,
                value,
                iterations,
                status,
            });
        }
        if fc <= fd {
            right = d;
            d = c;
            fd = fc;
            c = right - ratio * (right - left);
            fc = f(c);
        } else {
            left = c;
            c = d;
            fc = fd;
            d = left + ratio * (right - left);
            fd = f(d);
        }
        if !fc.is_finite() || !fd.is_finite() {
            return Err(Error::NonFiniteEvaluation);
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
