//! Canonical Prufer tree coding (Pruefer, 1918), using a minimum-leaf heap.
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
pub const MAX_VERTICES: usize = 1_000_000;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    TooManyVertices,
    WrongLength,
    InvalidLabel { position: usize, label: usize },
    SelfLoop { edge: usize },
    RepeatedEdge { edge: usize },
    NotTree,
}
fn size(n: usize) -> Result<(), Error> {
    if n > MAX_VERTICES {
        Err(Error::TooManyVertices)
    } else {
        Ok(())
    }
}
/// Labels are 0..vertices. n=0 and n=1 each encode the empty edge list as [];
/// n=2 encodes its sole tree as []. Caps precede length and label validation.
/// Edges are undirected; reversed duplicates and loops are errors.
/// O(n log(n+1)) time and O(n) auxiliary space.
pub fn encode(vertices: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Error> {
    size(vertices)?;
    if edges.len() != vertices.saturating_sub(1) {
        return Err(Error::WrongLength);
    }
    let mut seen = BTreeSet::new();
    let mut adjacency = vec![Vec::new(); vertices];
    let mut sets = crate::disjoint_set::DisjointSet::new(vertices);
    for (i, &(u, v)) in edges.iter().enumerate() {
        for label in [u, v] {
            if label >= vertices {
                return Err(Error::InvalidLabel { position: i, label });
            }
        }
        if u == v {
            return Err(Error::SelfLoop { edge: i });
        }
        if !seen.insert((u.min(v), u.max(v))) {
            return Err(Error::RepeatedEdge { edge: i });
        }
        if !sets.union(u, v).expect("validated") {
            return Err(Error::NotTree);
        }
        adjacency[u].push(v);
        adjacency[v].push(u);
    }
    let mut degree: Vec<_> = adjacency.iter().map(Vec::len).collect();
    let mut leaves: BinaryHeap<_> = degree
        .iter()
        .enumerate()
        .filter(|&(_, d)| *d == 1)
        .map(|(v, _)| Reverse(v))
        .collect();
    let mut code = Vec::with_capacity(vertices.saturating_sub(2));
    for _ in 0..vertices.saturating_sub(2) {
        let Reverse(leaf) = leaves.pop().expect("validated tree");
        let next = *adjacency[leaf]
            .iter()
            .find(|&&v| degree[v] > 0)
            .expect("live neighbor");
        code.push(next);
        degree[leaf] = 0;
        degree[next] -= 1;
        if degree[next] == 1 {
            leaves.push(Reverse(next));
        }
    }
    Ok(code)
}
/// The vertex count disambiguates empty codes for n=0,1,2.
/// Output edges have ascending endpoints and are sorted lexicographically.
/// Every length n-2 word over 0..n decodes to a tree for n>=2.
/// O(n log(n+1)) time and O(n) auxiliary space.
pub fn decode(vertices: usize, code: &[usize]) -> Result<Vec<(usize, usize)>, Error> {
    size(vertices)?;
    if code.len() != vertices.saturating_sub(2) {
        return Err(Error::WrongLength);
    }
    let mut degree = vec![1usize; vertices];
    for (position, &label) in code.iter().enumerate() {
        if label >= vertices {
            return Err(Error::InvalidLabel { position, label });
        }
        degree[label] += 1;
    }
    if vertices < 2 {
        return Ok(Vec::new());
    }
    let mut leaves: BinaryHeap<_> = degree
        .iter()
        .enumerate()
        .filter(|&(_, d)| *d == 1)
        .map(|(v, _)| Reverse(v))
        .collect();
    let mut edges = Vec::with_capacity(vertices - 1);
    for &v in code {
        let Reverse(u) = leaves.pop().expect("degree law");
        edges.push((u.min(v), u.max(v)));
        degree[u] = 0;
        degree[v] -= 1;
        if degree[v] == 1 {
            leaves.push(Reverse(v));
        }
    }
    let Reverse(u) = leaves.pop().expect("two final leaves");
    let Reverse(v) = leaves.pop().expect("two final leaves");
    edges.push((u.min(v), u.max(v)));
    edges.sort_unstable();
    Ok(edges)
}
#[cfg(test)]
mod tests;
