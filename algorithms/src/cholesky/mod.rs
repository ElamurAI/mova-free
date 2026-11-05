//! Cholesky factorization; Golub and Van Loan, Matrix Computations, fourth edition, section 4.2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NotSymmetric,
    NonPositivePivot,
    AllocationFailed,
    NonFiniteArithmetic,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub lower: Vec<Vec<f64>>,
    pub reconstruction_residual: f64,
}
/// n=1..64, finite magnitudes <=1e100, exact symmetry; positive encountered pivots required.
/// Frobenius reconstruction residual is an observation, not a universal error certificate.
pub fn factor(a: &[Vec<f64>]) -> Result<Report, Error> {
    let n = a.len();
    if n == 0
        || n > 64
        || a.iter().any(|r| r.len() != n)
        || a.iter()
            .flatten()
            .any(|v| !v.is_finite() || v.abs() > 1e100)
    {
        return Err(Error::InvalidInput);
    }
    for (i, row) in a.iter().enumerate() {
        for (j, other) in a.iter().enumerate().take(i) {
            if row[j] != other[i] {
                return Err(Error::NotSymmetric);
            }
        }
    }
    let mut lower: Vec<Vec<f64>> = Vec::new();
    lower
        .try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    for _ in 0..n {
        lower.push(buffer(n)?);
    }
    for i in 0..n {
        for j in 0..=i {
            let mut s = a[i][j];
            for (&left, &right) in lower[i][..j].iter().zip(&lower[j][..j]) {
                s = finite(s - left * right)?;
            }
            if i == j {
                if s <= 0.0 {
                    return Err(Error::NonPositivePivot);
                }
                lower[i][j] = s.sqrt();
            } else {
                lower[i][j] = finite(s / lower[j][j])?;
            }
        }
    }
    let mut reconstruction_residual = 0.0_f64;
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for (&left, &right) in lower[i][..=i.min(j)].iter().zip(&lower[j][..=i.min(j)]) {
                s = finite(s + left * right)?;
            }
            reconstruction_residual = finite(reconstruction_residual.hypot(finite(s - a[i][j])?))?;
        }
    }
    Ok(Report {
        lower,
        reconstruction_residual,
    })
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
#[cfg(test)]
mod tests;
