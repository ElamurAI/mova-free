//! Adaptive Simpson quadrature; see Lyness (1969), JACM 16, 483–495.
//! The Richardson estimate is heuristic, without a smoothness or derivative bound.
//! A deterministic callback must return finite values. Callback panics are not caught.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    EstimatedTolerance,
    EvaluationBudget,
    DepthBudget,
    Stagnation,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Report {
    pub value: f64,
    /// Sum of accepted estimates; f64::MAX means unknown after any unresolved panel.
    pub estimated_error: f64,
    pub evaluations: usize,
    pub status: Status,
}
#[derive(Clone, Copy)]
struct Panel {
    a: f64,
    b: f64,
    fa: f64,
    fm: f64,
    fb: f64,
    coarse: f64,
    tol: f64,
    depth: usize,
}
fn finite(v: f64) -> Result<f64, Error> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(Error::NonFiniteArithmetic)
    }
}
fn midpoint(a: f64, b: f64) -> f64 {
    a * 0.5 + b * 0.5
}
fn simpson(a: f64, b: f64, fa: f64, fm: f64, fb: f64) -> Result<f64, Error> {
    finite((b - a) * (fa / 6.0 + fm * (2.0 / 3.0) + fb / 6.0))
}
/// Ordered finite endpoints; positive absolute tolerance; depth <=64; budget >=3.
/// Zero-width interval returns zero without evaluating the callback.
/// Budget or stagnation reports retain unresolved coarse panels, with unknown error.
pub fn integrate<F: FnMut(f64) -> f64>(
    mut f: F,
    a: f64,
    b: f64,
    tolerance: f64,
    max_evaluations: usize,
    max_depth: usize,
) -> Result<Report, Error> {
    if !a.is_finite()
        || !b.is_finite()
        || a > b
        || !tolerance.is_finite()
        || tolerance <= 0.0
        || max_evaluations < 3
        || max_depth > 64
    {
        return Err(Error::InvalidInput);
    }
    let mut report = Report {
        value: 0.0,
        estimated_error: 0.0,
        evaluations: 0,
        status: Status::EstimatedTolerance,
    };
    if a == b {
        return Ok(report);
    }
    finite(b - a)?;
    let m = midpoint(a, b);
    let fa = finite(f(a))?;
    let fm = finite(f(m))?;
    let fb = finite(f(b))?;
    report.evaluations = 3;
    // At most depth+1 pending panels: bounded stack allocation, no heap allocation.
    let initial = Panel {
        a,
        b,
        fa,
        fm,
        fb,
        coarse: simpson(a, b, fa, fm, fb)?,
        tol: tolerance,
        depth: 0,
    };
    let mut stack = [initial; 65];
    let mut len = 1;
    while len > 0 {
        len -= 1;
        let p = stack[len];
        let m = midpoint(p.a, p.b);
        let l = midpoint(p.a, m);
        let r = midpoint(m, p.b);
        let stopped = if !(p.a < l && l < m && m < r && r < p.b) {
            Some(Status::Stagnation)
        } else if max_evaluations - report.evaluations < 2 {
            Some(Status::EvaluationBudget)
        } else {
            None
        };
        if let Some(status) = stopped {
            if report.status == Status::EstimatedTolerance {
                report.status = status;
            }
            report.value = finite(report.value + p.coarse)?;
            report.estimated_error = f64::MAX;
            continue;
        }
        let fl = finite(f(l))?;
        let fr = finite(f(r))?;
        report.evaluations += 2;
        let left = simpson(p.a, m, p.fa, fl, p.fm)?;
        let right = simpson(m, p.b, p.fm, fr, p.fb)?;
        let refined = finite(left + right)?;
        let delta = finite(refined - p.coarse)?;
        let error = delta.abs() / 15.0;
        if error <= p.tol {
            report.value = finite(report.value + finite(refined + delta / 15.0)?)?;
            if report.estimated_error != f64::MAX {
                report.estimated_error = finite(report.estimated_error + error)?;
            }
        } else if p.depth == max_depth || p.tol * 0.5 == 0.0 {
            if report.status == Status::EstimatedTolerance {
                report.status = if p.depth == max_depth {
                    Status::DepthBudget
                } else {
                    Status::Stagnation
                };
            }
            report.value = finite(report.value + refined)?;
            report.estimated_error = f64::MAX;
        } else {
            stack[len] = Panel {
                a: m,
                b: p.b,
                fa: p.fm,
                fm: fr,
                fb: p.fb,
                coarse: right,
                tol: p.tol * 0.5,
                depth: p.depth + 1,
            };
            len += 1;
            stack[len] = Panel {
                a: p.a,
                b: m,
                fa: p.fa,
                fm: fl,
                fb: p.fm,
                coarse: left,
                tol: p.tol * 0.5,
                depth: p.depth + 1,
            };
            len += 1;
        }
    }
    Ok(report)
}
#[cfg(test)]
mod tests;
