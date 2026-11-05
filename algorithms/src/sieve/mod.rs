//! Inclusive Eratosthenes sieve with checked sizing and fallible allocation.

/// Failure to represent or allocate the sieve's storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SieveError {
    SizeOverflow,
    AllocationFailed,
}

impl std::fmt::Display for SieveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::SizeOverflow => "sieve storage size is not representable",
            Self::AllocationFailed => "sieve storage allocation failed",
        })
    }
}
impl std::error::Error for SieveError {}

fn reserve<T>(values: &mut Vec<T>, count: usize) -> Result<(), SieveError> {
    let bytes = count
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(SieveError::SizeOverflow)?;
    if bytes > isize::MAX as usize {
        return Err(SieveError::SizeOverflow);
    }
    values
        .try_reserve_exact(count)
        .map_err(|_| SieveError::AllocationFailed)
}

/// Returns all primes <= limit, in increasing order.
/// O(limit log log limit) time and O(limit) space for limit >= 3.
/// Reported allocator failures are returned; process aborts are outside Result.
pub fn sieve(limit: usize) -> Result<Vec<usize>, SieveError> {
    if limit < 2 {
        return Ok(Vec::new());
    }
    let count = limit.checked_add(1).ok_or(SieveError::SizeOverflow)?;
    let mut flags = Vec::<u8>::new();
    reserve(&mut flags, count)?;
    flags.resize(count, 1);
    flags[0] = 0;
    flags[1] = 0;
    let mut p = 2;
    while p <= limit / p {
        if flags[p] != 0 {
            let mut multiple = p * p;
            loop {
                flags[multiple] = 0;
                if limit - multiple < p {
                    break;
                }
                multiple += p;
            }
        }
        p += 1;
    }
    let prime_count = flags.iter().filter(|&&flag| flag != 0).count();
    let mut primes = Vec::new();
    reserve(&mut primes, prime_count)?;
    for (value, flag) in flags.into_iter().enumerate() {
        if flag != 0 {
            primes.push(value);
        }
    }
    Ok(primes)
}

#[cfg(test)]
mod tests;
