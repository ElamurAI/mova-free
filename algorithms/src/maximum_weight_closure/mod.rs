//! Original reduction; Picard, Maximal closure of a graph and applications
//! to combinatorial problems, Management Science 22 (1976).
use crate::{dinic, max_flow::Edge};
pub const MAX_VERTICES: usize = 512;
pub const MAX_EDGES: usize = 131_072;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidImplication { index: usize },
    Overflow,
    Flow(dinic::Error),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closure {
    pub weight: i128,
    pub selected: Vec<usize>,
}
/// (u,v) requires v when u is selected. Limits then endpoints then arithmetic.
/// Positive sum plus one must fit u64; i64::MIN negative weights supported.
/// Sorted residual source side; deterministic but no canonical tie promise.
/// O(V²E) time/O(V+E) space; allocation may abort. Empty set always feasible.
pub fn maximum_weight_closure(
    weights: &[i64],
    implications: &[(usize, usize)],
) -> Result<Closure, Error> {
    let n = weights.len();
    if n > MAX_VERTICES || implications.len() > MAX_EDGES {
        return Err(Error::LimitExceeded);
    }
    for (index, &(u, v)) in implications.iter().enumerate() {
        if u >= n || v >= n {
            return Err(Error::InvalidImplication { index });
        }
    }
    let mut positive = 0u64;
    for &w in weights {
        if w > 0 {
            positive = positive.checked_add(w as u64).ok_or(Error::Overflow)?;
        }
    }
    let infinity = positive.checked_add(1).ok_or(Error::Overflow)?;
    let mut edges = Vec::new();
    for (v, &w) in weights.iter().enumerate() {
        if w > 0 {
            edges.push(Edge {
                from: n,
                to: v,
                capacity: w as u64,
            });
        }
        if w < 0 {
            edges.push(Edge {
                from: v,
                to: n + 1,
                capacity: w.unsigned_abs(),
            });
        }
    }
    for &(u, v) in implications {
        edges.push(Edge {
            from: u,
            to: v,
            capacity: infinity,
        });
    }
    let flow = dinic::max_flow(n + 2, &edges, n, n + 1).map_err(Error::Flow)?;
    let weight = i128::from(positive) - i128::try_from(flow.value).map_err(|_| Error::Overflow)?;
    Ok(Closure {
        weight,
        selected: (0..n).filter(|&v| flow.source_side[v]).collect(),
    })
}
#[cfg(test)]
mod tests;
