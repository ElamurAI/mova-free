//! Lexicographically smallest topological ordering of a directed multigraph.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologicalSortError {
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
    Cycle,
}

/// Kahn's algorithm with the smallest currently available vertex chosen first.
/// All edges are checked before processing. Parallel edges count separately.
/// Empty input has the empty ordering. Inputs are unchanged.
/// Time O(E+V log(V+1)); auxiliary space O(V).
pub fn topological_sort(graph: &[Vec<usize>]) -> Result<Vec<usize>, TopologicalSortError> {
    let n = graph.len();
    let mut indegree = vec![0usize; n];
    for (from, edges) in graph.iter().enumerate() {
        for &to in edges {
            if to >= n {
                return Err(TopologicalSortError::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
            // Total edges in a materialized graph fit its addressable allocation.
            indegree[to] += 1;
        }
    }
    let mut ready = BinaryHeap::new();
    for (vertex, &degree) in indegree.iter().enumerate() {
        if degree == 0 {
            ready.push(Reverse(vertex));
        }
    }
    let mut order = Vec::with_capacity(n);
    while let Some(Reverse(from)) = ready.pop() {
        order.push(from);
        for &to in &graph[from] {
            indegree[to] -= 1;
            if indegree[to] == 0 {
                ready.push(Reverse(to));
            }
        }
    }
    if order.len() == n {
        Ok(order)
    } else {
        Err(TopologicalSortError::Cycle)
    }
}

#[cfg(test)]
mod tests;
