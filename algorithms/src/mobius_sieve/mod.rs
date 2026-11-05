//! Linear prime sieve for the Mobius function.
//! Mathematical source: Hardy and Wright, An Introduction to the Theory of
//! Numbers, chapter XVI (the Mobius function).
//! Each composite is assigned by its least prime once: O(n) time/space.
pub const MAX_N: usize = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobiusError {
    LimitExceeded { requested: usize, maximum: usize },
}

/// Returns mu(0)..=mu(n), with mu(0)=0 and mu(1)=1.
/// Rejects n>MAX_N before allocating; allocation failure follows Vec policy.
pub fn mobius_sieve(n: usize) -> Result<Vec<i8>, MobiusError> {
    if n > MAX_N {
        return Err(MobiusError::LimitExceeded {
            requested: n,
            maximum: MAX_N,
        });
    }
    let mut mu = vec![0; n + 1];
    let mut composite = vec![false; n + 1];
    let mut primes = Vec::new();
    if n >= 1 {
        mu[1] = 1;
    }
    for i in 2..=n {
        if !composite[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            if p > n / i {
                break;
            }
            let product = i * p;
            composite[product] = true;
            if i % p == 0 {
                mu[product] = 0;
                break;
            }
            mu[product] = -mu[i];
        }
    }
    Ok(mu)
}

#[cfg(test)]
mod tests;
