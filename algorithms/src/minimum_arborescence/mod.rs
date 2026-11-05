//! Original Chu-Liu/Edmonds contraction, objective only.
//! Chu and Liu, On the shortest arborescence of a directed graph (1965);
//! Edmonds, Optimum branchings (1967).
pub const MAX_VERTICES: usize = 512;
pub const MAX_EDGES: usize = 131_072;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: i64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidRoot,
    InvalidEdge { index: usize },
    Overflow,
}
/// Outward spanning arborescence rooted at root; None means infeasible.
/// Limits, root, edges validated in order. Loops ignored; first cost tie wins.
/// O(VE+V²) time, O(V+E) memory; allocation may abort.
pub fn minimum_arborescence(
    vertices: usize,
    root: usize,
    edges: &[Edge],
) -> Result<Option<i128>, Error> {
    if vertices > MAX_VERTICES || edges.len() > MAX_EDGES {
        return Err(Error::LimitExceeded);
    }
    if root >= vertices {
        return Err(Error::InvalidRoot);
    }
    for (index, e) in edges.iter().enumerate() {
        if e.from >= vertices || e.to >= vertices {
            return Err(Error::InvalidEdge { index });
        }
    }
    let mut arcs: Vec<_> = edges
        .iter()
        .map(|e| (e.from, e.to, i128::from(e.weight)))
        .collect();
    let mut n = vertices;
    let mut r = root;
    let mut total = 0i128;
    loop {
        let mut incoming = vec![i128::MAX; n];
        let mut parent = vec![0; n];
        for &(u, v, w) in &arcs {
            if u != v && v != r && w < incoming[v] {
                incoming[v] = w;
                parent[v] = u;
            }
        }
        incoming[r] = 0;
        if incoming.contains(&i128::MAX) {
            return Ok(None);
        }
        for &w in &incoming {
            total = total.checked_add(w).ok_or(Error::Overflow)?;
        }
        let (mut id, mut count, visits) = cycle_components(&parent, r);
        debug_assert!(visits < n);
        if count == 0 {
            return Ok(Some(total));
        }
        for x in &mut id {
            if *x == usize::MAX {
                *x = count;
                count += 1;
            }
        }
        let mut next = Vec::new();
        for (u, v, w) in arcs {
            if id[u] != id[v] {
                next.push((
                    id[u],
                    id[v],
                    w.checked_sub(incoming[v]).ok_or(Error::Overflow)?,
                ));
            }
        }
        r = id[r];
        n = count;
        arcs = next;
    }
}
// Count primary parent advances to make the linear scan invariant observable.
// Each nonroot vertex is stamped once; cycle labeling visits disjoint cycles.
fn cycle_components(parent: &[usize], root: usize) -> (Vec<usize>, usize, usize) {
    let n = parent.len();
    let mut id = vec![usize::MAX; n];
    let mut seen = vec![usize::MAX; n];
    let mut count = 0;
    let mut visits = 0;
    for start in 0..n {
        let mut v = start;
        while v != root && seen[v] == usize::MAX {
            seen[v] = start;
            visits += 1;
            v = parent[v];
        }
        // An earlier traversal has already classified its reachable cycle.
        // Only a vertex stamped by this start closes a newly discovered cycle.
        if v != root && seen[v] == start && id[v] == usize::MAX {
            id[v] = count;
            let mut u = parent[v];
            while u != v {
                id[u] = count;
                u = parent[u];
            }
            count += 1;
        }
    }
    (id, count, visits)
}
#[cfg(test)]
mod tests;
