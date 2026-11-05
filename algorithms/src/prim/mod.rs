//! Minimum spanning forests by Prim's greedy cut rule (BSTJ 36(6), 1957).
pub use crate::kruskal::{Edge, Forest};
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ResourceLimit,
    InvalidEndpoint {
        edge: usize,
        vertex: usize,
        vertices: usize,
    },
    TotalWeightOverflow,
}
/// Reuses Kruskal's edge and forest records. At most 1,000,000 vertices/edges;
/// limits precede endpoint validation (input order, u before v), then loops are ignored.
/// Components start at their smallest vertex. Heap ties use
/// (weight,min endpoint,max endpoint,input index); returned indices are acceptance order.
/// O(V+E log(E+1)) time and O(V+E) storage. Allocation failure is not a typed error.
pub fn minimum_spanning_forest(vertices: usize, edges: &[Edge]) -> Result<Forest, Error> {
    if vertices > 1_000_000 || edges.len() > 1_000_000 {
        return Err(Error::ResourceLimit);
    }
    let mut adjacent = vec![Vec::new(); vertices];
    for (index, edge) in edges.iter().enumerate() {
        for vertex in [edge.u, edge.v] {
            if vertex >= vertices {
                return Err(Error::InvalidEndpoint {
                    edge: index,
                    vertex,
                    vertices,
                });
            }
        }
        if edge.u != edge.v {
            adjacent[edge.u].push(index);
            adjacent[edge.v].push(index);
        }
    }
    let mut visited = vec![false; vertices];
    let mut result = Forest {
        edge_indices: Vec::new(),
        total_weight: 0,
        components: 0,
    };
    let mut heap = BinaryHeap::new();
    for root in 0..vertices {
        if visited[root] {
            continue;
        }
        result.components += 1;
        visited[root] = true;
        let mut vertex = root;
        loop {
            for &index in &adjacent[vertex] {
                let e = edges[index];
                heap.push(Reverse((e.weight, e.u.min(e.v), e.u.max(e.v), index)));
            }
            let next = loop {
                let Some(Reverse((_, _, _, index))) = heap.pop() else {
                    break None;
                };
                let e = edges[index];
                if visited[e.u] && visited[e.v] {
                    continue;
                }
                break Some((index, if visited[e.u] { e.v } else { e.u }));
            };
            let Some((index, to)) = next else {
                break;
            };
            result.total_weight = result
                .total_weight
                .checked_add(u128::from(edges[index].weight))
                .ok_or(Error::TotalWeightOverflow)?;
            result.edge_indices.push(index);
            visited[to] = true;
            vertex = to;
        }
    }
    Ok(result)
}
#[cfg(test)]
mod tests;
