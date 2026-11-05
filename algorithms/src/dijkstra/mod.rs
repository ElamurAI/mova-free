//! Exact nonnegative directed shortest paths using a lazy minimum heap.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DijkstraError {
    InvalidSource {
        source: usize,
        vertices: usize,
    },
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
    DistanceOverflow,
}

/// Returns exact u128 distances, including paths whose cost exceeds u64::MAX.
/// Source is checked first, then all edges in adjacency order, including unreachable ones.
/// Checked addition prevents wrapping even on hypothetical unusually wide platforms.
/// Time O(V+E log(E+1)); auxiliary space O(V+E), including stale heap entries.
pub fn dijkstra(
    graph: &[Vec<(usize, u64)>],
    source: usize,
) -> Result<Vec<Option<u128>>, DijkstraError> {
    let n = graph.len();
    if source >= n {
        return Err(DijkstraError::InvalidSource {
            source,
            vertices: n,
        });
    }
    for (from, edges) in graph.iter().enumerate() {
        for &(to, _) in edges {
            if to >= n {
                return Err(DijkstraError::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
        }
    }
    let mut distances = vec![None; n];
    let mut heap = BinaryHeap::new();
    distances[source] = Some(0);
    heap.push(Reverse((0u128, source)));
    while let Some(Reverse((distance, from))) = heap.pop() {
        if distances[from] != Some(distance) {
            continue;
        }
        for &(to, weight) in &graph[from] {
            let candidate = distance
                .checked_add(u128::from(weight))
                .ok_or(DijkstraError::DistanceOverflow)?;
            if distances[to].is_none_or(|old| candidate < old) {
                distances[to] = Some(candidate);
                heap.push(Reverse((candidate, to)));
            }
        }
    }
    Ok(distances)
}

#[cfg(test)]
mod tests;
