//! Articulation vertices of an undirected multigraph, using iterative DFS.
pub use crate::bridges::{Edge, Error};

/// Return ascending vertices whose deletion increases component count.
/// Deletion removes the vertex and all incident edges. Isolates and single-edge
/// components contribute no articulation vertex; parallel edges and loops work.
/// O(vertices + edges.len()) time and auxiliary space, without recursion.
pub fn articulation_points(vertices: usize, edges: &[Edge]) -> Result<Vec<usize>, Error> {
    let graph = crate::bridges::adjacency(vertices, edges)?;
    let mut seen = vec![usize::MAX; vertices];
    let mut low = vec![0; vertices];
    let mut parent = vec![None; vertices];
    let mut children = vec![0; vertices];
    let mut next = vec![0; vertices];
    let mut cut = vec![false; vertices];
    let mut clock = 0;
    for root in 0..vertices {
        if seen[root] != usize::MAX {
            continue;
        }
        seen[root] = clock;
        low[root] = clock;
        clock += 1;
        let mut stack = vec![root];
        while let Some(&v) = stack.last() {
            if next[v] < graph[v].len() {
                let (w, edge) = graph[v][next[v]];
                next[v] += 1;
                if parent[v].is_some_and(|(_, id)| id == edge) {
                    continue;
                }
                if seen[w] == usize::MAX {
                    parent[w] = Some((v, edge));
                    children[v] += 1;
                    seen[w] = clock;
                    low[w] = clock;
                    clock += 1;
                    stack.push(w);
                } else {
                    low[v] = low[v].min(seen[w]);
                }
            } else {
                stack.pop();
                if let Some((p, _)) = parent[v] {
                    if parent[p].is_some() && low[v] >= seen[p] {
                        cut[p] = true;
                    }
                    low[p] = low[p].min(low[v]);
                } else {
                    cut[v] = children[v] > 1;
                }
            }
        }
    }
    Ok(cut
        .iter()
        .enumerate()
        .filter_map(|(v, &yes)| yes.then_some(v))
        .collect())
}

#[cfg(test)]
mod tests;
