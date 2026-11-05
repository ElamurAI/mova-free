//! Bridge edge identities of an undirected multigraph, using iterative lowlink DFS.

/// An undirected edge; its position in the input is its stable identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
}

/// The first invalid endpoint, scanning edges in order and u before v.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidEndpoint {
        edge: usize,
        vertex: usize,
        vertices: usize,
    },
}

pub(crate) fn adjacency(
    vertices: usize,
    edges: &[Edge],
) -> Result<Vec<Vec<(usize, usize)>>, Error> {
    for (id, edge) in edges.iter().enumerate() {
        for vertex in [edge.u, edge.v] {
            if vertex >= vertices {
                return Err(Error::InvalidEndpoint {
                    edge: id,
                    vertex,
                    vertices,
                });
            }
        }
    }
    let mut graph = vec![Vec::new(); vertices];
    for (id, edge) in edges.iter().enumerate() {
        graph[edge.u].push((edge.v, id));
        graph[edge.v].push((edge.u, id));
    }
    Ok(graph)
}

/// Return ascending original edge IDs whose removal increases component count.
/// Loops are never bridges; parallel edges are distinct. Isolates are supported.
/// O(vertices + edges.len()) time and auxiliary space, with no recursive DFS.
pub fn bridges(vertices: usize, edges: &[Edge]) -> Result<Vec<usize>, Error> {
    let graph = adjacency(vertices, edges)?;
    let mut seen = vec![usize::MAX; vertices];
    let mut low = vec![0; vertices];
    let mut parent = vec![None; vertices];
    let mut next = vec![0; vertices];
    let mut bridge = vec![false; edges.len()];
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
                let (w, id) = graph[v][next[v]];
                next[v] += 1;
                if parent[v].is_some_and(|(_, edge)| edge == id) {
                    continue;
                }
                if seen[w] == usize::MAX {
                    parent[w] = Some((v, id));
                    seen[w] = clock;
                    low[w] = clock;
                    clock += 1;
                    stack.push(w);
                } else {
                    low[v] = low[v].min(seen[w]);
                }
            } else {
                stack.pop();
                if let Some((p, edge)) = parent[v] {
                    bridge[edge] = low[v] > seen[p];
                    low[p] = low[p].min(low[v]);
                }
            }
        }
    }
    Ok(bridge
        .iter()
        .enumerate()
        .filter_map(|(id, &yes)| yes.then_some(id))
        .collect())
}

#[cfg(test)]
pub(crate) mod tests;
