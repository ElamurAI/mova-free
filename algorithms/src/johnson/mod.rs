//! Signed all-pairs distances: Johnson, JACM 24(1), 1977, pp. 1–13.
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
    NegativeCycle,
    ArithmeticOverflow,
    ResourceLimit,
}

/// At most 1024 vertices and 1,000,000 edges; limits are checked before endpoints.
/// All endpoints are checked in adjacency order before detecting any negative cycle.
/// Returns reflexive exact minimum walk weights, or rejects a negative cycle anywhere.
/// Empty graphs return an empty matrix. Parallel edges and loops are allowed.
/// Potentials, reweighted edges and distances stay i128; no u64 narrowing occurs.
/// Time O(V(V+E)+V E log(E+1)); space O(V²+V+E), including lazy heap entries.
/// Allocation failure is not converted into an error.
pub fn johnson(graph: &[Vec<(usize, i64)>]) -> Result<Vec<Vec<Option<i128>>>, Error> {
    let n = graph.len();
    if n > 1024
        || graph
            .iter()
            .try_fold(0usize, |a, row| a.checked_add(row.len()))
            .is_none_or(|e| e > 1_000_000)
    {
        return Err(Error::ResourceLimit);
    }
    for (from, row) in graph.iter().enumerate() {
        for &(to, _) in row {
            if to >= n {
                return Err(Error::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
        }
    }
    // Zero initialization is the implicit zero-edge super-source.
    let mut h = vec![0i128; n];
    for round in 0..n {
        let mut next = h.clone();
        for (u, row) in graph.iter().enumerate() {
            for &(v, w) in row {
                let value = h[u]
                    .checked_add(i128::from(w))
                    .ok_or(Error::ArithmeticOverflow)?;
                next[v] = next[v].min(value);
            }
        }
        if next == h {
            break;
        }
        if round + 1 == n {
            return Err(Error::NegativeCycle);
        }
        h = next;
    }
    let mut reweighted = vec![Vec::new(); n];
    for (u, row) in graph.iter().enumerate() {
        for &(v, w) in row {
            let weight = i128::from(w)
                .checked_add(h[u])
                .and_then(|x| x.checked_sub(h[v]))
                .ok_or(Error::ArithmeticOverflow)?;
            debug_assert!(weight >= 0);
            reweighted[u].push((v, weight));
        }
    }
    let mut output = Vec::with_capacity(n);
    for source in 0..n {
        let mut distance = vec![None; n];
        let mut heap = BinaryHeap::new();
        distance[source] = Some(0i128);
        heap.push(Reverse((0i128, source)));
        while let Some(Reverse((cost, u))) = heap.pop() {
            if distance[u] != Some(cost) {
                continue;
            }
            for &(v, w) in &reweighted[u] {
                let candidate = cost.checked_add(w).ok_or(Error::ArithmeticOverflow)?;
                if distance[v].is_none_or(|old| candidate < old) {
                    distance[v] = Some(candidate);
                    heap.push(Reverse((candidate, v)));
                }
            }
        }
        for (v, value) in distance.iter_mut().enumerate() {
            if let Some(cost) = value {
                *cost = cost
                    .checked_sub(h[source])
                    .and_then(|x| x.checked_add(h[v]))
                    .ok_or(Error::ArithmeticOverflow)?;
            }
        }
        output.push(distance);
    }
    Ok(output)
}
#[cfg(test)]
mod tests;
