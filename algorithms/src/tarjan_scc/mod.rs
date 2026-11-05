//! Original iterative lowlink SCC algorithm; R. E. Tarjan, "Depth-First Search
//! and Linear Graph Algorithms", SIAM Journal on Computing 1(2), 1972.
pub const MAX_VERTICES: usize = 1_000_000;
pub const MAX_EDGES: usize = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
}

/// Validate vertex cap, total arc cap, then endpoints in adjacency order.
/// Returns sorted components ordered by their smallest vertex, compatible with
/// strongly_connected_components::components. Empty, loop and duplicate arcs
/// are supported. Traversal O(V+E), canonical sorting O(V log(V+1)), auxiliary
/// storage O(V) excluding returned components. No recursion or reverse graph.
/// Allocation failure is not returned as a typed error.
pub fn components(graph: &[Vec<usize>]) -> Result<Vec<Vec<usize>>, Error> {
    let n = graph.len();
    if n > MAX_VERTICES {
        return Err(Error::LimitExceeded);
    }
    let mut edges = 0usize;
    for row in graph {
        edges = edges.checked_add(row.len()).ok_or(Error::LimitExceeded)?;
        if edges > MAX_EDGES {
            return Err(Error::LimitExceeded);
        }
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
    let mut index = vec![usize::MAX; n];
    let mut low = vec![0; n];
    let mut active = vec![false; n];
    let mut pending = Vec::new();
    let mut frames = Vec::new();
    let mut clock = 0;
    let mut result = Vec::new();
    for root in 0..n {
        if index[root] != usize::MAX {
            continue;
        }
        index[root] = clock;
        low[root] = clock;
        clock += 1;
        active[root] = true;
        pending.push(root);
        frames.push((root, 0usize));
        while let Some((vertex, next)) = frames.last_mut() {
            let from = *vertex;
            if *next < graph[from].len() {
                let to = graph[from][*next];
                *next += 1;
                if index[to] == usize::MAX {
                    index[to] = clock;
                    low[to] = clock;
                    clock += 1;
                    active[to] = true;
                    pending.push(to);
                    frames.push((to, 0));
                } else if active[to] {
                    low[from] = low[from].min(index[to]);
                }
            } else {
                frames.pop();
                if low[from] == index[from] {
                    let mut component = Vec::new();
                    loop {
                        let member = pending.pop().expect("root remains on SCC stack");
                        active[member] = false;
                        component.push(member);
                        if member == from {
                            break;
                        }
                    }
                    component.sort_unstable();
                    result.push(component);
                }
                if let Some(&(parent, _)) = frames.last() {
                    low[parent] = low[parent].min(low[from]);
                }
            }
        }
    }
    result.sort_unstable_by_key(|component| component[0]);
    Ok(result)
}

#[cfg(test)]
mod tests;
