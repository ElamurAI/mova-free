//! Segmented form of the sieve of Eratosthenes (classical prime sieving).
/// Hard cap on the inclusive upper endpoint; base sieve uses at most 1,000,001 entries.
pub const MAX_HIGH: u64 = 1_000_000_000_000;
/// Hard cap on inclusive interval width.
pub const MAX_WIDTH: u64 = 1_000_000;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SieveError {
    ReversedInterval,
    HighLimit,
    WidthLimit,
}
impl std::fmt::Display for SieveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ReversedInterval => "low exceeds high",
            Self::HighLimit => "high exceeds MAX_HIGH",
            Self::WidthLimit => "interval exceeds MAX_WIDTH",
        })
    }
}
impl std::error::Error for SieveError {}

/// Enumerates primes in the inclusive interval, in ascending order.
/// Validates order, endpoint cap, then width cap before allocating.
/// Includes a fresh base sieve: time is
/// O(sqrt(high) loglog(high) + width loglog(high) + sqrt(high) + width).
/// Storage is O(sqrt(high) + width + output). Allocator failure can abort.
pub fn segmented_sieve(low: u64, high: u64) -> Result<Vec<u64>, SieveError> {
    if low > high {
        return Err(SieveError::ReversedInterval);
    }
    if high > MAX_HIGH {
        return Err(SieveError::HighLimit);
    }
    let width = high - low + 1;
    if width > MAX_WIDTH {
        return Err(SieveError::WidthLimit);
    }
    // Binary search avoids floating-point rounding near perfect squares.
    let (mut lo, mut hi) = (0u64, 1_000_001u64);
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if mid * mid <= high {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let limit = lo as usize;
    let mut base = vec![true; limit + 1];
    base[0] = false;
    if limit >= 1 {
        base[1] = false;
    }
    let mut segment = vec![true; width as usize];
    for p in 2..=limit {
        if !base[p] {
            continue;
        }
        if p <= limit / p {
            for multiple in (p * p..=limit).step_by(p) {
                base[multiple] = false;
            }
        }
        let p = p as u64;
        let start = (p * p).max(low.div_ceil(p) * p);
        if start <= high {
            for multiple in (start..=high).step_by(p as usize) {
                segment[(multiple - low) as usize] = false;
            }
        }
    }
    Ok(segment
        .into_iter()
        .enumerate()
        .filter_map(|(i, survives)| {
            let value = low + i as u64;
            (survives && value >= 2).then_some(value)
        })
        .collect())
}
#[cfg(test)]
mod tests;
