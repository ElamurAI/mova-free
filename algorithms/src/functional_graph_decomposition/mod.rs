//! Original indegree peeling; Kahn, Topological sorting of large networks (1962).
use std::collections::VecDeque;
pub const MAX_VERTICES: usize = 1_000_000;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidSuccessor { vertex: usize },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decomposition {
    pub cycles: Vec<Vec<usize>>,
    pub cycle_id: Vec<usize>,
    pub entry: Vec<usize>,
    pub distance: Vec<usize>,
}
/// Original zero-based coordinates; cycles start at their minimum and follow
/// successors. IDs increase by minimum. Entry is first cycle vertex reached.
/// Limits precede first bad successor. O(n) time/space; allocation may abort.
pub fn decompose(successor: &[usize]) -> Result<Decomposition, Error> {
    let n = successor.len();
    if n > MAX_VERTICES {
        return Err(Error::LimitExceeded);
    }
    for (vertex, &s) in successor.iter().enumerate() {
        if s >= n {
            return Err(Error::InvalidSuccessor { vertex });
        }
    }
    let mut degree = vec![0usize; n];
    for &s in successor {
        degree[s] += 1;
    }
    let mut queue: VecDeque<_> = (0..n).filter(|&v| degree[v] == 0).collect();
    let mut peeled = Vec::new();
    while let Some(v) = queue.pop_front() {
        peeled.push(v);
        degree[successor[v]] -= 1;
        if degree[successor[v]] == 0 {
            queue.push_back(successor[v]);
        }
    }
    let mut out = Decomposition {
        cycles: Vec::new(),
        cycle_id: vec![usize::MAX; n],
        entry: vec![0; n],
        distance: vec![0; n],
    };
    for (v, &indegree) in degree.iter().enumerate() {
        if indegree == 0 || out.cycle_id[v] != usize::MAX {
            continue;
        }
        let id = out.cycles.len();
        let mut cycle = Vec::new();
        let mut u = v;
        loop {
            out.cycle_id[u] = id;
            out.entry[u] = u;
            cycle.push(u);
            u = successor[u];
            if u == v {
                break;
            }
        }
        out.cycles.push(cycle);
    }
    for &v in peeled.iter().rev() {
        let s = successor[v];
        out.cycle_id[v] = out.cycle_id[s];
        out.entry[v] = out.entry[s];
        out.distance[v] = out.distance[s] + 1;
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
