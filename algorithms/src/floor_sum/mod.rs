//! Euclidean lattice-point floor sum, using Euclid's remainder algorithm
//! (Elements, Book VII). The floor-sum reciprocity follows by counting
//! integer points under a line in the two coordinate orders.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloorSumError {
    ZeroModulus,
    Overflow,
}
impl std::fmt::Display for FloorSumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ZeroModulus => "modulus must be positive",
            Self::Overflow => "sum exceeds u128",
        })
    }
}
impl std::error::Error for FloorSumError {}

/// Exact sum of floor((a*i+b)/m) for 0 <= i < n. Zero modulus is checked
/// first, including for empty sums. Every u64 parameter is supported;
/// results exceeding u128 are rejected. O(log(max(a,m)+1)) time, O(1) space.
pub fn floor_sum(n: u64, m: u64, a: u64, b: u64) -> Result<u128, FloorSumError> {
    if m == 0 {
        return Err(FloorSumError::ZeroModulus);
    }
    let (mut n, mut m, mut a, mut b) = (u128::from(n), u128::from(m), u128::from(a), u128::from(b));
    let mut answer = 0u128;
    loop {
        if n == 0 {
            return Ok(answer);
        }
        let triangle = if n % 2 == 0 {
            (n / 2) * (n - 1)
        } else {
            n * ((n - 1) / 2)
        };
        let contribution = triangle.checked_mul(a / m).ok_or(FloorSumError::Overflow)?;
        answer = answer
            .checked_add(contribution)
            .ok_or(FloorSumError::Overflow)?;
        answer = answer
            .checked_add(n.checked_mul(b / m).ok_or(FloorSumError::Overflow)?)
            .ok_or(FloorSumError::Overflow)?;
        a %= m;
        b %= m;
        // n never grows; a,m,b stay within u64. This product fits u128.
        let height = a * n + b;
        if height < m {
            return Ok(answer);
        }
        n = height / m;
        b = height % m;
        std::mem::swap(&mut a, &mut m);
    }
}
#[cfg(test)]
mod tests;
