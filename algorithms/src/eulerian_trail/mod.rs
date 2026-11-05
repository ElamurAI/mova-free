//! Deterministic undirected Euler trails by iterative Hierholzer traversal.
pub use crate::bridges::{Edge, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trail {
    /// Consecutive endpoints of the returned traversal.
    pub vertices: Vec<usize>,
    /// Original input edge identities, each used exactly once.
    pub edges: Vec<usize>,
}

/// Return a trail using all edges, or None if one does not exist.
/// Start at the smallest odd-degree vertex, otherwise the smallest incident
/// vertex. Traverse available adjacency entries in original edge order.
/// Isolates do not prevent a trail. Empty edges give [0] when vertices > 0,
/// or an empty vertex list when vertices == 0. Loops count twice toward degree.
/// Validation precedes existence checks. O(vertices + edges.len()) time/space.
pub fn eulerian_trail(vertices: usize, edges: &[Edge]) -> Result<Option<Trail>, Error> {
    let graph = crate::bridges::adjacency(vertices, edges)?;
    if edges.is_empty() {
        return Ok(Some(Trail {
            vertices: if vertices == 0 { vec![] } else { vec![0] },
            edges: vec![],
        }));
    }
    let odd: Vec<_> = (0..vertices)
        .filter(|&v| !graph[v].len().is_multiple_of(2))
        .collect();
    if !odd.is_empty() && odd.len() != 2 {
        return Ok(None);
    }
    let start = odd
        .first()
        .copied()
        .or_else(|| (0..vertices).find(|&v| !graph[v].is_empty()))
        .expect("nonempty valid edges");
    let mut used = vec![false; edges.len()];
    let mut next = vec![0; vertices];
    let mut stack = vec![(start, None)];
    let mut trail = Trail {
        vertices: Vec::new(),
        edges: Vec::new(),
    };
    while let Some(&(v, incoming)) = stack.last() {
        while next[v] < graph[v].len() && used[graph[v][next[v]].1] {
            next[v] += 1;
        }
        if next[v] == graph[v].len() {
            stack.pop();
            trail.vertices.push(v);
            if let Some(edge) = incoming {
                trail.edges.push(edge);
            }
        } else {
            let (w, edge) = graph[v][next[v]];
            next[v] += 1;
            used[edge] = true;
            stack.push((w, Some(edge)));
        }
    }
    if trail.edges.len() != edges.len() {
        return Ok(None);
    }
    trail.vertices.reverse();
    trail.edges.reverse();
    Ok(Some(trail))
}

#[cfg(test)]
mod tests;
