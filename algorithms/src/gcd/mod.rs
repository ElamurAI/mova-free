//! Euclidean gcd, composed from repository-owned math/src/nt.rs::gcd_u64
//! and math/src/calc.rs::gcd_small. No external solution source.

/// Greatest common divisor of two unsigned integers, with `gcd(0, 0) = 0`.
/// Uses constant space and logarithmically many remainder operations.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

#[cfg(test)]
mod tests;
