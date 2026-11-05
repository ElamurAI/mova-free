//! Exact single-source shortest paths with signed directed edge weights.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellmanFordError {
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
    ReachableNegativeCycle,
}

/// Returns minimum walk weights, or an error if a negative cycle is reachable.
/// Unreachable negative cycles are ignored. Source validation precedes validation
/// of every edge in adjacency order, including unreachable edges. Parallel edges
/// and self-loops are allowed. Checked i128 sums never silently wrap.
/// Synchronous rounds take O(V(V+E)) time and O(V) auxiliary space.
pub fn bellman_ford(
    graph: &[Vec<(usize, i64)>],
    source: usize,
) -> Result<Vec<Option<i128>>, BellmanFordError> {
    let n = graph.len();
    if source >= n {
        return Err(BellmanFordError::InvalidSource {
            source,
            vertices: n,
        });
    }
    for (from, edges) in graph.iter().enumerate() {
        for &(to, _) in edges {
            if to >= n {
                return Err(BellmanFordError::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
        }
    }
    let mut distances = vec![None; n];
    distances[source] = Some(0i128);
    for round in 0..n {
        let mut next = distances.clone();
        let mut changed = false;
        for (from, edges) in graph.iter().enumerate() {
            let Some(distance) = distances[from] else {
                continue;
            };
            for &(to, weight) in edges {
                let candidate = distance
                    .checked_add(i128::from(weight))
                    .ok_or(BellmanFordError::DistanceOverflow)?;
                if next[to].is_none_or(|old| candidate < old) {
                    next[to] = Some(candidate);
                    changed = true;
                }
            }
        }
        if !changed {
            return Ok(distances);
        }
        if round == n - 1 {
            return Err(BellmanFordError::ReachableNegativeCycle);
        }
        distances = next;
    }
    Ok(distances)
}

#[cfg(test)]
mod tests;
