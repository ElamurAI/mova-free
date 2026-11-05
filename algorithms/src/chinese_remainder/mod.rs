//! Generalized Chinese remainder theorem for possibly noncoprime moduli.

use crate::extended_gcd::extended_gcd;

/// A residue class; input residues need not be normalized.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Congruence {
    pub residue: u64,
    pub modulus: u64,
}

/// Invalid input or an unrepresentable combined modulus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ZeroModulus { index: usize },
    ModulusOverflow,
}

/// Returns the common class, or None when constraints are incompatible.
/// Empty input gives 0 modulo 1. All zero moduli are checked before arithmetic.
/// Then constraints merge left to right: incompatibility precedes overflow at
/// each merge; an earlier overflow can precede a later incompatibility.
/// O(k log U) word operations for k constraints, U=u64::MAX; O(1) extra space.
pub fn solve(constraints: &[Congruence]) -> Result<Option<Congruence>, Error> {
    if let Some(index) = constraints.iter().position(|c| c.modulus == 0) {
        return Err(Error::ZeroModulus { index });
    }
    let mut current = Congruence {
        residue: 0,
        modulus: 1,
    };
    for &next in constraints {
        let b = next.residue % next.modulus;
        let bezout = extended_gcd(current.modulus, next.modulus);
        let difference = i128::from(b) - i128::from(current.residue);
        let gcd = i128::from(bezout.gcd);
        if difference % gcd != 0 {
            return Ok(None);
        }
        let reduced = next.modulus / bezout.gcd;
        let modulus = u128::from(current.modulus) * u128::from(reduced);
        let modulus = u64::try_from(modulus).map_err(|_| Error::ModulusOverflow)?;
        // Normalize both signed factors before multiplying: their product
        // can exceed i128, but two u64 values always multiply safely in u128.
        let reduced_signed = i128::from(reduced);
        let delta = (difference / gcd).rem_euclid(reduced_signed) as u128;
        let inverse = bezout.x.rem_euclid(reduced_signed) as u128;
        let step = delta * inverse % u128::from(reduced);
        let residue = (u128::from(current.residue) + u128::from(current.modulus) * step)
            % u128::from(modulus);
        current = Congruence {
            residue: residue as u64,
            modulus,
        };
    }
    Ok(Some(current))
}

#[cfg(test)]
mod tests;
