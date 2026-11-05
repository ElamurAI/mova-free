//! Projected gradient for box-constrained quadratics; Bertsekas, Nonlinear
//! Programming (1999). Symmetric diagonally dominant PSD matrices only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    AllocationFailed,
    NonFiniteArithmetic,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Converged,
    BudgetExhausted,
    Stagnated,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub x: Vec<f64>,
    pub value: f64,
    pub residual: f64,
    pub iterations: usize,
    pub status: Status,
}
pub(crate) fn buffer(n: usize) -> Result<Vec<f64>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(n, 0.0);
    Ok(v)
}
pub(crate) fn bounded(v: f64) -> bool {
    v.is_finite() && v.abs() <= 1e50
}
/// Returns an absolute row-sum Lipschitz bound. Exact symmetry and conservative
/// diagonal dominance are checked; general PSD matrices may be rejected.
pub(crate) fn validate(
    q: &[Vec<f64>],
    b: &[f64],
    x: &[f64],
    tol: f64,
    budget: usize,
) -> Result<f64, Error> {
    let n = b.len();
    if n == 0
        || n > 64
        || q.len() != n
        || x.len() != n
        || q.iter()
            .any(|r| r.len() != n || r.iter().any(|&v| !bounded(v)))
        || b.iter().chain(x).any(|&v| !bounded(v))
        || !tol.is_finite()
        || tol <= 0.0
        || budget > 100000
    {
        return Err(Error::InvalidInput);
    }
    let mut bound = 0.0_f64;
    for (i, row) in q.iter().enumerate() {
        let mut off = 0.0;
        for (j, other_row) in q.iter().enumerate() {
            if row[j] != other_row[i] {
                return Err(Error::InvalidInput);
            }
            if i != j {
                off += q[i][j].abs();
            }
        }
        // Roundoff safety margin excludes borderline dense dominance. Diagonal-only
        // and exactly zero matrices remain accepted.
        if q[i][i] < 0.0 || (off > 0.0 && q[i][i] < off * (1.0 + 128.0 * f64::EPSILON)) {
            return Err(Error::InvalidInput);
        }
        bound = bound.max(q[i][i] + off);
    }
    Ok(bound.max(1e-100))
}
pub(crate) fn evaluate(q: &[Vec<f64>], b: &[f64], x: &[f64], g: &mut [f64]) -> Result<f64, Error> {
    let mut value = 0.0;
    for i in 0..x.len() {
        let dot: f64 = q[i].iter().zip(x).map(|(a, v)| a * v).sum();
        g[i] = dot - b[i];
        value += x[i] * (0.5 * dot - b[i]);
    }
    if !value.is_finite() || g.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFiniteArithmetic);
    }
    Ok(value)
}
/// Minimize .5*x'Q*x-linear'x. Dimension 1..64, magnitude <=1e50.
/// Initial point is clipped into the box. Residual is the infinity norm of the
/// projected step with step 1/L, not distance to an optimizer. Zero Q uses L=1e-100.
#[allow(clippy::too_many_arguments)]
pub fn minimize(
    matrix: &[Vec<f64>],
    linear: &[f64],
    lower: &[f64],
    upper: &[f64],
    initial: &[f64],
    tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let scale = validate(matrix, linear, initial, tolerance, max_iterations)?;
    let n = linear.len();
    if lower.len() != n
        || upper.len() != n
        || lower
            .iter()
            .zip(upper)
            .any(|(&a, &b)| !bounded(a) || !bounded(b) || a > b)
    {
        return Err(Error::InvalidInput);
    }
    let mut x = buffer(n)?;
    for i in 0..n {
        x[i] = initial[i].clamp(lower[i], upper[i]);
    }
    let mut gradient = buffer(n)?;
    let mut next = buffer(n)?;
    for iterations in 0..=max_iterations {
        let value = evaluate(matrix, linear, &x, &mut gradient)?;
        let mut residual = 0.0_f64;
        for i in 0..n {
            let raw = x[i] - gradient[i] / scale;
            if !raw.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            next[i] = raw.clamp(lower[i], upper[i]);
            residual = residual.max((x[i] - next[i]).abs());
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
        // A nonzero residual normally implies a changed iterate. This explicit guard
        // also documents representational stagnation separately from iteration budget.
        if x == next {
            return Ok(Report {
                x,
                value,
                residual,
                iterations,
                status: Status::Stagnated,
            });
        }
        std::mem::swap(&mut x, &mut next);
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
