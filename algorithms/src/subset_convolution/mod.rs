//! Ranked subset convolution; Bjorklund et al., Fourier meets Mobius, STOC 2007.
pub const MODULUS: u64 = 998244353;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Shape,
    TooLarge,
}
/// Returns sum a[T]b[S\\T] over T subset S, modulo MODULUS.
/// Equal nonempty power-two lengths, at most4096. Allocator failure is not caught.
pub fn convolve(a: &[u64], b: &[u64]) -> Result<Vec<u64>, Error> {
    let n = a.len();
    if n > 4096 || b.len() > 4096 {
        return Err(Error::TooLarge);
    }
    if n != b.len() || !n.is_power_of_two() {
        return Err(Error::Shape);
    }
    let d = n.trailing_zeros() as usize;
    let mut f = vec![vec![0; n]; d + 1];
    let mut g = f.clone();
    for s in 0..n {
        let k = s.count_ones() as usize;
        f[k][s] = a[s] % MODULUS;
        g[k][s] = b[s] % MODULUS;
    }
    for bit in 0..d {
        for s in 0..n {
            if s & (1 << bit) != 0 {
                for k in 0..=d {
                    f[k][s] = (f[k][s] + f[k][s ^ (1 << bit)]) % MODULUS;
                    g[k][s] = (g[k][s] + g[k][s ^ (1 << bit)]) % MODULUS;
                }
            }
        }
    }
    let mut h = vec![vec![0; n]; d + 1];
    for k in 0..=d {
        for i in 0..=k {
            for s in 0..n {
                h[k][s] = (h[k][s] + f[i][s] * g[k - i][s] % MODULUS) % MODULUS;
            }
        }
    }
    for bit in 0..d {
        for s in 0..n {
            if s & (1 << bit) != 0 {
                for row in &mut h {
                    row[s] = (row[s] + MODULUS - row[s ^ (1 << bit)]) % MODULUS;
                }
            }
        }
    }
    Ok((0..n).map(|s| h[s.count_ones() as usize][s]).collect())
}
#[cfg(test)]
mod tests;
