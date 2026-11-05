//! Minimal finite-prefix recurrence over a fixed prime field.
//! Original implementation of the algorithm described by J. L. Massey,
//! "Shift-register synthesis and BCH decoding", IEEE Transactions on
//! Information Theory 15(1), 1969, pp. 122–127 (following Berlekamp).

pub const MODULUS: u64 = 998_244_353;
/// Quadratic-work policy cap, not a speed guarantee.
pub const MAX_TERMS: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooManyTerms,
    AllocationFailed,
}

fn zeros(n: usize) -> Result<Vec<u64>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(n, 0);
    Ok(v)
}

fn power(mut x: u64, mut e: u64) -> u64 {
    let mut y = 1;
    while e > 0 {
        if e & 1 != 0 {
            y = y * x % MODULUS;
        }
        x = x * x % MODULUS;
        e >>= 1;
    }
    y
}

/// Return a minimum-length c satisfying s[t] = sum_j c[j]*s[t-1-j]
/// modulo MODULUS for every c.len() <= t < s.len(). Coefficients are in
/// the lag order accepted by linear_recurrence::nth_term. Empty/all-zero
/// prefixes return an empty vector (nth_term itself requires nonempty order).
/// Inputs are reduced modulo the prime. A finite prefix need not uniquely
/// identify coefficients or the true continuation; this deterministic answer
/// only asserts compatibility and minimum order on the supplied prefix.
/// O(n² + n log MODULUS) word time, O(n) space. Length is checked before
/// allocation; allocation failures are reported. Products fit u64 because
/// (MODULUS-1)² < 2^60, and every sum/difference is below 2*MODULUS.
pub fn minimal_recurrence(s: &[u64]) -> Result<Vec<u64>, Error> {
    if s.len() > MAX_TERMS {
        return Err(Error::TooManyTerms);
    }
    let mut c = zeros(s.len() + 1)?;
    let mut b = zeros(s.len() + 1)?;
    let mut saved = zeros(s.len() + 1)?;
    c[0] = 1;
    b[0] = 1;
    let (mut order, mut shift, mut previous) = (0, 1, 1);
    for n in 0..s.len() {
        let mut discrepancy = s[n] % MODULUS;
        for j in 1..=order {
            discrepancy = (discrepancy + c[j] * (s[n - j] % MODULUS) % MODULUS) % MODULUS;
        }
        if discrepancy == 0 {
            shift += 1;
            continue;
        }
        saved.copy_from_slice(&c);
        // previous is a stored nonzero discrepancy; Fermat inversion is valid.
        let ratio = discrepancy * power(previous, MODULUS - 2) % MODULUS;
        for j in 0..c.len() - shift {
            c[j + shift] = (c[j + shift] + MODULUS - ratio * b[j] % MODULUS) % MODULUS;
        }
        if 2 * order <= n {
            order = n + 1 - order;
            b.copy_from_slice(&saved);
            previous = discrepancy;
            shift = 1;
        } else {
            shift += 1;
        }
    }
    for j in 0..order {
        c[j] = (MODULUS - c[j + 1]) % MODULUS;
    }
    c.truncate(order);
    Ok(c)
}

#[cfg(test)]
mod tests;
