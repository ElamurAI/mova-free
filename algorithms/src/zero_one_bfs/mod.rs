//! Exact shortest distances for directed graphs with binary edge weights.
//! Original implementation of the deque specialization of Dijkstra's algorithm;
//! mathematical background: cp-algorithms, "0-1 BFS" (no source code copied).
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidSource {
        source: usize,
        vertices: usize,
    },
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
    InvalidWeight {
        from: usize,
        to: usize,
        weight: u8,
    },
    DistanceOverflow,
}

/// Returns None exactly for unreachable vertices, and Some(0) at source.
/// Checks source first, then every edge in row order (endpoint before weight),
/// including unreachable rows. Parallel edges, loops and cycles are accepted.
/// Strict improvements go to the deque front for weight zero, back for one.
/// Time O(V+E), auxiliary space O(V+E), assuming successful allocations.
pub fn zero_one_bfs(
    graph: &[Vec<(usize, u8)>],
    source: usize,
) -> Result<Vec<Option<usize>>, Error> {
    let n = graph.len();
    if source >= n {
        return Err(Error::InvalidSource {
            source,
            vertices: n,
        });
    }
    for (from, edges) in graph.iter().enumerate() {
        for &(to, weight) in edges {
            if to >= n {
                return Err(Error::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
            if weight > 1 {
                return Err(Error::InvalidWeight { from, to, weight });
            }
        }
    }
    let mut distance = vec![None; n];
    distance[source] = Some(0usize);
    let mut queue = VecDeque::from([(source, 0usize)]);
    while let Some((from, known)) = queue.pop_front() {
        if distance[from] != Some(known) {
            continue;
        }
        for &(to, weight) in &graph[from] {
            let candidate = known
                .checked_add(usize::from(weight))
                .ok_or(Error::DistanceOverflow)?;
            if distance[to].is_none_or(|old| candidate < old) {
                distance[to] = Some(candidate);
                if weight == 0 {
                    queue.push_front((to, candidate));
                } else {
                    queue.push_back((to, candidate));
                }
            }
        }
    }
    Ok(distance)
}

#[cfg(test)]
mod tests;
