//! Modified Gram-Schmidt; Bjorck, Numerical Methods for Least Squares Problems (1996), section 2.4.
//! One-pass projections, positive diagonal R; not a rank-revealing factorization.
pub use crate::lu_decomposition::Error;
use crate::lu_decomposition::{
    buffer, control, dot, finite, matrix, norm, orthogonality, validate,
};
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    /// Row-major thin Q (m by n).
    pub q: Vec<Vec<f64>>,
    pub r: Vec<Vec<f64>>,
    pub rank: usize,
    pub reconstruction_residual: f64,
    pub orthogonality_residual: f64,
}
/// 1<=n<=64, n<=m<=128, finite |entry|<=1e100; absolute tolerance in [0,1e100].
/// A column norm <= tolerance fails with its index. Rank equals n on success only.
pub fn factor(a: &[Vec<f64>], tolerance: f64) -> Result<Report, Error> {
    let (m, n) = validate(a, false)?;
    control(tolerance, 0)?;
    let mut columns = matrix(n, m)?;
    let mut r = matrix(n, n)?;
    for j in 0..n {
        let mut v = buffer(m)?;
        for i in 0..m {
            v[i] = a[i][j];
        }
        for k in 0..j {
            r[k][j] = dot(&columns[k], &v)?;
            for i in 0..m {
                v[i] = finite(v[i] - r[k][j] * columns[k][i])?;
            }
        }
        r[j][j] = norm(&v)?;
        if r[j][j] <= tolerance {
            return Err(Error::DependentColumn { index: j });
        }
        for i in 0..m {
            columns[j][i] = finite(v[i] / r[j][j])?;
        }
    }
    let orthogonality_residual = orthogonality(&columns)?;
    let mut q = matrix(m, n)?;
    let mut reconstruction_residual = 0.0_f64;
    for i in 0..m {
        for j in 0..n {
            q[i][j] = columns[j][i];
            let mut s = 0.0;
            for k in 0..n {
                s = finite(s + columns[k][i] * r[k][j])?;
            }
            reconstruction_residual = finite(reconstruction_residual.hypot(finite(s - a[i][j])?))?;
        }
    }
    Ok(Report {
        q,
        r,
        rank: n,
        reconstruction_residual,
        orthogonality_residual,
    })
}
#[cfg(test)]
mod tests;
