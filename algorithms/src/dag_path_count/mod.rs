//! Exact checked path counting by dynamic programming in topological order.
//! Concept: Kahn (1962) ordering, followed by the standard DAG path recurrence.
use crate::topological_sort::{TopologicalSortError, topological_sort};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidSource { source: usize, vertices: usize },
    Graph(TopologicalSortError),
    Overflow { from: usize, to: usize },
}

/// Counts source-to-vertex paths, including the empty path to source.
/// Parallel edges are distinct choices; unreachable vertices have zero counts.
/// Checks source first, then all endpoints and global acyclicity, even outside
/// the reachable component. Overflow is the first failed addition in the
/// lexicographically smallest topological order and input adjacency order.
/// O(E+V log(V+1)) time and O(V) auxiliary space; allocation failure is excluded.
pub fn dag_path_count(graph: &[Vec<usize>], source: usize) -> Result<Vec<u128>, Error> {
    if source >= graph.len() {
        return Err(Error::InvalidSource {
            source,
            vertices: graph.len(),
        });
    }
    let order = topological_sort(graph).map_err(Error::Graph)?;
    let mut counts = vec![0u128; graph.len()];
    counts[source] = 1;
    for from in order {
        for &to in &graph[from] {
            counts[to] = counts[to]
                .checked_add(counts[from])
                .ok_or(Error::Overflow { from, to })?;
        }
    }
    Ok(counts)
}

#[cfg(test)]
mod tests;
