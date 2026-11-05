//! Power iteration; von Mises and Pollaczek-Geiringer (1929), Praktische Verfahren der Gleichungsauflosung.
//! A small residual does not imply the dominant eigenvalue: the start can miss its eigenspace.
pub use crate::lu_decomposition::Error;
use crate::lu_decomposition::{canonical, control, dot, finite, initial, matvec, norm, validate};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Converged,
    Budget,
    Breakdown,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub eigenvalue: f64,
    pub eigenvector: Vec<f64>,
    pub iterations: usize,
    pub residual: f64,
    pub status: Status,
}
/// Square n=1..64, finite |entries/start|<=1e100, nonzero start.
/// Absolute residual tolerance [0,1e100], max_iterations<=4096 (zero observes the start).
/// Stop precedence: converged residual, exhausted budget, zero-image breakdown.
pub fn solve(
    a: &[Vec<f64>],
    start: &[f64],
    tolerance: f64,
    max_iterations: usize,
) -> Result<Report, Error> {
    let (n, _) = validate(a, true)?;
    control(tolerance, max_iterations)?;
    let mut x = initial(start, n)?;
    for iterations in 0..=max_iterations {
        let mut y = matvec(a, &x)?;
        let eigenvalue = dot(&x, &y)?;
        let mut residual = 0.0_f64;
        for (&y, &x) in y.iter().zip(&x) {
            residual = finite(residual.hypot(finite(y - eigenvalue * x)?))?;
        }
        let s = norm(&y)?;
        let status = if residual <= tolerance {
            Some(Status::Converged)
        } else if iterations == max_iterations {
            Some(Status::Budget)
        } else if s == 0.0 {
            Some(Status::Breakdown)
        } else {
            None
        };
        if let Some(status) = status {
            return Ok(Report {
                eigenvalue,
                eigenvector: x,
                iterations,
                residual,
                status,
            });
        }
        for y in &mut y {
            *y = finite(*y / s)?;
        }
        canonical(&mut y);
        x = y;
    }
    unreachable!("bounded loop returns at budget")
}
#[cfg(test)]
mod tests;
