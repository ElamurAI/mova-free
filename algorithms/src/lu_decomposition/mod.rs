//! Partial-pivot LU; Golub and Van Loan, Matrix Computations, 4th ed., section 3.4.
//! Original bounded implementation; residuals are observations, not error guarantees.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NotSymmetric,
    SingularPivot { index: usize },
    DependentColumn { index: usize },
    ZeroInitialVector,
    AllocationFailed,
    NonFiniteArithmetic,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub lower: Vec<Vec<f64>>,
    pub upper: Vec<Vec<f64>>,
    /// Row i of PA is row permutation[i] of A.
    pub permutation: Vec<usize>,
    pub reconstruction_residual: f64,
}
/// Square n=1..64, finite |entry|<=1e100. Earliest maximal pivot wins ties.
/// Exactly zero pivots fail; tiny nonzero pivots are not a conditioning certificate.
pub fn factor(a: &[Vec<f64>]) -> Result<Report, Error> {
    let (n, _) = validate(a, true)?;
    let mut upper = copy(a)?;
    let mut lower = identity(n)?;
    let mut permutation = Vec::new();
    permutation
        .try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    permutation.extend(0..n);
    for k in 0..n {
        let mut p = k;
        for i in k + 1..n {
            if upper[i][k].abs() > upper[p][k].abs() {
                p = i;
            }
        }
        if upper[p][k] == 0.0 {
            return Err(Error::SingularPivot { index: k });
        }
        upper.swap(k, p);
        permutation.swap(k, p);
        if p != k {
            let (left, right) = lower.split_at_mut(p);
            for (x, y) in left[k][..k].iter_mut().zip(&mut right[0][..k]) {
                std::mem::swap(x, y);
            }
        }
        for i in k + 1..n {
            lower[i][k] = finite(upper[i][k] / upper[k][k])?;
            upper[i][k] = 0.0;
            let (before, after) = upper.split_at_mut(i);
            for (x, &pivot) in after[0][k + 1..].iter_mut().zip(&before[k][k + 1..]) {
                *x = finite(*x - lower[i][k] * pivot)?;
            }
        }
    }
    let mut reconstruction_residual = 0.0_f64;
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..n {
                s = finite(s + lower[i][k] * upper[k][j])?;
            }
            reconstruction_residual =
                finite(reconstruction_residual.hypot(finite(s - a[permutation[i]][j])?))?;
        }
    }
    Ok(Report {
        lower,
        upper,
        permutation,
        reconstruction_residual,
    })
}
pub(crate) fn finite(v: f64) -> Result<f64, Error> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(Error::NonFiniteArithmetic)
    }
}
pub(crate) fn buffer(n: usize) -> Result<Vec<f64>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(n, 0.0);
    Ok(v)
}
pub(crate) fn matrix(m: usize, n: usize) -> Result<Vec<Vec<f64>>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(m)
        .map_err(|_| Error::AllocationFailed)?;
    for _ in 0..m {
        v.push(buffer(n)?);
    }
    Ok(v)
}
pub(crate) fn copy(a: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, Error> {
    let mut b = matrix(a.len(), a[0].len())?;
    for (b, a) in b.iter_mut().zip(a) {
        b.copy_from_slice(a);
    }
    Ok(b)
}
pub(crate) fn identity(n: usize) -> Result<Vec<Vec<f64>>, Error> {
    let mut a = matrix(n, n)?;
    for (i, row) in a.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    Ok(a)
}
pub(crate) fn validate(a: &[Vec<f64>], square: bool) -> Result<(usize, usize), Error> {
    let m = a.len();
    let n = a.first().map_or(0, Vec::len);
    if n == 0
        || n > 64
        || m < n
        || m > 128
        || (square && m != n)
        || a.iter().any(|r| r.len() != n)
        || a.iter()
            .flatten()
            .any(|x| !x.is_finite() || x.abs() > 1e100)
    {
        return Err(Error::InvalidInput);
    }
    Ok((m, n))
}
pub(crate) fn symmetric(a: &[Vec<f64>]) -> Result<(), Error> {
    for (i, row) in a.iter().enumerate() {
        for j in 0..i {
            if row[j] != a[j][i] {
                return Err(Error::NotSymmetric);
            }
        }
    }
    Ok(())
}
pub(crate) fn control(tolerance: f64, budget: usize) -> Result<(), Error> {
    if !tolerance.is_finite() || !(0.0..=1e100).contains(&tolerance) || budget > 4096 {
        Err(Error::InvalidInput)
    } else {
        Ok(())
    }
}
pub(crate) fn dot(x: &[f64], y: &[f64]) -> Result<f64, Error> {
    let mut s = 0.0;
    for (&x, &y) in x.iter().zip(y) {
        s = finite(s + x * y)?;
    }
    Ok(s)
}
pub(crate) fn norm(x: &[f64]) -> Result<f64, Error> {
    let mut s = 0.0_f64;
    for &x in x {
        s = finite(s.hypot(x))?;
    }
    Ok(s)
}
pub(crate) fn matvec(a: &[Vec<f64>], x: &[f64]) -> Result<Vec<f64>, Error> {
    let mut y = buffer(a.len())?;
    for (y, row) in y.iter_mut().zip(a) {
        *y = dot(row, x)?;
    }
    Ok(y)
}
pub(crate) fn canonical(x: &mut [f64]) {
    let mut p = 0;
    for i in 1..x.len() {
        if x[i].abs() > x[p].abs() {
            p = i;
        }
    }
    if x[p] < 0.0 {
        for v in x {
            *v = -*v;
        }
    }
}
pub(crate) fn initial(x: &[f64], n: usize) -> Result<Vec<f64>, Error> {
    if x.len() != n || x.iter().any(|v| !v.is_finite() || v.abs() > 1e100) {
        return Err(Error::InvalidInput);
    }
    let s = norm(x)?;
    if s == 0.0 {
        return Err(Error::ZeroInitialVector);
    }
    let mut y = buffer(n)?;
    for (y, x) in y.iter_mut().zip(x) {
        *y = *x / s;
    }
    canonical(&mut y);
    Ok(y)
}
/// Basis vectors are stored by column.
pub(crate) fn orthogonality(q: &[Vec<f64>]) -> Result<f64, Error> {
    let mut s = 0.0_f64;
    for (i, x) in q.iter().enumerate() {
        for (j, y) in q.iter().enumerate() {
            s = finite(s.hypot(finite(dot(x, y)? - if i == j { 1.0 } else { 0.0 })?))?;
        }
    }
    Ok(s)
}
#[cfg(test)]
mod tests;
