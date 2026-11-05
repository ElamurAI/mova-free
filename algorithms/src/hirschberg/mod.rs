//! Linear-row longest common subsequence reconstruction.
//! Original implementation of D. S. Hirschberg, "A linear space algorithm
//! for computing maximal common subsequences", CACM 18(6), 1975, 341–343.

pub const MAX_LENGTH: usize = 1_000_000;
pub const MAX_CELLS: usize = 16_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    LengthLimit,
    WorkLimit,
    AllocationFailed,
}

/// Increasing `(left_index, right_index)` pairs witnessing an exact LCS.
/// Equality must be stable and behave as an equivalence relation. The shorter
/// input is the row axis (right on equal lengths). Splits choose the smallest
/// maximizing row position; singleton matches choose the first row occurrence.
/// This deterministic witness need not be lexicographically smallest globally.
/// Length limits precede the product work limit, then fallible allocations.
/// O(m*n) equality calls, O(min(m,n)+log(max(m,n))) auxiliary storage plus output;
/// equality's own cost and recursion stack allocation are not controlled here.
pub fn hirschberg<T: Eq>(left: &[T], right: &[T]) -> Result<Vec<(usize, usize)>, Error> {
    if left.len().max(right.len()) > MAX_LENGTH {
        return Err(Error::LengthLimit);
    }
    if left
        .len()
        .checked_mul(right.len())
        .is_none_or(|n| n > MAX_CELLS)
    {
        return Err(Error::WorkLimit);
    }
    if left.is_empty() || right.is_empty() {
        return Ok(Vec::new());
    }
    let swapped = left.len() < right.len();
    let (a, b) = if swapped {
        (right, left)
    } else {
        (left, right)
    };
    let mut forward = row(b.len() + 1)?;
    let mut backward = row(b.len() + 1)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(b.len())
        .map_err(|_| Error::AllocationFailed)?;
    reconstruct(a, b, (0, 0), &mut forward, &mut backward, &mut output);
    if swapped {
        for pair in &mut output {
            *pair = (pair.1, pair.0);
        }
    }
    Ok(output)
}

fn row(len: usize) -> Result<Vec<usize>, Error> {
    let mut row = Vec::new();
    row.try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    row.resize(len, 0);
    Ok(row)
}

fn lengths<T: Eq>(a: &[T], b: &[T], reverse: bool, row: &mut [usize]) {
    row[..=b.len()].fill(0);
    for i in 0..a.len() {
        let mut diagonal = 0;
        for j in 0..b.len() {
            let previous = row[j + 1];
            let equal = if reverse {
                a[a.len() - 1 - i] == b[b.len() - 1 - j]
            } else {
                a[i] == b[j]
            };
            row[j + 1] = if equal {
                diagonal + 1
            } else {
                row[j].max(previous)
            };
            diagonal = previous;
        }
    }
}

fn reconstruct<T: Eq>(
    a: &[T],
    b: &[T],
    offset: (usize, usize),
    forward: &mut [usize],
    backward: &mut [usize],
    output: &mut Vec<(usize, usize)>,
) {
    if a.is_empty() || b.is_empty() {
        return;
    }
    if a.len() == 1 {
        if let Some(j) = b.iter().position(|x| *x == a[0]) {
            output.push((offset.0, offset.1 + j));
        }
        return;
    }
    let middle = a.len() / 2;
    lengths(&a[..middle], b, false, forward);
    lengths(&a[middle..], b, true, backward);
    let mut split = 0;
    let mut best = 0;
    for j in 0..=b.len() {
        let score = forward[j] + backward[b.len() - j];
        if score > best {
            best = score;
            split = j;
        }
    }
    reconstruct(&a[..middle], &b[..split], offset, forward, backward, output);
    reconstruct(
        &a[middle..],
        &b[split..],
        (offset.0 + middle, offset.1 + split),
        forward,
        backward,
        output,
    );
}

#[cfg(test)]
mod tests;
