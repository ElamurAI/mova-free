//! Modular Fibonacci numbers by fast doubling.
//! Identities: F(2k)=F(k)(2F(k+1)-F(k)),
//! F(2k+1)=F(k)^2+F(k+1)^2.
//! Reference: Knuth, The Art of Computer Programming, volume 1, section 1.2.8.
//! O(log(n+1)) fixed-width arithmetic and O(1) space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibonacciError {
    ZeroModulus,
}

/// Returns F(n) modulo modulus for every u64 index and nonzero modulus.
pub fn fibonacci_mod(n: u64, modulus: u64) -> Result<u64, FibonacciError> {
    if modulus == 0 {
        return Err(FibonacciError::ZeroModulus);
    }
    let m = u128::from(modulus);
    let (mut a, mut b) = (0u128, 1 % m);
    for bit in (0..(u64::BITS - n.leading_zeros())).rev() {
        let twice_b_minus_a = (2 * b + m - a) % m;
        let c = a * twice_b_minus_a % m;
        // Reduce each square before adding: their unreduced sum may exceed u128.
        let d = ((a * a % m) + (b * b % m)) % m;
        if (n >> bit) & 1 == 0 {
            (a, b) = (c, d);
        } else {
            (a, b) = (d, (c + d) % m);
        }
    }
    Ok(a as u64)
}

#[cfg(test)]
mod tests;
