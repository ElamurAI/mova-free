//! Forward/back substitution; Golub and Van Loan, Matrix Computations, fourth edition, section 3.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NotTriangular,
    ZeroDiagonal,
    AllocationFailed,
    NonFiniteArithmetic,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triangle {
    Lower,
    Upper,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub x: Vec<f64>,
    pub residual_norm: f64,
}
/// Nonempty square n<=64; finite magnitudes <=1e100. Exact structural zeros required.
/// Fresh absolute Euclidean residual is not a solution-error bound.
pub fn solve(a: &[Vec<f64>], b: &[f64], triangle: Triangle) -> Result<Report, Error> {
    let n = b.len();
    if n == 0
        || n > 64
        || a.len() != n
        || a.iter().any(|r| r.len() != n)
        || a.iter()
            .flatten()
            .chain(b)
            .any(|v| !v.is_finite() || v.abs() > 1e100)
    {
        return Err(Error::InvalidInput);
    }
    for (i, row) in a.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if ((triangle == Triangle::Lower && j > i) || (triangle == Triangle::Upper && j < i))
                && v != 0.0
            {
                return Err(Error::NotTriangular);
            }
        }
    }
    if (0..n).any(|i| a[i][i] == 0.0) {
        return Err(Error::ZeroDiagonal);
    }
    let mut x = buffer(n)?;
    for step in 0..n {
        let i = if triangle == Triangle::Lower {
            step
        } else {
            n - 1 - step
        };
        let mut s = b[i];
        for (j, &xj) in x.iter().enumerate() {
            if (triangle == Triangle::Lower && j < i) || (triangle == Triangle::Upper && j > i) {
                s = finite(s - a[i][j] * xj)?;
            }
        }
        x[i] = finite(s / a[i][i])?;
    }
    let residual_norm = residual(a, b, &x)?;
    Ok(Report { x, residual_norm })
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
