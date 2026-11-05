//! Boruvka's cut-based minimum spanning forest (Boruvka, 1926).
//! Original implementation; signed multigraph weights and input-index tie breaks.
use crate::disjoint_set::DisjointSet;
pub const MAX_VERTICES: usize = 1_000_000;
pub const MAX_EDGES: usize = 4_000_000;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: i64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Forest {
    pub edge_indices: Vec<usize>,
    pub total_weight: i128,
    pub components: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    TooManyVertices,
    TooManyEdges,
    InvalidEndpoint { edge: usize, vertex: usize },
}
/// Caps are checked first, then endpoints in input order, u before v.
/// Loops are validated but never selected; parallel edges are supported.
/// Each round chooses the least (weight,input index) outgoing edge per component.
/// Components are visited by ascending DSU root; accepted indices are finally sorted.
/// The i128 sum is exact under MAX_VERTICES and i64 weights.
/// O((V+E) log(V+1) alpha(V)) time, O(V) auxiliary space plus output.
/// Empty and disconnected graphs return forests, including isolated vertices.
pub fn minimum_spanning_forest(vertices: usize, edges: &[Edge]) -> Result<Forest, Error> {
    if vertices > MAX_VERTICES {
        return Err(Error::TooManyVertices);
    }
    if edges.len() > MAX_EDGES {
        return Err(Error::TooManyEdges);
    }
    for (i, e) in edges.iter().enumerate() {
        for v in [e.u, e.v] {
            if v >= vertices {
                return Err(Error::InvalidEndpoint { edge: i, vertex: v });
            }
        }
    }
    let mut sets = DisjointSet::new(vertices);
    let mut result = Forest {
        edge_indices: Vec::new(),
        total_weight: 0,
        components: vertices,
    };
    loop {
        let mut cheapest: Vec<Option<usize>> = vec![None; vertices];
        for (i, e) in edges.iter().enumerate() {
            let a = sets.find(e.u).expect("validated");
            let b = sets.find(e.v).expect("validated");
            if a == b {
                continue;
            }
            for root in [a, b] {
                if cheapest[root].is_none_or(|j| (e.weight, i) < (edges[j].weight, j)) {
                    cheapest[root] = Some(i);
                }
            }
        }
        let before = sets.components();
        for i in cheapest.into_iter().flatten() {
            let e = edges[i];
            if sets.union(e.u, e.v).expect("validated") {
                result.edge_indices.push(i);
                result.total_weight += i128::from(e.weight);
            }
        }
        if sets.components() == before {
            break;
        }
    }
    result.edge_indices.sort_unstable();
    result.components = sets.components();
    Ok(result)
}
#[cfg(test)]
mod tests;
