//! Minimum bipartite vertex cover by alternating reachability.
//! Attribution: D. König, "Gráfok és mátrixok", Matematikai és Fizikai Lapok
//! 38 (1931), 116–119 (matching/cover equality for bipartite graphs).
//! Original implementation; matching is delegated to this crate's Hopcroft–Karp.
use crate::hopcroft_karp;
pub use hopcroft_karp::{Error, MAX_EDGES, MAX_VERTICES};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cover {
    /// Increasing left vertex indices.
    pub left: Vec<usize>,
    /// Increasing right vertex indices.
    pub right: Vec<usize>,
}

/// Validate combined vertex budget, edge budget, then endpoints via matching.
/// Duplicate edges are allowed. Returns a deterministic minimum cover; no
/// lexicographically minimum cover claim. Empty partitions are accepted.
/// Time O((L+R+E)*sqrt(L+R+1)); auxiliary space O(L+R).
/// Allocation failure is outside the typed error contract.
pub fn minimum_vertex_cover(graph: &[Vec<usize>], right_vertices: usize) -> Result<Cover, Error> {
    let mates = hopcroft_karp::maximum_matching(graph, right_vertices)?;
    let mut reverse = vec![None; right_vertices];
    let mut seen_left = vec![false; graph.len()];
    let mut seen_right = vec![false; right_vertices];
    let mut queue = std::collections::VecDeque::new();
    for (left, mate) in mates.iter().enumerate() {
        if let Some(right) = *mate {
            reverse[right] = Some(left);
        } else {
            seen_left[left] = true;
            queue.push_back(left);
        }
    }
    while let Some(left) = queue.pop_front() {
        for &right in &graph[left] {
            if mates[left] == Some(right) || seen_right[right] {
                continue;
            }
            seen_right[right] = true;
            if let Some(next) = reverse[right]
                && !seen_left[next]
            {
                seen_left[next] = true;
                queue.push_back(next);
            }
        }
    }
    Ok(Cover {
        left: (0..graph.len()).filter(|&left| !seen_left[left]).collect(),
        right: (0..right_vertices)
            .filter(|&right| seen_right[right])
            .collect(),
    })
}

#[cfg(test)]
mod tests;
