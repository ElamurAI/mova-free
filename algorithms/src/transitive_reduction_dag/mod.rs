//! DAG transitive reduction; Aho, Garey, Ullman, SIAM J. Comput. 1 (1972).
pub const MAX_VERTICES: usize = 256;
pub const MAX_EDGES: usize = 65_536;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidVertex,
    NotSimpleUndirected,
    Cycle,
}
fn matrix(graph: &[Vec<usize>], undirected: bool) -> Result<Vec<Vec<bool>>, Error> {
    let n = graph.len();
    if n > MAX_VERTICES
        || graph
            .iter()
            .try_fold(0usize, |s, r| s.checked_add(r.len()))
            .is_none_or(|m| m > MAX_EDGES)
    {
        return Err(Error::LimitExceeded);
    }
    let mut a = vec![vec![false; n]; n];
    for (u, row) in graph.iter().enumerate() {
        for &v in row {
            if v >= n {
                return Err(Error::InvalidVertex);
            }
            if undirected && (u == v || a[u][v]) {
                return Err(Error::NotSimpleUndirected);
            }
            a[u][v] = true;
        }
    }
    if undirected {
        for (u, row) in a.iter().enumerate() {
            for (v, &cell) in row.iter().enumerate() {
                if cell != a[v][u] {
                    return Err(Error::NotSimpleUndirected);
                }
            }
        }
    }
    Ok(a)
}

/// Unique reduction as sorted adjacency rows. Duplicate edges normalize.
/// Caps then endpoints then cycles are validated. O(V^3+E) time, O(V^2) memory.
pub fn transitive_reduction_dag(graph: &[Vec<usize>]) -> Result<Vec<Vec<usize>>, Error> {
    let a = matrix(graph, false)?;
    let n = graph.len();
    let mut reach = a.clone();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                reach[i][j] |= reach[i][k] && reach[k][j];
            }
        }
    }
    if (0..n).any(|v| reach[v][v]) {
        return Err(Error::Cycle);
    }
    Ok((0..n)
        .map(|u| {
            (0..n)
                .filter(|&v| a[u][v] && !(0..n).any(|w| w != v && a[u][w] && reach[w][v]))
                .collect()
        })
        .collect())
}
#[cfg(test)]
mod tests;
