//! Exact trial-division prime factorization, with ascending prime powers.

/// Zero has no finite prime factorization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ZeroInput,
}

/// Returns ascending (prime, positive exponent) pairs; one returns an empty list.
/// O(sqrt(n)) trial divisions in the worst case; O(log n) output storage.
/// Large primes may be slow. Allocation failure is not represented by Error.
pub fn factorize(mut n: u64) -> Result<Vec<(u64, u32)>, Error> {
    if n == 0 {
        return Err(Error::ZeroInput);
    }
    let mut factors = Vec::new();
    let mut divisor = 2;
    while divisor <= n / divisor {
        let mut exponent = 0;
        while n.is_multiple_of(divisor) {
            n /= divisor;
            exponent += 1;
        }
        if exponent != 0 {
            factors.push((divisor, exponent));
        }
        divisor = if divisor == 2 { 3 } else { divisor + 2 };
    }
    if n > 1 {
        factors.push((n, 1));
    }
    Ok(factors)
}

#[cfg(test)]
mod tests;
