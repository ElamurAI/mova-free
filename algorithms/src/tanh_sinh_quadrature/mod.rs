//! Tanh-sinh quadrature; Takahasi and Mori (1974), Double exponential formulas.
//! Finite transform window [-3,3], dyadic trapezoid refinement without sample reuse.
//! Difference of levels estimates discretization only, NOT omitted tails or a guaranteed error.
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
/// Requires budget>=7. Each level samples ascending t in [-3,3], including
/// window endpoints with half weight; h starts at 1. All x are strictly interior.
/// Repeats old samples on each level. Validate geometry before callbacks.
/// Invalid input calls nothing; first nonfinite sample aborts; panics propagate.
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
        || max_evaluations < 7
    {
        return Err(Error::InvalidInput);
    }
    let half = span * 0.5;
    let mid = a + half;
    let mut panels = 6usize;
    let mut count = 0usize;
    let mut value = 0.0;
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
        if panels >= max_evaluations - count {
            return report(Status::Budget);
        }
        let h = 6.0 / panels as f64;
        let point = |i: usize| {
            let t = -3.0 + i as f64 * h;
            let u = std::f64::consts::FRAC_PI_2 * t.sinh();
            let c = u.cosh();
            (
                mid + half * u.tanh(),
                half * std::f64::consts::FRAC_PI_2 * t.cosh() / (c * c),
            )
        };
        let mut previous = a;
        for i in 0..=panels {
            let (x, w) = point(i);
            if !(previous < x && x < b) || !w.is_finite() {
                return report(Status::Stagnation);
            }
            previous = x;
        }
        let mut sum = 0.0;
        for i in 0..=panels {
            let (x, w) = point(i);
            let y = f(x);
            count += 1;
            if !y.is_finite() {
                return Err(Error::NonFiniteCallback);
            }
            let term = y * w * if i == 0 || i == panels { 0.5 } else { 1.0 };
            sum += term;
            if !term.is_finite() || !sum.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
        let refined = h * sum;
        if !refined.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        if count > panels + 1 {
            let delta = (refined - value).abs();
            if !delta.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            error = Some(delta);
        }
        value = refined;
        if error.is_some_and(|e| e <= tolerance) {
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
