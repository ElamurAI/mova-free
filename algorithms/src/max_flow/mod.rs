//! Indexed directed maximum flow using Edmonds–Karp shortest augmenting paths.
//! Original implementation of the classical algorithm (Edmonds and Karp, 1972).
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub capacity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    pub value: u128,
    /// Flow per original edge, including parallel, antiparallel and loop edges.
    pub edge_flows: Vec<u64>,
    /// Vertices reachable from the source in the final residual graph.
    pub source_side: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidVertex { vertex: usize, vertices: usize },
    SameTerminal,
    InvalidEdge { index: usize },
    Overflow,
}

#[derive(Clone, Copy)]
struct Arc {
    from: usize,
    to: usize,
    residual: u64,
}

/// Validates source, sink, distinctness, then edges in input order.
/// FIFO BFS and original edge order choose one deterministic optimum. Loops
/// retain zero flow. Allocation failure is outside the error contract.
/// Space O(V+E); conservative time O(V+E+VE(V+E)), accounting for clearing V
/// predecessors per search even when most vertices are isolated.
pub fn max_flow(
    vertices: usize,
    edges: &[Edge],
    source: usize,
    sink: usize,
) -> Result<Flow, Error> {
    for vertex in [source, sink] {
        if vertex >= vertices {
            return Err(Error::InvalidVertex { vertex, vertices });
        }
    }
    if source == sink {
        return Err(Error::SameTerminal);
    }
    for (index, edge) in edges.iter().enumerate() {
        if edge.from >= vertices || edge.to >= vertices {
            return Err(Error::InvalidEdge { index });
        }
    }
    let mut graph = vec![Vec::new(); vertices];
    let mut arcs = Vec::new();
    for edge in edges {
        let index = arcs.len();
        arcs.push(Arc {
            from: edge.from,
            to: edge.to,
            residual: edge.capacity,
        });
        arcs.push(Arc {
            from: edge.to,
            to: edge.from,
            residual: 0,
        });
        graph[edge.from].push(index);
        graph[edge.to].push(index + 1);
    }
    let mut value = 0u128;
    loop {
        let mut parent = vec![None; vertices];
        let mut seen = vec![false; vertices];
        seen[source] = true;
        let mut queue = VecDeque::from([source]);
        while let Some(vertex) = queue.pop_front() {
            for &index in &graph[vertex] {
                let arc = arcs[index];
                if arc.residual > 0 && !seen[arc.to] {
                    seen[arc.to] = true;
                    parent[arc.to] = Some(index);
                    queue.push_back(arc.to);
                }
            }
        }
        if !seen[sink] {
            return Ok(Flow {
                value,
                edge_flows: arcs
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|pair| pair[1].residual)
                    .collect(),
                source_side: seen,
            });
        }
        let mut amount = u64::MAX;
        let mut vertex = sink;
        while vertex != source {
            let index = parent[vertex].expect("reachable non-source has predecessor");
            amount = amount.min(arcs[index].residual);
            vertex = arcs[index].from;
        }
        value = value
            .checked_add(u128::from(amount))
            .ok_or(Error::Overflow)?;
        vertex = sink;
        while vertex != source {
            let index = parent[vertex].expect("reachable non-source has predecessor");
            arcs[index].residual = arcs[index]
                .residual
                .checked_sub(amount)
                .ok_or(Error::Overflow)?;
            arcs[index ^ 1].residual = arcs[index ^ 1]
                .residual
                .checked_add(amount)
                .ok_or(Error::Overflow)?;
            vertex = arcs[index].from;
        }
    }
}

#[cfg(test)]
mod tests;
