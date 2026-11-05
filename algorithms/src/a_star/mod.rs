//! A* with globally validated consistent, nonnegative heuristics.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidVertex { vertex: usize, vertices: usize },
    HeuristicLength { expected: usize, actual: usize },
    NonzeroGoalHeuristic,
    InvalidEdge { from: usize, to: usize },
    InconsistentHeuristic { from: usize, to: usize },
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub distance: u128,
    pub vertices: Vec<usize>,
}

/// Find a shortest directed path, or None when the goal is unreachable.
/// Validate start, goal, heuristic length, goal value, all endpoints, then all
/// consistency inequalities, in that order (row order within each scan).
/// Consistency and h(goal)=0 justify terminating when the goal is popped.
/// Loops, zero weights and parallel arcs are allowed. Ties use (f,g,vertex)
/// order and keep the first strict improvement's predecessor. The witness is
/// deterministic for fixed input, but not promised lexicographically smallest.
/// Costs and priorities use checked u128 sums. Allocation failure is outside
/// this API's errors. Time O(V+E+(V+E) log(V+E+1)); space O(V+E).
pub fn shortest_path(
    graph: &[Vec<(usize, u64)>],
    start: usize,
    goal: usize,
    heuristic: &[u64],
) -> Result<Option<Path>, Error> {
    let n = graph.len();
    for vertex in [start, goal] {
        if vertex >= n {
            return Err(Error::InvalidVertex {
                vertex,
                vertices: n,
            });
        }
    }
    if heuristic.len() != n {
        return Err(Error::HeuristicLength {
            expected: n,
            actual: heuristic.len(),
        });
    }
    if heuristic[goal] != 0 {
        return Err(Error::NonzeroGoalHeuristic);
    }
    for (from, edges) in graph.iter().enumerate() {
        for &(to, _) in edges {
            if to >= n {
                return Err(Error::InvalidEdge { from, to });
            }
        }
    }
    for (from, edges) in graph.iter().enumerate() {
        for &(to, weight) in edges {
            if u128::from(heuristic[from]) > u128::from(weight) + u128::from(heuristic[to]) {
                return Err(Error::InconsistentHeuristic { from, to });
            }
        }
    }
    let mut distance = vec![None; n];
    let mut predecessor = vec![None; n];
    let mut queue = BinaryHeap::new();
    distance[start] = Some(0u128);
    queue.push(Reverse((u128::from(heuristic[start]), 0u128, start)));
    while let Some(Reverse((_, cost, from))) = queue.pop() {
        if distance[from] != Some(cost) {
            continue;
        }
        if from == goal {
            let mut vertices = vec![goal];
            let mut current = goal;
            while current != start {
                current = predecessor[current].expect("relaxed non-start vertex has predecessor");
                vertices.push(current);
            }
            vertices.reverse();
            return Ok(Some(Path {
                distance: cost,
                vertices,
            }));
        }
        for &(to, weight) in &graph[from] {
            let candidate = cost
                .checked_add(u128::from(weight))
                .ok_or(Error::ArithmeticOverflow)?;
            if distance[to].is_none_or(|old| candidate < old) {
                let priority = candidate
                    .checked_add(u128::from(heuristic[to]))
                    .ok_or(Error::ArithmeticOverflow)?;
                distance[to] = Some(candidate);
                predecessor[to] = Some(from);
                queue.push(Reverse((priority, candidate, to)));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests;
