//! Signed shortest distances in a directed acyclic multigraph.
//! Original implementation of topological relaxation; see Cormen, Leiserson,
//! Rivest and Stein, Introduction to Algorithms, 3rd ed., section 24.2.
use crate::topological_sort::{TopologicalSortError, topological_sort};

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
    Cycle,
    DistanceOverflow,
}

/// Source validation precedes global endpoint validation, then global cycle
/// detection (including unreachable cycles). None denotes unreachable vertices.
/// Reuses the deterministic topological sorter; checked i128 sums widen i64 weights.
/// Time O(E+V log(V+1)); auxiliary space O(V+E). Inputs are unchanged.
pub fn shortest_paths(
    graph: &[Vec<(usize, i64)>],
    source: usize,
) -> Result<Vec<Option<i128>>, Error> {
    let n = graph.len();
    if source >= n {
        return Err(Error::InvalidSource {
            source,
            vertices: n,
        });
    }
    let endpoints: Vec<Vec<usize>> = graph
        .iter()
        .map(|row| row.iter().map(|&(to, _)| to).collect())
        .collect();
    let order = topological_sort(&endpoints).map_err(|error| match error {
        TopologicalSortError::InvalidEdge { from, to, vertices } => {
            Error::InvalidEdge { from, to, vertices }
        }
        TopologicalSortError::Cycle => Error::Cycle,
    })?;
    let mut distance = vec![None; n];
    distance[source] = Some(0i128);
    for from in order {
        if let Some(known) = distance[from] {
            for &(to, weight) in &graph[from] {
                let candidate = known
                    .checked_add(i128::from(weight))
                    .ok_or(Error::DistanceOverflow)?;
                if distance[to].is_none_or(|old| candidate < old) {
                    distance[to] = Some(candidate);
                }
            }
        }
    }
    Ok(distance)
}

#[cfg(test)]
mod tests;
