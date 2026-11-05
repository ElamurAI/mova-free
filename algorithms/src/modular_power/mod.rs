//! Modular exponentiation, specializing the repository-owned square-and-multiply
//! idea in math/src/nt.rs::modpow_plain to unsigned fixed-width arithmetic.

/// A zero modulus has no supported residue class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZeroModulus;

impl std::fmt::Display for ZeroModulus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("modulus must be nonzero")
    }
}
impl std::error::Error for ZeroModulus {}

/// Returns base^exponent modulo modulus, with 0^0 treated as the identity.
/// Rejects modulus zero; modulus one always returns zero.
/// Uses at most 64 exponent-bit iterations, constant space and exact u128 products.
pub fn modular_power(base: u64, mut exponent: u64, modulus: u64) -> Result<u64, ZeroModulus> {
    if modulus == 0 {
        return Err(ZeroModulus);
    }
    let m = u128::from(modulus);
    let mut power = u128::from(base) % m;
    let mut result = 1 % m;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = result * power % m;
        }
        power = power * power % m;
        exponent >>= 1;
    }
    Ok(result as u64)
}

#[cfg(test)]
mod tests;
