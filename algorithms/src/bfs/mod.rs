//! Unweighted directed shortest-path distances; malformed edges are rejected globally.
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BfsError {
    InvalidSource {
        source: usize,
        vertices: usize,
    },
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
}

/// Returns minimum edge counts, with None for unreachable vertices.
/// Source validation precedes edge validation, which follows adjacency order.
/// Time O(V+E), auxiliary space O(V). Inputs are never changed.
pub fn bfs(graph: &[Vec<usize>], source: usize) -> Result<Vec<Option<usize>>, BfsError> {
    let n = graph.len();
    if source >= n {
        return Err(BfsError::InvalidSource {
            source,
            vertices: n,
        });
    }
    for (from, edges) in graph.iter().enumerate() {
        for &to in edges {
            if to >= n {
                return Err(BfsError::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
        }
    }
    let mut distances = vec![None; n];
    let mut queue = VecDeque::new();
    distances[source] = Some(0);
    queue.push_back(source);
    while let Some(from) = queue.pop_front() {
        for &to in &graph[from] {
            if distances[to].is_none() {
                // A newly discovered shortest path is simple and has at most n-1 edges.
                distances[to] = Some(distances[from].unwrap() + 1);
                queue.push_back(to);
            }
        }
    }
    Ok(distances)
}

#[cfg(test)]
mod tests;
