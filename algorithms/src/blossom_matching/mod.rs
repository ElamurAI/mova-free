//! Original alternating-forest blossom contraction.
//! Reference: Edmonds, Paths, trees, and flowers, Canadian J. Math. 17 (1965).
use std::collections::VecDeque;
pub const MAX_VERTICES: usize = 512;
pub const MAX_EDGES: usize = 131_072;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidEdge { index: usize },
}
/// Maximum cardinality matching as symmetric original zero-based mates.
/// Loops ignored, parallel edges collapsed, adjacency sorted for deterministic
/// ties. Limits precede first endpoint error. O(V³+E log E) time/O(V+E) space.
/// Empty graph succeeds. Allocation failure is process-level, no callbacks.
pub fn maximum_matching(
    vertices: usize,
    edges: &[(usize, usize)],
) -> Result<Vec<Option<usize>>, Error> {
    if vertices > MAX_VERTICES || edges.len() > MAX_EDGES {
        return Err(Error::LimitExceeded);
    }
    let mut graph = vec![Vec::new(); vertices];
    for (index, &(u, v)) in edges.iter().enumerate() {
        if u >= vertices || v >= vertices {
            return Err(Error::InvalidEdge { index });
        }
        if u != v {
            graph[u].push(v);
            graph[v].push(u);
        }
    }
    for adj in &mut graph {
        adj.sort_unstable();
        adj.dedup();
    }
    let mut mate = vec![None; vertices];
    for root in 0..vertices {
        if mate[root].is_none() {
            augment(root, &graph, &mut mate);
        }
    }
    Ok(mate)
}
fn ancestor(
    mut a: usize,
    mut b: usize,
    base: &[usize],
    parent: &[usize],
    mate: &[Option<usize>],
) -> usize {
    let mut seen = vec![false; base.len()];
    loop {
        a = base[a];
        seen[a] = true;
        if let Some(m) = mate[a] {
            a = parent[m];
        } else {
            break;
        }
    }
    loop {
        b = base[b];
        if seen[b] {
            return b;
        }
        b = parent[mate[b].expect("alternating forest root")];
    }
}
fn mark(
    mut v: usize,
    base_vertex: usize,
    mut child: usize,
    base: &[usize],
    parent: &mut [usize],
    mate: &[Option<usize>],
    flower: &mut [bool],
) {
    while base[v] != base_vertex {
        let m = mate[v].expect("nonroot blossom path");
        flower[base[v]] = true;
        flower[base[m]] = true;
        parent[v] = child;
        child = m;
        v = parent[m];
    }
}
fn augment(root: usize, graph: &[Vec<usize>], mate: &mut [Option<usize>]) {
    let n = graph.len();
    let mut parent = vec![usize::MAX; n];
    let mut base: Vec<_> = (0..n).collect();
    let mut used = vec![false; n];
    used[root] = true;
    let mut queue = VecDeque::from([root]);
    while let Some(v) = queue.pop_front() {
        for &u in &graph[v] {
            if base[v] == base[u] || mate[v] == Some(u) {
                continue;
            }
            if u == root || mate[u].is_some_and(|m| parent[m] != usize::MAX) {
                let b = ancestor(v, u, &base, &parent, mate);
                let mut flower = vec![false; n];
                mark(v, b, u, &base, &mut parent, mate, &mut flower);
                mark(u, b, v, &base, &mut parent, mate, &mut flower);
                for i in 0..n {
                    if flower[base[i]] {
                        base[i] = b;
                        if !used[i] {
                            used[i] = true;
                            queue.push_back(i);
                        }
                    }
                }
            } else if parent[u] == usize::MAX {
                parent[u] = v;
                if let Some(m) = mate[u] {
                    used[m] = true;
                    queue.push_back(m);
                } else {
                    let mut x = u;
                    loop {
                        let p = parent[x];
                        let next = mate[p];
                        mate[x] = Some(p);
                        mate[p] = Some(x);
                        if let Some(y) = next {
                            x = y;
                        } else {
                            return;
                        }
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests;
