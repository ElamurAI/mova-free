//! Deterministic minimum spanning forests of undirected weighted multigraphs.
use crate::disjoint_set::DisjointSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Forest {
    /// Original input indices, in deterministic acceptance order.
    pub edge_indices: Vec<usize>,
    pub total_weight: u128,
    /// Number of connected components, including isolated vertices.
    pub components: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidEndpoint {
        edge: usize,
        vertex: usize,
        vertices: usize,
    },
    TotalWeightOverflow,
}

/// Validates every endpoint in input order (u before v), then ignores self loops.
/// Ties use (weight, min endpoint, max endpoint, original index).
/// Returns a minimum spanning tree for each connected component, including isolates.
/// Time O(V+E log(E+1)+E alpha(V)); auxiliary space O(V+E).
pub fn minimum_spanning_forest(vertices: usize, edges: &[Edge]) -> Result<Forest, Error> {
    for (edge, value) in edges.iter().enumerate() {
        for vertex in [value.u, value.v] {
            if vertex >= vertices {
                return Err(Error::InvalidEndpoint {
                    edge,
                    vertex,
                    vertices,
                });
            }
        }
    }
    let mut order: Vec<usize> = (0..edges.len())
        .filter(|&i| edges[i].u != edges[i].v)
        .collect();
    order.sort_unstable_by_key(|&i| {
        let edge = edges[i];
        (edge.weight, edge.u.min(edge.v), edge.u.max(edge.v), i)
    });
    let mut sets = DisjointSet::new(vertices);
    let mut forest = Forest {
        edge_indices: Vec::new(),
        total_weight: 0,
        components: vertices,
    };
    for index in order {
        let edge = edges[index];
        if sets
            .union(edge.u, edge.v)
            .expect("all endpoints were validated")
        {
            forest.total_weight = forest
                .total_weight
                .checked_add(u128::from(edge.weight))
                .ok_or(Error::TotalWeightOverflow)?;
            forest.edge_indices.push(index);
        }
    }
    forest.components = sets.components();
    Ok(forest)
}
#[cfg(test)]
mod tests;
