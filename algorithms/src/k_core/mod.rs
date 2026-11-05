//! Degree peeling k-core. S. B. Seidman, Social Networks 5 (1983), 269–287.
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

/// Ascending vertices of the unique maximal induced subgraph of minimum degree k.
/// Simple undirected input only. O(V^2+E) time/space including validation.
pub fn k_core(graph: &[Vec<usize>], k: usize) -> Result<Vec<usize>, Error> {
    let _ = matrix(graph, true)?;
    let n = graph.len();
    let mut degree: Vec<_> = graph.iter().map(Vec::len).collect();
    let mut live = vec![true; n];
    let mut queue = std::collections::VecDeque::new();
    for v in 0..n {
        if degree[v] < k {
            live[v] = false;
            queue.push_back(v);
        }
    }
    while let Some(v) = queue.pop_front() {
        for &w in &graph[v] {
            if live[w] {
                degree[w] -= 1;
                if degree[w] < k {
                    live[w] = false;
                    queue.push_back(w);
                }
            }
        }
    }
    Ok((0..n).filter(|&v| live[v]).collect())
}
#[cfg(test)]
mod tests;
