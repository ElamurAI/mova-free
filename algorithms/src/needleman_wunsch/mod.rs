//! Bounded global byte alignment. Origin: Needleman and Wunsch,
//! Journal of Molecular Biology 48 (1970), 443-453.
//! Original linear-gap dynamic program; no biological scoring assumptions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scoring {
    pub match_score: i64,
    pub mismatch_score: i64,
    pub gap_score: i64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alignment {
    pub score: i64,
    pub columns: Vec<(Option<u8>, Option<u8>)>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignmentError {
    SizeOverflow,
    CellLimit,
    ScoreOverflow,
    Allocation,
}

/// Maximize signed global score. Ties prefer diagonal, deletion, insertion.
/// The (a.len()+1)*(b.len()+1) matrix must fit max_cells. Size and budget
/// checks precede allocation. Every candidate addition is checked, including
/// losing candidates: overflow rejects the call. O((n+1)(m+1)) time/storage,
/// plus O(n+m) traceback output. All reservations return allocation errors.
pub fn align(
    a: &[u8],
    b: &[u8],
    scoring: Scoring,
    max_cells: usize,
) -> Result<Alignment, AlignmentError> {
    let rows = a.len().checked_add(1).ok_or(AlignmentError::SizeOverflow)?;
    let width = b.len().checked_add(1).ok_or(AlignmentError::SizeOverflow)?;
    let cells = rows
        .checked_mul(width)
        .ok_or(AlignmentError::SizeOverflow)?;
    if cells > max_cells {
        return Err(AlignmentError::CellLimit);
    }
    let mut scores = Vec::new();
    scores
        .try_reserve_exact(cells)
        .map_err(|_| AlignmentError::Allocation)?;
    scores.resize(cells, 0i64);
    let mut steps = Vec::new();
    steps
        .try_reserve_exact(cells)
        .map_err(|_| AlignmentError::Allocation)?;
    steps.resize(cells, 0u8);
    let add = |x: i64, y: i64| x.checked_add(y).ok_or(AlignmentError::ScoreOverflow);
    for i in 1..rows {
        scores[i * width] = add(scores[(i - 1) * width], scoring.gap_score)?;
        steps[i * width] = 1;
    }
    for j in 1..width {
        scores[j] = add(scores[j - 1], scoring.gap_score)?;
        steps[j] = 2;
    }
    for i in 1..rows {
        for j in 1..width {
            let index = i * width + j;
            let diagonal = add(
                scores[index - width - 1],
                if a[i - 1] == b[j - 1] {
                    scoring.match_score
                } else {
                    scoring.mismatch_score
                },
            )?;
            let deletion = add(scores[index - width], scoring.gap_score)?;
            let insertion = add(scores[index - 1], scoring.gap_score)?;
            let mut best = diagonal;
            let mut step = 0;
            if deletion > best {
                best = deletion;
                step = 1;
            }
            if insertion > best {
                best = insertion;
                step = 2;
            }
            scores[index] = best;
            steps[index] = step;
        }
    }
    let capacity = a
        .len()
        .checked_add(b.len())
        .ok_or(AlignmentError::SizeOverflow)?;
    let mut columns = Vec::new();
    columns
        .try_reserve_exact(capacity)
        .map_err(|_| AlignmentError::Allocation)?;
    let (mut i, mut j) = (a.len(), b.len());
    while i > 0 || j > 0 {
        match steps[i * width + j] {
            0 => {
                i -= 1;
                j -= 1;
                columns.push((Some(a[i]), Some(b[j])));
            }
            1 => {
                i -= 1;
                columns.push((Some(a[i]), None));
            }
            _ => {
                j -= 1;
                columns.push((None, Some(b[j])));
            }
        }
    }
    columns.reverse();
    Ok(Alignment {
        score: scores[cells - 1],
        columns,
    })
}
#[cfg(test)]
mod tests;
