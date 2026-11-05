//! Shanks (1971), baby-step giant-step in a fixed prime field.
use std::collections::BTreeMap;
pub const MODULUS: u64 = 998244353;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ResourceLimit,
}
fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 != 0 {
            r = r * a % MODULUS
        }
        a = a * a % MODULUS;
        e >>= 1;
    }
    r
}
/// Least nonnegative exponent, with 0^0=1. None means no solution.
/// Fixed prime needs31596 table entries, below fixed resource ceiling.
/// Standard allocator exhaustion is not caught.
pub fn discrete_log(base: u64, target: u64) -> Result<Option<u64>, Error> {
    let g = base % MODULUS;
    let t = target % MODULUS;
    if t == 1 {
        return Ok(Some(0));
    }
    if g == 0 {
        return Ok(if t == 0 { Some(1) } else { None });
    }
    if t == 0 {
        return Ok(None);
    }
    let mut n = 1u64;
    while n * n < MODULUS - 1 {
        n += 1;
    }
    if n > 40000 {
        return Err(Error::ResourceLimit);
    }
    let mut table = BTreeMap::new();
    let mut v = 1;
    for j in 0..n {
        table.entry(v).or_insert(j);
        v = v * g % MODULUS;
    }
    let factor = pow(pow(g, MODULUS - 2), n);
    let mut gamma = t;
    for i in 0..=n {
        if let Some(&j) = table.get(&gamma) {
            let x = i * n + j;
            if x < MODULUS - 1 {
                return Ok(Some(x));
            }
        }
        gamma = gamma * factor % MODULUS;
    }
    Ok(None)
}
#[cfg(test)]
mod tests;
