//! Dense conjugate gradients, with fresh residual stopping.
//! Hestenes and Stiefel (1952), Methods of conjugate gradients for solving linear systems.
//! Caller guarantees positive definiteness. Symmetry and encountered curvature are checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NotSymmetric,
    NonPositiveCurvature,
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
fn dot(a: &[f64], b: &[f64]) -> Result<f64, Error> {
    let mut sum = 0.0;
    for (a, b) in a.iter().zip(b) {
        sum += a * b;
        if !sum.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
    }
    Ok(sum)
}
fn multiply(a: &[Vec<f64>], x: &[f64], out: &mut [f64]) -> Result<(), Error> {
    for (row, y) in a.iter().zip(out) {
        *y = dot(row, x)?;
    }
    Ok(())
}
fn norm(v: &[f64]) -> Result<f64, Error> {
    let n = v.iter().fold(0.0_f64, |n, x| n.hypot(*x));
    if n.is_finite() {
        Ok(n)
    } else {
        Err(Error::NonFiniteArithmetic)
    }
}
/// Dimension 1..256; all inputs finite with magnitude <=1e100; budget <=100000.
/// Absolute fresh Euclidean residual is a stopping observation, not a solution-error bound.
/// Positive curvature along visited directions alone does not prove SPD.
pub fn solve(
    matrix: &[Vec<f64>],
    rhs: &[f64],
    initial: &[f64],
    residual_tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let n = rhs.len();
    if n == 0
        || n > 256
        || matrix.len() != n
        || initial.len() != n
        || matrix.iter().any(|r| r.len() != n)
        || matrix
            .iter()
            .flatten()
            .chain(rhs)
            .chain(initial)
            .any(|v| !v.is_finite() || v.abs() > 1e100)
        || !residual_tolerance.is_finite()
        || residual_tolerance <= 0.0
        || max_iterations > 100000
    {
        return Err(Error::InvalidInput);
    }
    for (i, row) in matrix.iter().enumerate() {
        for (j, other) in matrix.iter().enumerate().take(i) {
            if row[j] != other[i] {
                return Err(Error::NotSymmetric);
            }
        }
    }
    let mut x = buffer(n)?;
    x.copy_from_slice(initial);
    let mut r = buffer(n)?;
    let mut p = buffer(n)?;
    let mut ap = buffer(n)?;
    multiply(matrix, &x, &mut ap)?;
    for i in 0..n {
        r[i] = rhs[i] - ap[i];
        if !r[i].is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        p[i] = r[i];
    }
    for iterations in 0..=max_iterations {
        let residual_norm = norm(&r)?;
        let status = if residual_norm <= residual_tolerance {
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
        let rr = dot(&r, &r)?;
        if rr == 0.0 {
            return Ok(Report {
                x,
                residual_norm,
                iterations,
                status: Status::Stagnated,
            });
        }
        multiply(matrix, &p, &mut ap)?;
        let curvature = dot(&p, &ap)?;
        if curvature <= 0.0 {
            return Err(Error::NonPositiveCurvature);
        }
        let alpha = rr / curvature;
        if !alpha.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        let mut changed = false;
        for i in 0..n {
            let next = x[i] + alpha * p[i];
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
        multiply(matrix, &x, &mut ap)?;
        for i in 0..n {
            r[i] = rhs[i] - ap[i];
            if !r[i].is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
        let beta = dot(&r, &r)? / rr;
        if !beta.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        for i in 0..n {
            p[i] = r[i] + beta * p[i];
            if !p[i].is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests;
