//! Safeguarded parabolic/golden minimization. Brent (1973), chapter 5.
//! Original implementation; deterministic strictly unimodal callback assumed.
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
    pub left: f64,
    pub right: f64,
    pub estimate: f64,
    pub value: f64,
    pub iterations: usize,
    pub evaluations: usize,
    pub status: Status,
}
/// Absolute bracket width stopping; no objective or solution relative-error bound.
/// Endpoints bounded by 1e50, budget <=100000; at most budget+1 callbacks.
pub fn minimize<F: FnMut(f64) -> f64>(
    mut f: F,
    mut left: f64,
    mut right: f64,
    x_tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    if !left.is_finite()
        || !right.is_finite()
        || left.abs() > 1e50
        || right.abs() > 1e50
        || left >= right
        || !x_tolerance.is_finite()
        || x_tolerance <= 0.0
        || max_iterations > 100000
    {
        return Err(Error::InvalidInput);
    }
    let golden = (3.0 - 5.0_f64.sqrt()) / 2.0;
    let mut x = left + golden * (right - left);
    let mut w = x;
    let mut v = x;
    let mut fx = f(x);
    if !fx.is_finite() {
        return Err(Error::NonFiniteEvaluation);
    }
    let mut fw = fx;
    let mut fv = fx;
    let mut previous = right - left;
    for iterations in 0..=max_iterations {
        let finish = |status| {
            Ok(Report {
                left,
                right,
                estimate: x,
                value: fx,
                iterations,
                evaluations: iterations + 1,
                status,
            })
        };
        if right - left <= x_tolerance {
            return finish(Status::Converged);
        }
        if iterations == max_iterations {
            return finish(Status::BudgetExhausted);
        }
        let midpoint = left + (right - left) / 2.0;
        let minimum = (x_tolerance / 8.0).max(f64::EPSILON * x.abs());
        let mut delta = if x < midpoint {
            golden * (right - x)
        } else {
            golden * (left - x)
        };
        if x != w && x != v && w != v {
            let a = (x - w) * (fx - fv);
            let b = (x - v) * (fx - fw);
            let numerator = (x - v) * b - (x - w) * a;
            let denominator = 2.0 * (b - a);
            if !a.is_finite()
                || !b.is_finite()
                || !numerator.is_finite()
                || !denominator.is_finite()
            {
                return Err(Error::NonFiniteArithmetic);
            }
            if denominator != 0.0 {
                let p = numerator / denominator;
                if p.is_finite()
                    && p.abs() < 0.5 * previous.abs()
                    && x + p > left + minimum
                    && x + p < right - minimum
                {
                    delta = p;
                }
            }
        }
        previous = delta;
        let direction = if delta == 0.0 { midpoint - x } else { delta };
        let u = x + delta.abs().max(minimum).copysign(direction);
        if !(left < u && u < right) || u == x {
            return finish(Status::Stagnated);
        }
        let fu = f(u);
        if !fu.is_finite() {
            return Err(Error::NonFiniteEvaluation);
        }
        if fu <= fx {
            if u < x {
                right = x
            } else {
                left = x
            }
            v = w;
            fv = fw;
            w = x;
            fw = fx;
            x = u;
            fx = fu;
        } else {
            if u < x {
                left = u
            } else {
                right = u
            }
            if fu <= fw || w == x {
                v = w;
                fv = fw;
                w = u;
                fw = fu;
            } else if fu <= fv || v == x || v == w {
                v = u;
                fv = fu;
            }
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
