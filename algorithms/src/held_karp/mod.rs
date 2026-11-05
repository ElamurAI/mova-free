//! Exact directed traveling-salesperson cycle using subset dynamic programming.
//! Original implementation; classical attribution: M. Held and R. M. Karp,
//! "A Dynamic Programming Approach to Sequencing Problems", SIAM Journal (1962).

pub const MAX_VERTICES: usize = 18;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tour {
    /// Starts/ends at zero and visits every other vertex once.
    pub vertices: Vec<usize>,
    pub cost: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooManyVertices,
    NonSquareMatrix,
    AllocationFailed,
}

/// Find a minimum Hamiltonian cycle, or None if no such cycle exists.
/// None arcs are absent; self-arcs are ignored. Empty input returns an empty
/// zero-cost tour; singleton returns [0,0] at zero cost without requiring a loop.
/// Vertex cap is checked before shape. The cap bounds the exponential allocation
/// to (n-1)*2^(n-1) cells: time O(n^2*2^n), space O(n*2^n).
/// Increasing subset/end/predecessor scans retain the first equal-cost candidate.
/// The resulting witness is deterministic, not necessarily lexicographically first.
/// At most 18 u64 costs are added, so u128 arithmetic cannot overflow.
pub fn held_karp(arcs: &[Vec<Option<u64>>]) -> Result<Option<Tour>, Error> {
    let n = arcs.len();
    if n > MAX_VERTICES {
        return Err(Error::TooManyVertices);
    }
    if arcs.iter().any(|row| row.len() != n) {
        return Err(Error::NonSquareMatrix);
    }
    if n <= 1 {
        let mut vertices = filled(2 * n, 0usize)?;
        if n == 0 {
            vertices.clear();
        }
        return Ok(Some(Tour { vertices, cost: 0 }));
    }
    let width = n - 1;
    let states = 1usize << width;
    let mut dp = filled(states * width, u128::MAX)?;
    let mut parent = filled(states * width, u8::MAX)?;
    for end in 0..width {
        if let Some(cost) = arcs[0][end + 1] {
            dp[(1 << end) * width + end] = u128::from(cost);
        }
    }
    for subset in 1..states {
        for end in 0..width {
            let bit = 1 << end;
            if subset & bit == 0 {
                continue;
            }
            let earlier = subset ^ bit;
            if earlier == 0 {
                continue;
            }
            for before in 0..width {
                if earlier & (1 << before) == 0 {
                    continue;
                }
                let prefix = dp[earlier * width + before];
                if prefix == u128::MAX {
                    continue;
                }
                if let Some(edge) = arcs[before + 1][end + 1] {
                    let candidate = prefix + u128::from(edge);
                    let index = subset * width + end;
                    if candidate < dp[index] {
                        dp[index] = candidate;
                        parent[index] = before as u8;
                    }
                }
            }
        }
    }
    let full = states - 1;
    let mut best = u128::MAX;
    let mut last = 0;
    for end in 0..width {
        let prefix = dp[full * width + end];
        if prefix == u128::MAX {
            continue;
        }
        if let Some(edge) = arcs[end + 1][0] {
            let candidate = prefix + u128::from(edge);
            if candidate < best {
                best = candidate;
                last = end;
            }
        }
    }
    if best == u128::MAX {
        return Ok(None);
    }
    let mut vertices = filled(n + 1, 0usize)?;
    let mut subset = full;
    for position in (1..n).rev() {
        vertices[position] = last + 1;
        let before = parent[subset * width + last];
        subset ^= 1 << last;
        if subset != 0 {
            last = usize::from(before);
        }
    }
    Ok(Some(Tour {
        vertices,
        cost: best,
    }))
}

fn filled<T: Clone>(length: usize, value: T) -> Result<Vec<T>, Error> {
    let mut out = Vec::new();
    out.try_reserve_exact(length)
        .map_err(|_| Error::AllocationFailed)?;
    out.resize(length, value);
    Ok(out)
}

#[cfg(test)]
mod tests;
