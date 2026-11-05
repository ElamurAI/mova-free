//! Original iterative Hopcroft-Karp bipartite maximum-cardinality matching.
//! Attribution: J. E. Hopcroft and R. M. Karp, "An n^(5/2) Algorithm for Maximum
//! Matchings in Bipartite Graphs", SIAM Journal on Computing 2(4), 1973.
use std::collections::VecDeque;

pub const MAX_VERTICES: usize = 1_000_000;
pub const MAX_EDGES: usize = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidEdge {
        left: usize,
        right: usize,
        right_vertices: usize,
    },
}

/// Rows index left vertices. Return one optional right mate per left vertex.
/// Validate combined vertex cap, total edge cap, then endpoints in row order.
/// Duplicate neighbors are allowed. Deterministic left/adjacency order chooses
/// one optimum; no lexicographic minimality or permutation-invariant tie claim.
/// O((L+R+E) sqrt(L+R+1)) time and O(L+R) auxiliary storage, including explicit
/// alternating-path stacks. Allocation failure is outside the typed contract.
pub fn maximum_matching(
    graph: &[Vec<usize>],
    right_vertices: usize,
) -> Result<Vec<Option<usize>>, Error> {
    if graph.len() > MAX_VERTICES || right_vertices > MAX_VERTICES - graph.len() {
        return Err(Error::LimitExceeded);
    }
    let mut count = 0usize;
    for row in graph {
        count = count.checked_add(row.len()).ok_or(Error::LimitExceeded)?;
        if count > MAX_EDGES {
            return Err(Error::LimitExceeded);
        }
    }
    for (left, row) in graph.iter().enumerate() {
        for &right in row {
            if right >= right_vertices {
                return Err(Error::InvalidEdge {
                    left,
                    right,
                    right_vertices,
                });
            }
        }
    }
    let mut left_mate = vec![None; graph.len()];
    let mut right_mate: Vec<Option<usize>> = vec![None; right_vertices];
    let mut distance = vec![usize::MAX; graph.len()];
    let mut cursor = vec![0; graph.len()];
    let mut stack = Vec::new();
    let mut path = Vec::new();
    loop {
        distance.fill(usize::MAX);
        let mut queue = VecDeque::new();
        for left in 0..graph.len() {
            if left_mate[left].is_none() {
                distance[left] = 0;
                queue.push_back(left);
            }
        }
        let mut shortest = usize::MAX;
        while let Some(left) = queue.pop_front() {
            if distance[left] >= shortest {
                continue;
            }
            for &right in &graph[left] {
                if let Some(next) = right_mate[right] {
                    if distance[next] == usize::MAX {
                        distance[next] = distance[left] + 1;
                        queue.push_back(next);
                    }
                } else {
                    shortest = distance[left] + 1;
                }
            }
        }
        if shortest == usize::MAX {
            return Ok(left_mate);
        }
        cursor.fill(0);
        for root in 0..graph.len() {
            if left_mate[root].is_some() || distance[root] == usize::MAX {
                continue;
            }
            stack.clear();
            path.clear();
            stack.push(root);
            while let Some(&left) = stack.last() {
                if cursor[left] == graph[left].len() {
                    distance[left] = usize::MAX;
                    stack.pop();
                    path.pop();
                    continue;
                }
                let right = graph[left][cursor[left]];
                cursor[left] += 1;
                if let Some(next) = right_mate[right] {
                    if distance[next] == distance[left] + 1 && distance[next] < shortest {
                        stack.push(next);
                        path.push(right);
                    }
                } else if distance[left] + 1 == shortest {
                    path.push(right);
                    for (&u, &v) in stack.iter().zip(&path) {
                        left_mate[u] = Some(v);
                        right_mate[v] = Some(u);
                    }
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
