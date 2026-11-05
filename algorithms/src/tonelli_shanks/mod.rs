//! Modular square roots by the Tonelli–Shanks method.
//! D. Shanks, "Five Number-Theoretic Algorithms", Proceedings of the Second
//! Manitoba Conference on Numerical Mathematics (1972), 51–70.
//! Primality validation delegates to the full-u64 deterministic Miller–Rabin API.

/// Modulus validation precedes normalization or shortcuts for the radicand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModulusError {
    /// Zero and one do not define a prime field.
    InvalidModulus,
    /// A modulus at least two was proved composite by the primality API.
    CompositeModulus,
}
impl std::fmt::Display for ModulusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidModulus => "modulus must be at least two",
            Self::CompositeModulus => "modulus must be prime",
        })
    }
}
impl std::error::Error for ModulusError {}

fn mul(a: u64, b: u64, p: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
}
fn power(a: u64, e: u64, p: u64) -> u64 {
    crate::modular_power::modular_power(a, e, p).expect("validated prime modulus")
}

/// Returns the smaller square root of a modulo a prime modulus, or None for
/// a nonresidue. Accepts every u64 radicand, reducing it after validation.
/// Ascending nonresidue search is deterministic and finite (at most p-2
/// candidates); no small worst-case search bound is claimed. Worst-case
/// O(p log p + log² p) modular operations and O(1) auxiliary storage.
/// All modular products use u128; no randomized choices or allocations occur.
pub fn sqrt_mod(a: u64, modulus: u64) -> Result<Option<u64>, ModulusError> {
    let p = modulus;
    if p < 2 {
        return Err(ModulusError::InvalidModulus);
    }
    if !crate::miller_rabin::is_prime(p) {
        return Err(ModulusError::CompositeModulus);
    }
    let a = a % p;
    if p == 2 || a == 0 {
        return Ok(Some(a));
    }
    if power(a, (p - 1) / 2, p) != 1 {
        return Ok(None);
    }
    let s = (p - 1).trailing_zeros();
    let q = (p - 1) >> s;
    let mut z = 2;
    while power(z, (p - 1) / 2, p) != p - 1 {
        // A nonresidue exists among 2..p, so z cannot overflow.
        z += 1;
    }
    let mut c = power(z, q, p);
    let mut x = power(a, q.div_ceil(2), p);
    let mut t = power(a, q, p);
    let mut m = s;
    while t != 1 {
        let mut i = 0;
        let mut squared = t;
        while squared != 1 {
            squared = mul(squared, squared, p);
            i += 1;
        }
        // Residue and subgroup invariants ensure 0 < i < m <= 63.
        let b = power(c, 1u64 << (m - i - 1), p);
        x = mul(x, b, p);
        c = mul(b, b, p);
        t = mul(t, c, p);
        m = i;
    }
    Ok(Some(x.min(p - x)))
}

#[cfg(test)]
mod tests;
