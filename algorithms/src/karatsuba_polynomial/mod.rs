//! Karatsuba and Ofman (1962), original modular polynomial adaptation.
pub const MODULUS: u64 = 998244353;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooLarge,
}
fn product(a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len();
    let mut out = vec![0; 2 * n];
    if n <= 16 {
        for i in 0..n {
            for j in 0..n {
                out[i + j] = (out[i + j] + a[i] * b[j] % MODULUS) % MODULUS;
            }
        }
        return out;
    }
    let m = n / 2;
    let lo = product(&a[..m], &b[..m]);
    let hi = product(&a[m..], &b[m..]);
    let sa: Vec<_> = (0..m).map(|i| (a[i] + a[m + i]) % MODULUS).collect();
    let sb: Vec<_> = (0..m).map(|i| (b[i] + b[m + i]) % MODULUS).collect();
    let cross = product(&sa, &sb);
    for i in 0..n {
        out[i] = (out[i] + lo[i]) % MODULUS;
        out[i + n] = (out[i + n] + hi[i]) % MODULUS;
        out[i + m] = (out[i + m] + cross[i] + 2 * MODULUS - lo[i] - hi[i]) % MODULUS;
    }
    out
}
/// Ascending coefficients; preserves exact output length and trailing zeros.
/// Either input over4096 returns TooLarge, even if the other is empty.
pub fn multiply(a: &[u64], b: &[u64]) -> Result<Vec<u64>, Error> {
    if a.len() > 4096 || b.len() > 4096 {
        return Err(Error::TooLarge);
    }
    if a.is_empty() || b.is_empty() {
        return Ok(vec![]);
    }
    let n = a.len().max(b.len()).next_power_of_two();
    let mut aa = vec![0; n];
    let mut bb = vec![0; n];
    for (i, &x) in a.iter().enumerate() {
        aa[i] = x % MODULUS
    }
    for (i, &x) in b.iter().enumerate() {
        bb[i] = x % MODULUS
    }
    let mut out = product(&aa, &bb);
    out.truncate(a.len() + b.len() - 1);
    Ok(out)
}
#[cfg(test)]
mod tests;
