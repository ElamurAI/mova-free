//! Deterministic bounded Pollard rho using Floyd cycle detection.
//! J. M. Pollard, "A Monte Carlo method for factorization", BIT 15 (1975).
//! Fixed restart constants give reproducibility, not a success guarantee.
pub const MAX_STEPS: u64 = 10_000_000;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FactorError {
    Zero,
    BudgetLimit,
    BudgetExhausted,
}
impl std::fmt::Display for FactorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Zero => "zero has no finite prime factorization",
            Self::BudgetLimit => "budget exceeds MAX_STEPS",
            Self::BudgetExhausted => "rho iteration budget exhausted",
        })
    }
}
impl std::error::Error for FactorError {}

fn step(x: u64, c: u64, n: u64) -> u64 {
    ((u128::from(x) * u128::from(x) + u128::from(c)) % u128::from(n)) as u64
}

/// Sorted prime factors with multiplicity; one has an empty factorization.
/// Validates zero, then budget cap. Budget counts Floyd iterations globally
/// across all splits/restarts (three modular squares and one gcd per iteration).
/// Primality tests and even splits consume no iterations. Exhaustion returns
/// an error, never a partial successful list. No budget guarantees success.
/// Conservative O((budget+64)*log(n+1)) word operations and O(log(n+1))
/// words, including pending splits and output. Allocation failure can abort.
pub fn factorize(n: u64, budget: u64) -> Result<Vec<u64>, FactorError> {
    if n == 0 {
        return Err(FactorError::Zero);
    }
    if budget > MAX_STEPS {
        return Err(FactorError::BudgetLimit);
    }
    let mut remaining = budget;
    let mut pending = vec![n];
    let mut factors = Vec::new();
    while let Some(value) = pending.pop() {
        if value == 1 {
            continue;
        }
        if crate::miller_rabin::is_prime(value) {
            factors.push(value);
            continue;
        }
        if value % 2 == 0 {
            factors.push(2);
            pending.push(value / 2);
            continue;
        }
        let mut c = 1u64;
        let divisor = 'restart: loop {
            let (mut x, mut y) = (2u64, 2u64);
            loop {
                if remaining == 0 {
                    return Err(FactorError::BudgetExhausted);
                }
                remaining -= 1;
                x = step(x, c, value);
                y = step(step(y, c, value), c, value);
                let d = crate::gcd::gcd(x.abs_diff(y), value);
                if d == value {
                    c += 1;
                    break;
                }
                if d > 1 {
                    break 'restart d;
                }
            }
        };
        pending.push(divisor);
        pending.push(value / divisor);
    }
    factors.sort_unstable();
    Ok(factors)
}
#[cfg(test)]
mod tests;
