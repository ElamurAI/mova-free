//! Symmetric Jacobi rotations; Jacobi (1846), Uber ein leichtes Verfahren ... secularen Storungen.
//! Distinct from the stationary Jacobi linear solver. Eigenpairs remain in rotation order.
pub use crate::lu_decomposition::Error;
use crate::lu_decomposition::{
    buffer, canonical, control, copy, dot, finite, identity, matrix, matvec, orthogonality,
    symmetric, validate,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Converged,
    Budget,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub eigenvalues: Vec<f64>,
    /// Row-major matrix with eigenvectors in columns; largest absolute coordinate is nonnegative.
    pub eigenvectors: Vec<Vec<f64>>,
    pub rotations: usize,
    pub residual: f64,
    pub orthogonality_residual: f64,
    pub status: Status,
}
/// Exact symmetry, square n=1..64, finite |entry|<=1e100, budget<=4096.
/// Stop on fresh ||AV-VD||F <= absolute tolerance or budget. Zero budget is valid.
pub fn solve(a: &[Vec<f64>], tolerance: f64, max_rotations: usize) -> Result<Report, Error> {
    let (n, _) = validate(a, true)?;
    control(tolerance, max_rotations)?;
    symmetric(a)?;
    let mut b = copy(a)?;
    let mut v = identity(n)?;
    for rotations in 0..=max_rotations {
        let mut columns = matrix(n, n)?;
        let mut eigenvalues = buffer(n)?;
        let mut residual = 0.0_f64;
        for j in 0..n {
            for i in 0..n {
                columns[j][i] = v[i][j];
            }
            canonical(&mut columns[j]);
            let y = matvec(a, &columns[j])?;
            eigenvalues[j] = dot(&columns[j], &y)?;
            for i in 0..n {
                residual = finite(residual.hypot(finite(y[i] - eigenvalues[j] * columns[j][i])?))?;
            }
        }
        if residual <= tolerance || rotations == max_rotations {
            for i in 0..n {
                for j in 0..n {
                    v[i][j] = columns[j][i];
                }
            }
            return Ok(Report {
                eigenvalues,
                eigenvectors: v,
                rotations,
                residual,
                orthogonality_residual: orthogonality(&columns)?,
                status: if residual <= tolerance {
                    Status::Converged
                } else {
                    Status::Budget
                },
            });
        }
        let mut p = 0;
        let mut q = 0;
        let mut largest = 0.0;
        for (i, row) in b.iter().enumerate() {
            for (j, &entry) in row.iter().enumerate().skip(i + 1) {
                if entry.abs() > largest {
                    largest = entry.abs();
                    p = i;
                    q = j;
                }
            }
        }
        // If rounded offdiagonals vanish but the true residual does not, retain the
        // iterate and consume the remaining bounded rotations without a false convergence.
        if largest == 0.0 {
            continue;
        }
        let delta = finite((b[q][q] - b[p][p]) * 0.5)?;
        let h = delta.hypot(b[p][q]);
        let t = if delta >= 0.0 {
            b[p][q] / (delta + h)
        } else {
            b[p][q] / (delta - h)
        };
        let c = 1.0 / (1.0 + t * t).sqrt();
        let s = t * c;
        let app = b[p][p];
        let aqq = b[q][q];
        let apq = b[p][q];
        b[p][p] = finite(app - t * apq)?;
        b[q][q] = finite(aqq + t * apq)?;
        b[p][q] = 0.0;
        b[q][p] = 0.0;
        for k in 0..n {
            if k != p && k != q {
                let x = b[k][p];
                let y = b[k][q];
                b[k][p] = finite(c * x - s * y)?;
                b[p][k] = b[k][p];
                b[k][q] = finite(s * x + c * y)?;
                b[q][k] = b[k][q];
            }
            let x = v[k][p];
            let y = v[k][q];
            v[k][p] = finite(c * x - s * y)?;
            v[k][q] = finite(s * x + c * y)?;
        }
    }
    unreachable!("bounded loop returns at budget")
}
#[cfg(test)]
mod tests;
