//! Original iterative implementation of Dinic's blocking-flow algorithm.
//! Attribution: E. A. Dinic, "Algorithm for solution of a problem of maximum
//! flow in networks with power estimation", Soviet Mathematics Doklady 11 (1970).
use crate::max_flow::{Edge, Flow};
use std::collections::VecDeque;

pub const MAX_VERTICES: usize = 1_000_000;
pub const MAX_EDGES: usize = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidVertex { vertex: usize, vertices: usize },
    SameTerminal,
    InvalidEdge { index: usize },
    Overflow,
}

#[derive(Clone, Copy)]
struct Arc {
    to: usize,
    residual: u64,
}

/// Limits precede source, sink, distinctness, then input-order edge validation.
/// Parallel/antiparallel edges remain distinct; loops have zero flow. FIFO levels
/// and adjacency order select a deterministic optimum, not a canonical flow.
/// Uses O(V+E) storage and O(V^2 E + V^2 + E) conservative time. Paths use a
/// heap stack, not recursion. u128 totals and u64 residual pairs are checked;
/// Overflow is defensive under the caps. Allocation failure is not typed.
pub fn max_flow(
    vertices: usize,
    edges: &[Edge],
    source: usize,
    sink: usize,
) -> Result<Flow, Error> {
    if vertices > MAX_VERTICES || edges.len() > MAX_EDGES {
        return Err(Error::LimitExceeded);
    }
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
    let mut arcs = Vec::with_capacity(edges.len() * 2);
    for edge in edges {
        let index = arcs.len();
        arcs.push(Arc {
            to: edge.to,
            residual: edge.capacity,
        });
        arcs.push(Arc {
            to: edge.from,
            residual: 0,
        });
        graph[edge.from].push(index);
        graph[edge.to].push(index + 1);
    }
    let mut value = 0u128;
    let mut level = vec![usize::MAX; vertices];
    let mut cursor = vec![0; vertices];
    let mut path: Vec<usize> = Vec::new();
    loop {
        level.fill(usize::MAX);
        level[source] = 0;
        let mut queue = VecDeque::from([source]);
        while let Some(from) = queue.pop_front() {
            for &index in &graph[from] {
                let arc = arcs[index];
                if arc.residual != 0 && level[arc.to] == usize::MAX {
                    level[arc.to] = level[from] + 1;
                    queue.push_back(arc.to);
                }
            }
        }
        if level[sink] == usize::MAX {
            return Ok(Flow {
                value,
                edge_flows: arcs
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|pair| pair[1].residual)
                    .collect(),
                source_side: level.iter().map(|&d| d != usize::MAX).collect(),
            });
        }
        cursor.fill(0);
        loop {
            path.clear();
            let mut vertex = source;
            loop {
                if vertex == sink {
                    break;
                }
                while cursor[vertex] < graph[vertex].len() {
                    let index = graph[vertex][cursor[vertex]];
                    if arcs[index].residual > 0 && level[arcs[index].to] == level[vertex] + 1 {
                        break;
                    }
                    cursor[vertex] += 1;
                }
                if cursor[vertex] == graph[vertex].len() {
                    if let Some(index) = path.pop() {
                        vertex = arcs[index ^ 1].to;
                        cursor[vertex] += 1;
                    } else {
                        break;
                    }
                } else {
                    let index = graph[vertex][cursor[vertex]];
                    path.push(index);
                    vertex = arcs[index].to;
                }
            }
            if vertex != sink {
                break;
            }
            let amount = path
                .iter()
                .map(|&i| arcs[i].residual)
                .min()
                .expect("distinct terminals");
            value = value
                .checked_add(u128::from(amount))
                .ok_or(Error::Overflow)?;
            for &index in &path {
                arcs[index].residual = arcs[index]
                    .residual
                    .checked_sub(amount)
                    .ok_or(Error::Overflow)?;
                arcs[index ^ 1].residual = arcs[index ^ 1]
                    .residual
                    .checked_add(amount)
                    .ok_or(Error::Overflow)?;
            }
        }
    }
}

#[cfg(test)]
mod tests;
