//! Sinkhorn and Knopp (1967), Concerning nonnegative matrices and doubly
//! stochastic matrices. Alternating positive diagonal scaling with a budget.
//! Residual certifies only observed marginal discrepancy, not optimality or
//! distance to a limiting matrix. Extreme underflow/overflow is an error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Dimension,
    Configuration,
    Value,
    MarginalMass,
    Arithmetic,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Scaling {
    pub matrix: Vec<Vec<f64>>,
    pub residual: f64,
    pub iterations: usize,
    pub converged: bool,
}
pub fn sinkhorn_scaling(
    matrix: &[Vec<f64>],
    rows: &[f64],
    columns: &[f64],
    tolerance: f64,
    max_iterations: usize,
) -> Result<Scaling, Error> {
    let r = rows.len();
    let c = columns.len();
    if r == 0
        || c == 0
        || r > 64
        || c > 64
        || matrix.len() != r
        || matrix.iter().any(|row| row.len() != c)
    {
        return Err(Error::Dimension);
    }
    if !tolerance.is_finite() || tolerance <= 0.0 || max_iterations > 10000 {
        return Err(Error::Configuration);
    }
    if matrix
        .iter()
        .flatten()
        .chain(rows)
        .chain(columns)
        .any(|x| !x.is_finite() || !(1e-100..=1e100).contains(x))
    {
        return Err(Error::Value);
    }
    let a: f64 = rows.iter().sum();
    let b: f64 = columns.iter().sum();
    if (a - b).abs() > 1e-12 * a.max(b) {
        return Err(Error::MarginalMass);
    }
    let mut x = matrix.to_vec();
    for iteration in 0..=max_iterations {
        let residual = residual(&x, rows, columns);
        if residual <= tolerance || iteration == max_iterations {
            return Ok(Scaling {
                matrix: x,
                residual,
                iterations: iteration,
                converged: residual <= tolerance,
            });
        }
        for (i, row) in x.iter_mut().enumerate() {
            let scale = rows[i] / row.iter().sum::<f64>();
            for value in row {
                *value *= scale;
            }
        }
        for (j, &target) in columns.iter().enumerate() {
            let sum: f64 = x.iter().map(|row| row[j]).sum();
            let scale = target / sum;
            for row in &mut x {
                row[j] *= scale;
            }
        }
        if x.iter().flatten().any(|v| !v.is_finite() || *v <= 0.0) {
            return Err(Error::Arithmetic);
        }
    }
    unreachable!()
}
fn residual(x: &[Vec<f64>], rows: &[f64], columns: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for (row, &target) in x.iter().zip(rows) {
        result = result.max((row.iter().sum::<f64>() - target).abs());
    }
    for (j, &target) in columns.iter().enumerate() {
        result = result.max((x.iter().map(|r| r[j]).sum::<f64>() - target).abs());
    }
    result
}
#[cfg(test)]
mod tests;
