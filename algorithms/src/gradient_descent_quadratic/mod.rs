//! Fixed-step gradient descent for a bounded positive diagonal quadratic.
//! Nocedal and Wright, Numerical Optimization (2nd ed.), chapter 3.
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
    pub residual_norm: f64,
    pub iterations: usize,
    pub status: Status,
}
fn buffer(n: usize) -> Result<Vec<f64>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(n, 0.0);
    Ok(v)
}
/// Minimize `0.5 * x'Dx - linear'x` with step `1/max(diagonal)`.
/// Dimension 1..256; diagonal in [1e-100,1e100]; other entries bounded by 1e100.
/// Absolute Euclidean gradient norm stopping does not certify coordinate error.
pub fn minimize_diagonal(
    diagonal: &[f64],
    linear: &[f64],
    initial: &[f64],
    gradient_tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let n = diagonal.len();
    if n == 0
        || n > 256
        || linear.len() != n
        || initial.len() != n
        || diagonal
            .iter()
            .any(|d| !d.is_finite() || *d < 1e-100 || *d > 1e100)
        || linear
            .iter()
            .chain(initial)
            .any(|v| !v.is_finite() || v.abs() > 1e100)
        || !gradient_tolerance.is_finite()
        || gradient_tolerance <= 0.0
        || max_iterations > 100000
    {
        return Err(Error::InvalidInput);
    }
    let scale = diagonal.iter().copied().fold(0.0, f64::max);
    let mut x = buffer(n)?;
    x.copy_from_slice(initial);
    let mut gradient = buffer(n)?;
    for iterations in 0..=max_iterations {
        let mut residual_norm = 0.0_f64;
        for i in 0..n {
            gradient[i] = diagonal[i] * x[i] - linear[i];
            if !gradient[i].is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            residual_norm = residual_norm.hypot(gradient[i]);
        }
        if !residual_norm.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        let status = if residual_norm <= gradient_tolerance {
            Some(Status::Converged)
        } else if iterations == max_iterations {
            Some(Status::BudgetExhausted)
        } else {
            None
        };
        if let Some(status) = status {
            return Ok(Report {
                x,
                residual_norm,
                iterations,
                status,
            });
        }
        let mut changed = false;
        for i in 0..n {
            let next = x[i] - gradient[i] / scale;
            if !next.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            changed |= next != x[i];
            x[i] = next;
        }
        if !changed {
            return Ok(Report {
                x,
                residual_norm,
                iterations: iterations + 1,
                status: Status::Stagnated,
            });
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
