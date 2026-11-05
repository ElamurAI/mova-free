//! Maximum cardinality search. Tarjan and Yannakakis, SIAM J. Comput. 13 (1984).
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

/// Return selection order; maximal selected-neighbor count, smallest ID on ties.
/// Simple undirected input only. O(V^2+E) time/space including validation.
pub fn maximum_cardinality_search(graph: &[Vec<usize>]) -> Result<Vec<usize>, Error> {
    let _ = matrix(graph, true)?;
    let n = graph.len();
    let mut selected = vec![false; n];
    let mut weight = vec![0; n];
    let mut order = Vec::with_capacity(n);
    for _ in 0..n {
        let v = (0..n)
            .filter(|&v| !selected[v])
            .max_by_key(|&v| (weight[v], std::cmp::Reverse(v)))
            .expect("remaining vertex");
        selected[v] = true;
        order.push(v);
        for &w in &graph[v] {
            if !selected[w] {
                weight[w] += 1;
            }
        }
    }
    Ok(order)
}
#[cfg(test)]
mod tests;
