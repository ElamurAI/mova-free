//! Canonical byte run-length encoding, a classical lossless coding method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Run {
    pub byte: u8,
    pub count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    ZeroCount,
    AdjacentEqual,
    LengthOverflow,
    OutputLimit,
    Allocation,
}

/// Unique maximal runs. O(n) time, O(r) output; allocation may panic/abort.
pub fn encode(input: &[u8]) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    for &byte in input {
        if let Some(last) = runs.last_mut().filter(|run| run.byte == byte) {
            last.count += 1;
        } else {
            runs.push(Run { byte, count: 1 });
        }
    }
    runs
}

/// Decode canonical runs within an explicit output-byte budget.
/// In input order, reject zero, then adjacent equal, then sum overflow;
/// only after all runs are valid check the budget and reserve output.
/// O(r+n) time and O(n) output. Reservation errors are returned.
pub fn decode(runs: &[Run], max_output: usize) -> Result<Vec<u8>, DecodeError> {
    let mut total = 0usize;
    let mut previous = None;
    for run in runs {
        if run.count == 0 {
            return Err(DecodeError::ZeroCount);
        }
        if previous == Some(run.byte) {
            return Err(DecodeError::AdjacentEqual);
        }
        total = total
            .checked_add(run.count)
            .ok_or(DecodeError::LengthOverflow)?;
        previous = Some(run.byte);
    }
    if total > max_output {
        return Err(DecodeError::OutputLimit);
    }
    let mut result = Vec::new();
    result
        .try_reserve_exact(total)
        .map_err(|_| DecodeError::Allocation)?;
    for run in runs {
        result.resize(result.len() + run.count, run.byte);
    }
    Ok(result)
}
#[cfg(test)]
mod tests;
