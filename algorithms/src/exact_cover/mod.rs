//! Original bounded bit-mask Algorithm X, not DLX.
//! Reference: Knuth, Dancing Links (2000), Algorithm X.
pub const MAX_COLUMNS: usize = 64;
pub const MAX_ROWS: usize = 4096;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidColumn { row: usize, column: usize },
    DuplicateColumn { row: usize, column: usize },
    ResourceExhausted,
}
/// Row indices in search order; None only after complete infeasibility search.
/// Every visited state including success consumes budget. Empty rows ignored.
/// Limits then row-order endpoint/duplicate validation precede search.
/// O(budget*columns*rows) time, O(rows*columns) space; allocation may abort.
pub fn solve(
    columns: usize,
    rows: &[Vec<usize>],
    node_budget: u64,
) -> Result<Option<Vec<usize>>, Error> {
    if columns > MAX_COLUMNS || rows.len() > MAX_ROWS {
        return Err(Error::LimitExceeded);
    }
    let mut masks = Vec::new();
    for (row, values) in rows.iter().enumerate() {
        let mut mask = 0u64;
        for &column in values {
            if column >= columns {
                return Err(Error::InvalidColumn { row, column });
            }
            let bit = 1u64 << column;
            if mask & bit != 0 {
                return Err(Error::DuplicateColumn { row, column });
            }
            mask |= bit;
        }
        masks.push(mask);
    }
    let full = if columns == 64 {
        u64::MAX
    } else {
        (1u64 << columns) - 1
    };
    let mut budget = node_budget;
    let mut chosen = Vec::new();
    if search(full, &masks, &mut budget, &mut chosen)? {
        Ok(Some(chosen))
    } else {
        Ok(None)
    }
}
fn search(
    remaining: u64,
    masks: &[u64],
    budget: &mut u64,
    chosen: &mut Vec<usize>,
) -> Result<bool, Error> {
    if *budget == 0 {
        return Err(Error::ResourceExhausted);
    }
    *budget -= 1;
    if remaining == 0 {
        return Ok(true);
    }
    let mut best = Vec::new();
    let mut count = usize::MAX;
    for c in 0..64 {
        let bit = 1u64 << c;
        if remaining & bit == 0 {
            continue;
        }
        let candidates: Vec<_> = masks
            .iter()
            .enumerate()
            .filter_map(|(i, &m)| ((m & bit != 0) && (m & remaining == m)).then_some(i))
            .collect();
        if candidates.len() < count {
            count = candidates.len();
            best = candidates;
        }
        if count == 0 {
            return Ok(false);
        }
    }
    for i in best {
        chosen.push(i);
        if search(remaining ^ masks[i], masks, budget, chosen)? {
            return Ok(true);
        }
        chosen.pop();
    }
    Ok(false)
}
#[cfg(test)]
mod tests;
