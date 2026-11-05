//! Euclidean L1-ball projection via magnitude simplex projection.
//! Reference: Duchi et al., ICML (2008), Efficient Projections onto the l1-Ball.
pub use crate::simplex_projection::ProjectionError;
/// Return input inside the ball, otherwise project magnitudes and restore signs.
/// A nonfinite accumulated norm returns Arithmetic, conservatively. The simplex
/// mass tolerance also applies here; rounded outputs may exceed radius slightly.
pub fn project_l1_ball(input: &[f64], radius: f64) -> Result<Vec<f64>, ProjectionError> {
    crate::simplex_projection::validate(input, radius)?;
    let mut magnitudes = Vec::new();
    magnitudes
        .try_reserve_exact(input.len())
        .map_err(|_| ProjectionError::AllocationFailed)?;
    if radius == 0.0 {
        magnitudes.resize(input.len(), 0.0);
        return Ok(magnitudes);
    }
    let mut norm = 0.0;
    for &x in input {
        norm += x.abs();
        magnitudes.push(x.abs());
    }
    if !norm.is_finite() {
        return Err(ProjectionError::Arithmetic);
    }
    if norm <= radius {
        magnitudes.copy_from_slice(input);
        return Ok(magnitudes);
    }
    let mut out = crate::simplex_projection::project_simplex(&magnitudes, radius)?;
    for (x, &original) in out.iter_mut().zip(input) {
        *x = x.copysign(original);
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
