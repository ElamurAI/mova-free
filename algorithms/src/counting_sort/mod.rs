//! Exact counting sort over an explicitly bounded integer span.

/// Failure before the input is modified.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CountingSortError {
    /// The inclusive min-to-max span cannot fit in usize.
    RangeOverflow,
    /// The histogram would exceed the caller's bucket budget.
    BucketLimitExceeded { required: usize, limit: usize },
    /// The histogram capacity or allocation cannot be satisfied.
    AllocationFailed,
}

impl std::fmt::Display for CountingSortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RangeOverflow => f.write_str("counting sort range overflow"),
            Self::BucketLimitExceeded { required, limit } => write!(
                f,
                "counting sort requires {required} buckets; limit is {limit}"
            ),
            Self::AllocationFailed => f.write_str("counting sort allocation failed"),
        }
    }
}
impl std::error::Error for CountingSortError {}

/// Sort ascending, preserving every multiplicity.
///
/// A nonempty slice requires max - min + 1 buckets, including a singleton.
/// The bucket budget limits histogram elements, not input elements. All errors
/// leave the input unchanged. Time is O(n + span), auxiliary space O(span).
pub fn counting_sort(values: &mut [usize], max_buckets: usize) -> Result<(), CountingSortError> {
    let Some(&first) = values.first() else {
        return Ok(());
    };
    let (mut minimum, mut maximum) = (first, first);
    for &value in values.iter() {
        minimum = minimum.min(value);
        maximum = maximum.max(value);
    }
    let span = (maximum - minimum)
        .checked_add(1)
        .ok_or(CountingSortError::RangeOverflow)?;
    if span > max_buckets {
        return Err(CountingSortError::BucketLimitExceeded {
            required: span,
            limit: max_buckets,
        });
    }
    let mut counts = Vec::<usize>::new();
    counts
        .try_reserve_exact(span)
        .map_err(|_| CountingSortError::AllocationFailed)?;
    counts.resize(span, 0);
    for &value in values.iter() {
        counts[value - minimum] += 1;
    }
    let mut written = 0;
    for (offset, count) in counts.into_iter().enumerate() {
        values[written..written + count].fill(minimum + offset);
        written += count;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
