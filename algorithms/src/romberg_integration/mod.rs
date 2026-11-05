//! Nested trapezoids with Richardson extrapolation; Romberg (1955),
//! Det Kongelige Norske Videnskabers Selskab Forhandlinger 28, 30–36.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Configuration,
    NonFinite,
    Domain,
    Callback,
    Arithmetic,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Converged,
    WorkBudget,
    Stagnated,
    LevelLimit,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Report {
    pub value: f64,
    pub estimated_error: Option<f64>,
    pub evaluations: usize,
    pub levels: usize,
    pub status: Status,
}
/// Bounds |a|,|b|<=1e100, a<=b, tolerance finite positive; levels 1..=20,
/// evaluation budget >=2. Callback deterministic, finite |f|<=1e100; panic
/// propagates. Fixed stack, no allocation. Difference of successive diagonals
/// is only an absolute heuristic; Converged is NOT a certified error bound.
pub fn integrate<F: FnMut(f64) -> f64>(
    mut f: F,
    a: f64,
    b: f64,
    tolerance: f64,
    max_levels: usize,
    max_evaluations: usize,
) -> Result<Report, Error> {
    if !(1..=20).contains(&max_levels)
        || max_evaluations < 2
        || !tolerance.is_finite()
        || tolerance <= 0.0
    {
        return Err(Error::Configuration);
    }
    if !a.is_finite() || !b.is_finite() {
        return Err(Error::NonFinite);
    }
    if a > b || a.abs() > 1e100 || b.abs() > 1e100 {
        return Err(Error::Domain);
    }
    let mut r = Report {
        value: 0.0,
        estimated_error: None,
        evaluations: 0,
        levels: 0,
        status: Status::Converged,
    };
    if a == b {
        r.estimated_error = Some(0.0);
        return Ok(r);
    }
    let mut sample = |x| {
        let y = f(x);
        if y.is_finite() && y.abs() <= 1e100 {
            Ok(y)
        } else {
            Err(Error::Callback)
        }
    };
    let mut row = [0.0; 20];
    row[0] = (b - a) * (sample(a)? * 0.5 + sample(b)? * 0.5);
    r.value = row[0];
    r.evaluations = 2;
    r.levels = 1;
    r.status = Status::LevelLimit;
    for k in 1..max_levels {
        let added = 1usize << (k - 1);
        if added > max_evaluations - r.evaluations {
            r.status = Status::WorkBudget;
            return Ok(r);
        }
        let h = (b - a) / (2 * added) as f64;
        let mut sum = 0.0;
        let mut previous = a;
        for i in 0..added {
            let x = a + (2 * i + 1) as f64 * h;
            if x <= previous || x >= b {
                r.status = Status::Stagnated;
                return Ok(r);
            }
            sum += sample(x)?;
            r.evaluations += 1;
            previous = x;
        }
        let mut next = 0.5 * row[0] + h * sum;
        let mut factor = 4.0;
        for cell in row.iter_mut().take(k) {
            let old = *cell;
            *cell = next;
            next += (next - old) / (factor - 1.0);
            factor *= 4.0;
        }
        if !next.is_finite() {
            return Err(Error::Arithmetic);
        }
        row[k] = next;
        let difference = (next - r.value).abs();
        r.value = next;
        r.estimated_error = Some(difference);
        r.levels = k + 1;
        if difference <= tolerance {
            r.status = Status::Converged;
            return Ok(r);
        }
    }
    Ok(r)
}
#[cfg(test)]
mod tests;
