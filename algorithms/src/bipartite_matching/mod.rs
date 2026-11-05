//! Deterministic maximum cardinality matching of an explicitly bipartite graph.
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BipartiteMatchingError {
    InvalidEdge {
        left: usize,
        right: usize,
        right_vertices: usize,
    },
}

/// Each row lists right neighbors of one left vertex. Returns its matched right
/// vertex, or None. Duplicates are allowed and all endpoints are validated first.
/// Ties follow left index, FIFO search and adjacency order; the result is not
/// promised lexicographically minimal or invariant under adjacency permutations.
/// Iterative augmenting paths take O((L+1)(L+R+E)) time and O(L+R) space.
pub fn bipartite_matching(
    graph: &[Vec<usize>],
    right_vertices: usize,
) -> Result<Vec<Option<usize>>, BipartiteMatchingError> {
    for (left, edges) in graph.iter().enumerate() {
        for &right in edges {
            if right >= right_vertices {
                return Err(BipartiteMatchingError::InvalidEdge {
                    left,
                    right,
                    right_vertices,
                });
            }
        }
    }
    let mut left_mate = vec![None; graph.len()];
    let mut right_mate: Vec<Option<usize>> = vec![None; right_vertices];
    for root in 0..graph.len() {
        let mut parent = vec![None; right_vertices];
        let mut seen = vec![false; graph.len()];
        let mut queue = VecDeque::from([root]);
        seen[root] = true;
        let mut free = None;
        'search: while let Some(left) = queue.pop_front() {
            for &right in &graph[left] {
                if parent[right].is_some() {
                    continue;
                }
                parent[right] = Some(left);
                if let Some(next) = right_mate[right] {
                    if !seen[next] {
                        seen[next] = true;
                        queue.push_back(next);
                    }
                } else {
                    free = Some(right);
                    break 'search;
                }
            }
        }
        while let Some(right) = free {
            let left = parent[right].expect("discovered path has a predecessor");
            free = left_mate[left];
            left_mate[left] = Some(right);
            right_mate[right] = Some(left);
        }
    }
    Ok(left_mate)
}

#[cfg(test)]
mod tests;
