//! jacobi_iteration stationary iteration; Varga (1962), Matrix Iterative Analysis.
//! Convergence depends on the iteration matrix; no convergence assumption is silently imposed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    ZeroDiagonal,
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
/// n=1..64; finite magnitudes <=1e100; positive finite absolute residual tolerance; budget<=10000.
/// Fresh Euclidean residual stopping, then budget, then unchanged-iterate stagnation.
/// Residual tolerance does not certify solution accuracy without conditioning information.
pub fn solve(
    a: &[Vec<f64>],
    b: &[f64],
    initial: &[f64],
    residual_tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let n = b.len();
    if n == 0
        || n > 64
        || a.len() != n
        || initial.len() != n
        || a.iter().any(|r| r.len() != n)
        || a.iter()
            .flatten()
            .chain(b)
            .chain(initial)
            .any(|v| !v.is_finite() || v.abs() > 1e100)
        || !residual_tolerance.is_finite()
        || residual_tolerance <= 0.0
        || max_iterations > 10000
    {
        return Err(Error::InvalidInput);
    }
    if (0..n).any(|i| a[i][i] == 0.0) {
        return Err(Error::ZeroDiagonal);
    }
    let mut x = buffer(n)?;
    x.copy_from_slice(initial);
    let mut next = buffer(n)?;
    let mut unchanged = false;
    for iterations in 0..=max_iterations {
        let residual_norm = residual(a, b, &x)?;
        let status = if residual_norm <= residual_tolerance {
            Some(Status::Converged)
        } else if iterations == max_iterations {
            Some(Status::BudgetExhausted)
        } else if unchanged {
            Some(Status::Stagnated)
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
            let mut s = b[i];
            for (j, &xj) in x.iter().enumerate() {
                if i != j {
                    s = finite(s - a[i][j] * xj)?;
                }
            }
            next[i] = finite(s / a[i][i])?;
            changed |= next[i] != x[i];
        }
        std::mem::swap(&mut x, &mut next);
        unchanged = !changed;
    }
    unreachable!()
}
fn buffer(n: usize) -> Result<Vec<f64>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(n, 0.0);
    Ok(v)
}
fn finite(v: f64) -> Result<f64, Error> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(Error::NonFiniteArithmetic)
    }
}
fn residual(a: &[Vec<f64>], b: &[f64], x: &[f64]) -> Result<f64, Error> {
    let mut norm = 0.0_f64;
    for (row, &b) in a.iter().zip(b) {
        let mut s = 0.0;
        for (&a, &x) in row.iter().zip(x) {
            s = finite(s + a * x)?;
        }
        norm = finite(norm.hypot(finite(s - b)?))?;
    }
    Ok(norm)
}
#[cfg(test)]
mod tests;
