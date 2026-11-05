//! Householder least squares; Householder (1958), Unitary triangularization of a nonsymmetric matrix.
//! Original bounded implementation; unpivoted QR does not diagnose numerical rank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    RankDeficient,
    AllocationFailed,
    NonFiniteArithmetic,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub x: Vec<f64>,
    pub residual_norm: f64,
    pub normal_residual_norm: f64,
}
/// 1<=n<=64, n<=m<=128, finite magnitudes <=1e100.
/// Exact-zero encountered pivots are rejected; tiny pivots can yield inaccurate solutions.
/// Fresh primal and normal residuals are observations, not solution-error bounds.
pub fn least_squares(a: &[Vec<f64>], b: &[f64]) -> Result<Report, Error> {
    let m = a.len();
    let n = a.first().map_or(0, Vec::len);
    if n == 0
        || n > 64
        || m < n
        || m > 128
        || b.len() != m
        || a.iter().any(|r| r.len() != n)
        || a.iter()
            .flatten()
            .chain(b)
            .any(|v| !v.is_finite() || v.abs() > 1e100)
    {
        return Err(Error::InvalidInput);
    }
    let mut r = buffer(m * n)?;
    for i in 0..m {
        r[i * n..(i + 1) * n].copy_from_slice(&a[i]);
    }
    let mut c = buffer(m)?;
    c.copy_from_slice(b);
    let mut v = buffer(m)?;
    for k in 0..n {
        let mut norm = 0.0_f64;
        for i in k..m {
            norm = finite(norm.hypot(r[i * n + k]))?;
        }
        if norm == 0.0 {
            return Err(Error::RankDeficient);
        }
        let alpha = if r[k * n + k] >= 0.0 { -norm } else { norm };
        for i in k..m {
            v[i] = r[i * n + k];
        }
        v[k] = finite(v[k] - alpha)?;
        let mut vn = 0.0_f64;
        for &value in &v[k..] {
            vn = finite(vn.hypot(value))?;
        }
        if vn == 0.0 {
            return Err(Error::RankDeficient);
        }
        for value in &mut v[k..] {
            *value /= vn;
        }
        for j in k..n {
            let mut dot = 0.0;
            for i in k..m {
                dot = finite(dot + v[i] * r[i * n + j])?;
            }
            for i in k..m {
                r[i * n + j] = finite(r[i * n + j] - 2.0 * v[i] * dot)?;
            }
        }
        let mut dot = 0.0;
        for i in k..m {
            dot = finite(dot + v[i] * c[i])?;
        }
        for i in k..m {
            c[i] = finite(c[i] - 2.0 * v[i] * dot)?;
        }
        r[k * n + k] = alpha;
        for i in k + 1..m {
            r[i * n + k] = 0.0;
        }
    }
    let mut x = buffer(n)?;
    for i in (0..n).rev() {
        let mut s = c[i];
        for j in i + 1..n {
            s = finite(s - r[i * n + j] * x[j])?;
        }
        if r[i * n + i] == 0.0 {
            return Err(Error::RankDeficient);
        }
        x[i] = finite(s / r[i * n + i])?;
    }
    let residual_norm = residual(a, b, &x)?;
    for i in 0..m {
        let mut s = 0.0;
        for (j, &x) in x.iter().enumerate() {
            s = finite(s + a[i][j] * x)?;
        }
        c[i] = finite(s - b[i])?;
    }
    let mut normal_residual_norm = 0.0_f64;
    for (j, _) in x.iter().enumerate() {
        let mut s = 0.0;
        for i in 0..m {
            s = finite(s + a[i][j] * c[i])?;
        }
        normal_residual_norm = finite(normal_residual_norm.hypot(s))?;
    }
    Ok(Report {
        x,
        residual_norm,
        normal_residual_norm,
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
