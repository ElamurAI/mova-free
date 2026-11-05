//! Lanczos (1950), An Iteration Method for the Solution of the Eigenvalue Problem.
//! Bounded symmetric tridiagonalization with two full modified Gram-Schmidt
//! reorthogonalization passes each step. Dropped roundoff corrections are measured.
pub use crate::lu_decomposition::Error;
use crate::lu_decomposition::{
    control, dot, finite, initial, matvec, norm, orthogonality, symmetric, validate,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Complete,
    Budget,
    Breakdown,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    /// Columns of Q, each stored as a vector.
    pub basis: Vec<Vec<f64>>,
    pub diagonal: Vec<f64>,
    /// k-1 nonnegative offdiagonals for k columns.
    pub off_diagonal: Vec<f64>,
    pub steps: usize,
    /// Fresh ||AQ-QT||F, including any terminal residual, not only a recurrence estimate.
    pub residual: f64,
    pub orthogonality_residual: f64,
    pub status: Status,
}
/// n=1..64, exact symmetric finite matrix and nonzero start, magnitudes<=1e100.
/// Absolute breakdown tolerance [0,1e100]; max_steps<=4096, capped by dimension.
/// Zero budget yields an empty projection (zero residual), not a convergence claim.
/// At a finished step: dimension completion precedes small-beta breakdown, then budget.
pub fn tridiagonalize(
    a: &[Vec<f64>],
    start: &[f64],
    tolerance: f64,
    max_steps: usize,
) -> Result<Report, Error> {
    let (n, _) = validate(a, true)?;
    control(tolerance, max_steps)?;
    symmetric(a)?;
    let mut q = initial(start, n)?;
    let limit = max_steps.min(n);
    let mut basis: Vec<Vec<f64>> = Vec::new();
    basis
        .try_reserve_exact(limit)
        .map_err(|_| Error::AllocationFailed)?;
    let mut diagonal = Vec::new();
    diagonal
        .try_reserve_exact(limit)
        .map_err(|_| Error::AllocationFailed)?;
    let mut off_diagonal = Vec::new();
    off_diagonal
        .try_reserve_exact(limit.saturating_sub(1))
        .map_err(|_| Error::AllocationFailed)?;
    let mut status = Status::Budget;
    for j in 0..limit {
        let mut w = matvec(a, &q)?;
        let alpha = dot(&q, &w)?;
        for i in 0..n {
            w[i] = finite(w[i] - alpha * q[i])?;
            if j > 0 {
                w[i] = finite(w[i] - off_diagonal[j - 1] * basis[j - 1][i])?;
            }
        }
        basis.push(q);
        diagonal.push(alpha);
        for _ in 0..2 {
            for v in &basis {
                let correction = dot(v, &w)?;
                for i in 0..n {
                    w[i] = finite(w[i] - correction * v[i])?;
                }
            }
        }
        let beta = norm(&w)?;
        if j + 1 == n {
            status = Status::Complete;
            break;
        }
        if beta <= tolerance {
            status = Status::Breakdown;
            break;
        }
        if j + 1 == limit {
            break;
        }
        off_diagonal.push(beta);
        for w in &mut w {
            *w = finite(*w / beta)?;
        }
        // Keep recurrence sign: beta is nonnegative; only initial vector is canonicalized.
        q = w;
    }
    let steps = basis.len();
    let mut residual = 0.0_f64;
    for j in 0..steps {
        let y = matvec(a, &basis[j])?;
        for i in 0..n {
            let mut t = finite(diagonal[j] * basis[j][i])?;
            if j > 0 {
                t = finite(t + off_diagonal[j - 1] * basis[j - 1][i])?;
            }
            if j + 1 < steps {
                t = finite(t + off_diagonal[j] * basis[j + 1][i])?;
            }
            residual = finite(residual.hypot(finite(y[i] - t)?))?;
        }
    }
    Ok(Report {
        orthogonality_residual: orthogonality(&basis)?,
        basis,
        diagonal,
        off_diagonal,
        steps,
        residual,
        status,
    })
}
#[cfg(test)]
mod tests;
