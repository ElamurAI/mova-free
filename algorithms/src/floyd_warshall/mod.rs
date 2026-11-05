//! Exact all-pairs shortest distances for nonnegative directed weights.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
    DistanceOverflow,
}

/// Validates all endpoints, then returns minimum u128 path costs or None.
/// The empty path has cost zero; parallel arcs use their minimum weight.
/// Negative weights are excluded by u64. Candidate sums use checked addition.
/// Time O(V^3+E), auxiliary space O(V^2), including the returned matrix.
pub fn floyd_warshall(graph: &[Vec<(usize, u64)>]) -> Result<Vec<Vec<Option<u128>>>, Error> {
    let n = graph.len();
    for (from, edges) in graph.iter().enumerate() {
        for &(to, _) in edges {
            if to >= n {
                return Err(Error::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
        }
    }
    let mut distances = vec![vec![None; n]; n];
    for (from, edges) in graph.iter().enumerate() {
        distances[from][from] = Some(0);
        for &(to, weight) in edges {
            let weight = u128::from(weight);
            distances[from][to] = Some(distances[from][to].map_or(weight, |old| old.min(weight)));
        }
    }
    for via in 0..n {
        // A fixed pivot row keeps the recurrence independent of mutable row borrowing.
        let pivot = distances[via].clone();
        for row in &mut distances {
            if let Some(left) = row[via] {
                for (cell, &right) in row.iter_mut().zip(&pivot) {
                    if let Some(right) = right {
                        let candidate = left.checked_add(right).ok_or(Error::DistanceOverflow)?;
                        if cell.is_none_or(|old| candidate < old) {
                            *cell = Some(candidate);
                        }
                    }
                }
            }
        }
    }
    Ok(distances)
}
#[cfg(test)]
mod tests;
