//! Checked exact binomial coefficients, using integral multiplicative steps.
//! Original code; identity reference: NIST DLMF 26.3 (binomial coefficients).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidK,
    Overflow,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Exact n choose k, rejecting k > n and only genuine u128 result overflow.
/// Symmetry reduces the loop to min(k,n-k). Each denominator is cancelled
/// before multiplication, so representable coefficients never falsely overflow.
/// O(min(k,n-k) * log(n+1)) word operations, O(1) space. For u64 n,
/// overflowing inputs terminate early rather than traversing enormous k.
pub fn binomial_coefficient(n: u64, k: u64) -> Result<u128, Error> {
    if k > n {
        return Err(Error::InvalidK);
    }
    let k = k.min(n - k);
    let mut result = 1u128;
    for i in 1..=k {
        let numerator = u128::from(n - k + i);
        let denominator = u128::from(i);
        let common = gcd(numerator, denominator);
        let numerator = numerator / common;
        let denominator = denominator / common;
        // Coprimality and integrality imply denominator divides result.
        debug_assert_eq!(result % denominator, 0);
        result = (result / denominator)
            .checked_mul(numerator)
            .ok_or(Error::Overflow)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests;
