//! Exact field elimination; see Golub and Van Loan, Matrix Computations, 4th ed.
pub const MODULUS: u64 = 998244353;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooLarge,
    Shape,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Solution {
    pub particular: Vec<u64>,
    pub nullspace: Vec<Vec<u64>>,
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
/// Affine solution basis, free columns in ascending order; None is inconsistency.
/// Rows and variables capped128 before shape checks. Allocator failure is not caught.
pub fn solve(a: &[Vec<u64>], b: &[u64], variables: usize) -> Result<Option<Solution>, Error> {
    if a.len() > 128 || variables > 128 {
        return Err(Error::TooLarge);
    }
    if a.len() != b.len() || a.iter().any(|r| r.len() != variables) {
        return Err(Error::Shape);
    }
    let mut m: Vec<Vec<u64>> = a
        .iter()
        .zip(b)
        .map(|(row, &rhs)| {
            row.iter()
                .copied()
                .chain(std::iter::once(rhs))
                .map(|v| v % MODULUS)
                .collect()
        })
        .collect();
    let mut pivots = vec![];
    let mut rank = 0;
    for col in 0..variables {
        let Some(p) = (rank..m.len()).find(|&r| m[r][col] != 0) else {
            continue;
        };
        m.swap(rank, p);
        let inv = pow(m[rank][col], MODULUS - 2);
        for x in &mut m[rank] {
            *x = *x * inv % MODULUS
        }
        for r in 0..m.len() {
            if r != rank {
                let factor = m[r][col];
                // Indexed access couples two distinct rows without copying the pivot.
                #[allow(clippy::needless_range_loop)]
                for c in col..=variables {
                    m[r][c] = (m[r][c] + MODULUS - factor * m[rank][c] % MODULUS) % MODULUS;
                }
            }
        }
        pivots.push(col);
        rank += 1;
    }
    if m.iter()
        .any(|r| r[..variables].iter().all(|&x| x == 0) && r[variables] != 0)
    {
        return Ok(None);
    }
    let mut particular = vec![0; variables];
    for (r, &c) in pivots.iter().enumerate() {
        particular[c] = m[r][variables]
    }
    let mut nullspace = vec![];
    for free in 0..variables {
        if !pivots.contains(&free) {
            let mut v = vec![0; variables];
            v[free] = 1;
            for (r, &c) in pivots.iter().enumerate() {
                v[c] = (MODULUS - m[r][free]) % MODULUS;
            }
            nullspace.push(v);
        }
    }
    Ok(Some(Solution {
        particular,
        nullspace,
    }))
}
#[cfg(test)]
mod tests;
