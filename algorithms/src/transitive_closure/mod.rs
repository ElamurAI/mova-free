//! Packed reflexive reachability using Warshall's recurrence (JACM 9(1), 1962).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ResourceLimit,
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
}

/// A directed reflexive reachability matrix with private packed rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closure {
    rows: Vec<Vec<u64>>,
}
impl Closure {
    pub fn vertices(&self) -> usize {
        self.rows.len()
    }
    /// None for either invalid endpoint; Some(false) for valid unreachable pairs.
    pub fn reachable(&self, from: usize, to: usize) -> Option<bool> {
        if to >= self.vertices() {
            return None;
        }
        self.rows
            .get(from)
            .map(|row| row[to / 64] & (1u64 << (to % 64)) != 0)
    }
}
/// Limits precede adjacency-order endpoint validation: at most 8192 vertices
/// and 1,000,000 arcs. Duplicate arcs and self-loops do not change the result.
/// Empty paths make every vertex reachable from itself.
/// O(V² ceil(V/64)+E) time, O(V ceil(V/64)) storage.
/// Allocation failures are outside the typed error contract.
pub fn transitive_closure(graph: &[Vec<usize>]) -> Result<Closure, Error> {
    let n = graph.len();
    if n > 8192
        || graph
            .iter()
            .try_fold(0usize, |a, row| a.checked_add(row.len()))
            .is_none_or(|e| e > 1_000_000)
    {
        return Err(Error::ResourceLimit);
    }
    for (from, row) in graph.iter().enumerate() {
        for &to in row {
            if to >= n {
                return Err(Error::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
        }
    }
    let mut rows = vec![vec![0u64; n.div_ceil(64)]; n];
    for (u, row) in graph.iter().enumerate() {
        rows[u][u / 64] |= 1 << (u % 64);
        for &v in row {
            rows[u][v / 64] |= 1 << (v % 64);
        }
    }
    for k in 0..n {
        let pivot = rows[k].clone();
        for row in &mut rows {
            if row[k / 64] & (1 << (k % 64)) != 0 {
                for (word, &via) in row.iter_mut().zip(&pivot) {
                    *word |= via;
                }
            }
        }
    }
    Ok(Closure { rows })
}
#[cfg(test)]
mod tests;
