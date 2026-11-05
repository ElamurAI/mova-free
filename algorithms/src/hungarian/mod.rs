//! Rectangular minimum-cost assignment by primal-dual augmentations.
//! Original implementation; classical attribution: H. W. Kuhn, "The Hungarian
//! Method for the Assignment Problem", Naval Research Logistics Quarterly (1955).

pub const MAX_ROWS: usize = 512;
pub const MAX_COLUMNS: usize = 4096;
pub const MAX_WORK: usize = 20_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    /// Column selected for each row, with no repetitions.
    pub columns: Vec<usize>,
    pub cost: i128,
    /// Dual certificate: u[i] + v[j] <= costs[i][j], equality on selected edges.
    pub row_potentials: Vec<i128>,
    /// Nonpositive; unused columns have zero potential.
    pub column_potentials: Vec<i128>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooManyRows,
    TooManyColumns,
    RaggedMatrix,
    TooFewColumns,
    WorkLimit,
    AllocationFailed,
}

/// Assign every row to a distinct column with exact minimum signed cost.
/// Empty input has empty assignment and zero cost. Error precedence is row cap,
/// first-row column cap, rectangularity, rows <= columns, then work/allocation.
/// Ascending rows/columns and first strict slack improvement fix ties; the
/// witness is deterministic but not promised lexicographically smallest.
/// Time O(r^2*c), auxiliary space O(r+c); r*r*c must not exceed MAX_WORK.
/// Capped dimensions and i64 inputs bound all intermediate values well inside
/// i128; objective/potential arithmetic is therefore exact without saturation.
pub fn hungarian(costs: &[Vec<i64>]) -> Result<Assignment, Error> {
    let rows = costs.len();
    if rows > MAX_ROWS {
        return Err(Error::TooManyRows);
    }
    let cols = costs.first().map_or(0, Vec::len);
    if cols > MAX_COLUMNS {
        return Err(Error::TooManyColumns);
    }
    if costs.iter().any(|row| row.len() != cols) {
        return Err(Error::RaggedMatrix);
    }
    if rows > cols {
        return Err(Error::TooFewColumns);
    }
    if rows
        .checked_mul(rows)
        .and_then(|v| v.checked_mul(cols))
        .is_none_or(|work| work > MAX_WORK)
    {
        return Err(Error::WorkLimit);
    }
    let mut u = filled(rows + 1, 0i128)?;
    let mut v = filled(cols + 1, 0i128)?;
    let mut owner = filled(cols + 1, 0usize)?;
    let mut previous = filled(cols + 1, 0usize)?;
    let mut slack = filled(cols + 1, i128::MAX)?;
    let mut visited = filled(cols + 1, false)?;
    let mut columns = filled(rows, 0usize)?;
    for row in 1..=rows {
        owner[0] = row;
        slack.fill(i128::MAX);
        visited.fill(false);
        let mut column = 0;
        loop {
            visited[column] = true;
            let active_row = owner[column];
            let mut delta = i128::MAX;
            let mut next = 0;
            for candidate in 1..=cols {
                if !visited[candidate] {
                    let reduced = i128::from(costs[active_row - 1][candidate - 1])
                        - u[active_row]
                        - v[candidate];
                    if reduced < slack[candidate] {
                        slack[candidate] = reduced;
                        previous[candidate] = column;
                    }
                    if slack[candidate] < delta {
                        delta = slack[candidate];
                        next = candidate;
                    }
                }
            }
            for candidate in 0..=cols {
                if visited[candidate] {
                    u[owner[candidate]] += delta;
                    v[candidate] -= delta;
                } else {
                    slack[candidate] -= delta;
                }
            }
            column = next;
            if owner[column] == 0 {
                break;
            }
        }
        loop {
            let parent = previous[column];
            owner[column] = owner[parent];
            column = parent;
            if column == 0 {
                break;
            }
        }
    }
    for column in 1..=cols {
        if owner[column] != 0 {
            columns[owner[column] - 1] = column - 1;
        }
    }
    let cost = columns
        .iter()
        .enumerate()
        .map(|(row, &col)| i128::from(costs[row][col]))
        .sum();
    u.remove(0);
    v.remove(0);
    Ok(Assignment {
        columns,
        cost,
        row_potentials: u,
        column_potentials: v,
    })
}

fn filled<T: Clone>(length: usize, value: T) -> Result<Vec<T>, Error> {
    let mut out = Vec::new();
    out.try_reserve_exact(length)
        .map_err(|_| Error::AllocationFailed)?;
    out.resize(length, value);
    Ok(out)
}

#[cfg(test)]
mod tests;
