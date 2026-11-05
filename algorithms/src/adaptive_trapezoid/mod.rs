//! Dyadic adaptive trapezoid; Davis and Rabinowitz, Methods of Numerical Integration.
//! Global refinement, cached samples, O(evaluations) time and O(1) storage.
//! Consecutive difference /3 is an estimator, never an arbitrary-function guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteCallback,
    NonFiniteArithmetic,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    EstimatedTolerance,
    Budget,
    Stagnation,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Report {
    pub value: f64,
    pub estimated_error: Option<f64>,
    pub evaluations: usize,
    pub status: Status,
}
/// Calls a then b, then new midpoints left to right on each level. Invalid input
/// calls nothing. Nonfinite callback aborts immediately; callback panic propagates.
/// Budget funds whole levels only. A failed level returns no partial estimate.
pub fn integrate<F: FnMut(f64) -> f64>(
    mut f: F,
    a: f64,
    b: f64,
    tolerance: f64,
    max_evaluations: usize,
) -> Result<Report, Error> {
    let span = b - a;
    if !a.is_finite()
        || !b.is_finite()
        || a >= b
        || !span.is_finite()
        || !tolerance.is_finite()
        || tolerance <= 0.0
        || max_evaluations < 2
    {
        return Err(Error::InvalidInput);
    }
    let fa = f(a);
    if !fa.is_finite() {
        return Err(Error::NonFiniteCallback);
    }
    let fb = f(b);
    if !fb.is_finite() {
        return Err(Error::NonFiniteCallback);
    }
    let mut value = (fa * 0.5 + fb * 0.5) * span;
    if !value.is_finite() {
        return Err(Error::NonFiniteArithmetic);
    }
    let mut count = 2;
    let mut panels = 1usize;
    let mut error = None;
    loop {
        let report = |status| {
            Ok(Report {
                value,
                estimated_error: error,
                evaluations: count,
                status,
            })
        };
        if panels > max_evaluations - count {
            return report(Status::Budget);
        }
        let step = span / panels as f64;
        let mut sum = 0.0;
        let mut previous = a;
        // Validate all new abscissae before this level invokes the callback.
        for i in 0..panels {
            let x = a + (i as f64 + 0.5) * step;
            if !(previous < x && x < b) {
                return report(Status::Stagnation);
            }
            previous = x;
        }
        for i in 0..panels {
            let x = a + (i as f64 + 0.5) * step;
            let y = f(x);
            count += 1;
            if !y.is_finite() {
                return Err(Error::NonFiniteCallback);
            }
            sum += y;
            if !sum.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
        let refined = value * 0.5 + (step * 0.5) * sum;
        let delta = (refined - value).abs() / 3.0;
        if !refined.is_finite() || !delta.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        value = refined;
        error = Some(delta);
        if delta <= tolerance {
            return Ok(Report {
                value,
                estimated_error: error,
                evaluations: count,
                status: Status::EstimatedTolerance,
            });
        }
        match panels.checked_mul(2) {
            Some(next) => panels = next,
            None => {
                return Ok(Report {
                    value,
                    estimated_error: error,
                    evaluations: count,
                    status: Status::Budget,
                });
            }
        }
    }
}
#[cfg(test)]
mod tests;
