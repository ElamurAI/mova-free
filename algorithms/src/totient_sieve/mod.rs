//! Euler totients by prime-factor elimination.
//! Mathematical source: Hardy and Wright, An Introduction to the Theory of
//! Numbers, chapter V (Euler's function and its product formula).
//! Original sieve: O(n log log n) arithmetic work, O(n) storage.
pub const MAX_N: usize = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TotientError {
    LimitExceeded { requested: usize, maximum: usize },
}

/// Returns phi(0)..=phi(n), with phi(0)=0 and phi(1)=1.
/// Rejects n>MAX_N before allocating; allocation failure follows Vec policy.
pub fn totient_sieve(n: usize) -> Result<Vec<usize>, TotientError> {
    if n > MAX_N {
        return Err(TotientError::LimitExceeded {
            requested: n,
            maximum: MAX_N,
        });
    }
    let mut phi: Vec<usize> = (0..=n).collect();
    for p in 2..=n {
        if phi[p] == p {
            for multiple in (p..=n).step_by(p) {
                phi[multiple] -= phi[multiple] / p;
            }
        }
    }
    Ok(phi)
}

#[cfg(test)]
mod tests;
