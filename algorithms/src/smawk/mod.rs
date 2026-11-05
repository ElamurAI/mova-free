//! Aggarwal et al. (1987), Geometric applications of a matrix-searching algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    EmptyColumns,
    DimensionOverflow,
    BudgetExceeded,
    AllocationFailed,
}
fn vec_for<T>(n: usize) -> Result<Vec<T>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    Ok(v)
}
fn solve<F: FnMut(usize, usize) -> Result<i128, Error>>(
    rows: &[usize],
    cols: &[usize],
    out: &mut [usize],
    f: &mut F,
) -> Result<(), Error> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut reduced = vec_for(rows.len().min(cols.len()))?;
    for &c in cols {
        while let Some(&last) = reduced.last() {
            let row = rows[reduced.len() - 1];
            if f(row, c)? < f(row, last)? {
                reduced.pop();
            } else {
                break;
            }
        }
        if reduced.len() < rows.len() {
            reduced.push(c);
        }
    }
    let mut odd = vec_for(rows.len() / 2)?;
    odd.extend(rows.iter().skip(1).step_by(2).copied());
    solve(&odd, &reduced, out, f)?;
    let mut start = 0;
    for i in (0..rows.len()).step_by(2) {
        let end = if i + 1 < rows.len() {
            let mut e = start;
            while reduced[e] != out[rows[i + 1]] {
                e += 1;
            }
            e
        } else {
            reduced.len() - 1
        };
        let mut best = start;
        let mut value = f(rows[i], reduced[start])?;
        for (j, &column) in reduced.iter().enumerate().take(end + 1).skip(start + 1) {
            let candidate = f(rows[i], column)?;
            if candidate < value {
                best = j;
                value = candidate;
            }
        }
        out[rows[i]] = reduced[best];
        start = end;
    }
    Ok(())
}
/// Leftmost row minima of a totally monotone matrix, in O(rows+cols) evaluations.
/// Caller must supply a deterministic, side-effect-free callback whose every
/// submatrix has nondecreasing leftmost row minima. This is assumed, not checked.
/// Zero rows returns empty even with zero columns. Nonempty rows require columns.
/// Dimensions precede allocation; budget is checked before each callback invocation.
/// Callback panics propagate. Budget failure exposes no partial answer.
pub fn row_minima<F: FnMut(usize, usize) -> i128>(
    rows: usize,
    cols: usize,
    max_evaluations: usize,
    mut f: F,
) -> Result<Vec<usize>, Error> {
    if rows == 0 {
        return Ok(Vec::new());
    }
    if cols == 0 {
        return Err(Error::EmptyColumns);
    }
    if rows.max(cols) > isize::MAX as usize / std::mem::size_of::<usize>() {
        return Err(Error::DimensionOverflow);
    }
    let mut rr = vec_for(rows)?;
    rr.extend(0..rows);
    let mut cc = vec_for(cols)?;
    cc.extend(0..cols);
    let mut out = vec_for(rows)?;
    out.resize(rows, 0);
    let mut calls = 0usize;
    solve(&rr, &cc, &mut out, &mut |r, c| {
        if calls == max_evaluations {
            return Err(Error::BudgetExceeded);
        }
        calls += 1;
        Ok(f(r, c))
    })?;
    Ok(out)
}
#[cfg(test)]
mod tests;
