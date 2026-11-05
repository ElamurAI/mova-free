//! Fixed-point dominators; Aho, Sethi, Ullman, Compilers (1986), data-flow analysis.
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

/// Sorted dominator sets, empty at unreachable vertices. Start dominates itself.
/// Caps precede start then endpoint validation. Duplicates and loops allowed.
/// O(V^2(V+E)) conservative time, O(V^2+E) storage; no recursive traversal.
pub fn dominators(graph: &[Vec<usize>], start: usize) -> Result<Vec<Vec<usize>>, Error> {
    if graph.len() > MAX_VERTICES
        || graph
            .iter()
            .try_fold(0usize, |s, r| s.checked_add(r.len()))
            .is_none_or(|m| m > MAX_EDGES)
    {
        return Err(Error::LimitExceeded);
    }
    if start >= graph.len() {
        return Err(Error::InvalidVertex);
    }
    let a = matrix(graph, false)?;
    let n = graph.len();
    let mut reach = vec![false; n];
    reach[start] = true;
    let mut stack = vec![start];
    while let Some(v) = stack.pop() {
        for &w in &graph[v] {
            if !reach[w] {
                reach[w] = true;
                stack.push(w);
            }
        }
    }
    let mut dom = vec![vec![false; n]; n];
    for v in 0..n {
        if reach[v] {
            dom[v] = reach.clone();
        }
    }
    dom[start].fill(false);
    dom[start][start] = true;
    loop {
        let mut changed = false;
        for v in 0..n {
            if !reach[v] || v == start {
                continue;
            }
            let mut next = reach.clone();
            for p in 0..n {
                if reach[p] && a[p][v] {
                    for (x, &d) in next.iter_mut().zip(&dom[p]) {
                        *x &= d;
                    }
                }
            }
            next[v] = true;
            if next != dom[v] {
                dom[v] = next;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    Ok(dom
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(v, &b)| b.then_some(v))
                .collect()
        })
        .collect())
}
#[cfg(test)]
mod tests;
