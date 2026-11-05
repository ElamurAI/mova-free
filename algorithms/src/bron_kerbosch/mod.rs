//! Pivoted Bron-Kerbosch maximum-clique search.
//! Reference: Bron and Kerbosch, Algorithm 457, CACM 16(9), 1973.
//! Original bitset implementation, retaining only one maximum clique.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    TooManyVertices,
    TooManyEdges,
    InvalidEndpoint { edge: usize, vertex: usize },
    SelfLoop { edge: usize },
    RepeatedEdge { edge: usize },
}
/// Returns the lexicographically smallest sorted maximum clique (empty for n=0).
/// Simple undirected graph, labels 0..n, n<=64, at most n(n-1)/2 edges.
/// Caps precede endpoint, loop and duplicate validation in input order.
/// Exact exponential search: conservative O(n^2 2^n) time, O(n^2) auxiliary
/// space including recursive candidate vectors. The cap bounds representation,
/// not practical runtime: adversarial 64-vertex inputs may be very expensive.
pub fn maximum_clique(vertices: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Error> {
    if vertices > 64 {
        return Err(Error::TooManyVertices);
    }
    if edges.len() > vertices * vertices.saturating_sub(1) / 2 {
        return Err(Error::TooManyEdges);
    }
    let mut neighbors = vec![0u64; vertices];
    for (i, &(u, v)) in edges.iter().enumerate() {
        for vertex in [u, v] {
            if vertex >= vertices {
                return Err(Error::InvalidEndpoint { edge: i, vertex });
            }
        }
        if u == v {
            return Err(Error::SelfLoop { edge: i });
        }
        if neighbors[u] & (1u64 << v) != 0 {
            return Err(Error::RepeatedEdge { edge: i });
        }
        neighbors[u] |= 1u64 << v;
        neighbors[v] |= 1u64 << u;
    }
    let mut best = Vec::new();
    let all = if vertices == 64 {
        u64::MAX
    } else {
        (1u64 << vertices) - 1
    };
    search(&neighbors, &mut Vec::new(), all, 0, &mut best);
    Ok(best)
}
fn search(
    neighbors: &[u64],
    current: &mut Vec<usize>,
    mut possible: u64,
    mut excluded: u64,
    best: &mut Vec<usize>,
) {
    if current.len() + (possible.count_ones() as usize) < best.len() {
        return;
    }
    if possible == 0 && excluded == 0 {
        let mut candidate = current.clone();
        candidate.sort_unstable();
        if candidate.len() > best.len() || (candidate.len() == best.len() && candidate < *best) {
            *best = candidate;
        }
        return;
    }
    let mut union = possible | excluded;
    let mut pivot = None;
    let mut score = 0;
    while union != 0 {
        let u = union.trailing_zeros() as usize;
        union &= union - 1;
        let count = (possible & neighbors[u]).count_ones();
        if pivot.is_none() || count > score {
            pivot = Some(u);
            score = count;
        }
    }
    let mut branches = possible & !pivot.map_or(0, |u| neighbors[u]);
    while branches != 0 {
        let v = branches.trailing_zeros() as usize;
        let bit = 1u64 << v;
        branches &= !bit;
        current.push(v);
        search(
            neighbors,
            current,
            possible & neighbors[v],
            excluded & neighbors[v],
            best,
        );
        current.pop();
        possible &= !bit;
        excluded |= bit;
    }
}
#[cfg(test)]
mod tests;
