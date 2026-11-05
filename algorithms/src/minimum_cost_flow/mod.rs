//! Successive shortest augmenting paths; Ahuja, Magnanti, Orlin, Network Flows (1993).
pub const MAX_VERTICES: usize = 64;
pub const MAX_EDGES: usize = 512;
pub const MAX_FLOW: u64 = 1024;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub capacity: u64,
    pub cost: i64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    pub value: u64,
    pub cost: i128,
    pub edge_flows: Vec<u64>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LimitExceeded,
    InvalidVertex,
    SameTerminal,
    InvalidEdge { index: usize },
    NegativeCycle,
    Overflow,
}
#[derive(Clone, Copy)]
struct Arc {
    from: usize,
    to: usize,
    capacity: u64,
    cost: i128,
}
fn distances(n: usize, arcs: &[Arc], source: Option<usize>) -> Result<Vec<Option<i128>>, Error> {
    let mut d: Vec<Option<i128>> = vec![if source.is_none() { Some(0) } else { None }; n];
    if let Some(s) = source {
        d[s] = Some(0);
    }
    for round in 0..n {
        let mut next = d.clone();
        let mut changed = false;
        for e in arcs {
            if e.capacity == 0 {
                continue;
            }
            if let Some(x) = d[e.from] {
                let value = x.checked_add(e.cost).ok_or(Error::Overflow)?;
                if next[e.to].is_none_or(|old| value < old) {
                    next[e.to] = Some(value);
                    changed = true;
                }
            }
        }
        d = next;
        if !changed {
            return Ok(d);
        }
        if round + 1 == n {
            return Err(Error::NegativeCycle);
        }
    }
    Ok(d)
}
/// Maximize flow up to limit, then minimize cost. Negative cycles anywhere in the
/// positive-capacity input are unsupported, even if disconnected or limit is zero.
/// Validation: caps, terminals, distinctness, input-order endpoints, cycles.
/// Parallel edges and loops supported. Strict relaxation and input-order FIFO
/// tight paths give deterministic witnesses, not a canonical optimal flow.
/// O((limit+1)V(E+V)) time, O(V+E) memory. Allocation failure is not typed.
pub fn minimum_cost_flow(
    vertices: usize,
    edges: &[Edge],
    source: usize,
    sink: usize,
    limit: u64,
) -> Result<Flow, Error> {
    if vertices > MAX_VERTICES || edges.len() > MAX_EDGES || limit > MAX_FLOW {
        return Err(Error::LimitExceeded);
    }
    if source >= vertices || sink >= vertices {
        return Err(Error::InvalidVertex);
    }
    if source == sink {
        return Err(Error::SameTerminal);
    }
    let mut arcs = Vec::with_capacity(edges.len() * 2);
    let mut adj = vec![Vec::new(); vertices];
    for (index, e) in edges.iter().enumerate() {
        if e.from >= vertices || e.to >= vertices {
            return Err(Error::InvalidEdge { index });
        }
        let i = arcs.len();
        arcs.push(Arc {
            from: e.from,
            to: e.to,
            capacity: e.capacity,
            cost: i128::from(e.cost),
        });
        arcs.push(Arc {
            from: e.to,
            to: e.from,
            capacity: 0,
            cost: -i128::from(e.cost),
        });
        adj[e.from].push(i);
        adj[e.to].push(i + 1);
    }
    distances(vertices, &arcs, None)?;
    let mut value = 0u64;
    let mut cost = 0i128;
    while value < limit {
        let d = distances(vertices, &arcs, Some(source))?;
        let Some(distance) = d[sink] else {
            break;
        };
        let mut prev = vec![None; vertices];
        let mut seen = vec![false; vertices];
        seen[source] = true;
        let mut queue = std::collections::VecDeque::from([source]);
        while let Some(u) = queue.pop_front() {
            for &i in &adj[u] {
                let e = arcs[i];
                if e.capacity > 0
                    && !seen[e.to]
                    && d[u].and_then(|x| x.checked_add(e.cost)) == d[e.to]
                {
                    seen[e.to] = true;
                    prev[e.to] = Some(i);
                    queue.push_back(e.to);
                }
            }
        }
        let mut path = Vec::new();
        let mut v = sink;
        while v != source {
            let i = prev[v].expect("shortest distance has a tight path");
            path.push(i);
            v = arcs[i].from;
        }
        let amount = path
            .iter()
            .fold(limit - value, |x, &i| x.min(arcs[i].capacity));
        value = value.checked_add(amount).ok_or(Error::Overflow)?;
        cost = cost
            .checked_add(
                distance
                    .checked_mul(i128::from(amount))
                    .ok_or(Error::Overflow)?,
            )
            .ok_or(Error::Overflow)?;
        for i in path {
            arcs[i].capacity = arcs[i]
                .capacity
                .checked_sub(amount)
                .ok_or(Error::Overflow)?;
            arcs[i ^ 1].capacity = arcs[i ^ 1]
                .capacity
                .checked_add(amount)
                .ok_or(Error::Overflow)?;
        }
    }
    Ok(Flow {
        value,
        cost,
        edge_flows: (0..edges.len()).map(|i| arcs[2 * i + 1].capacity).collect(),
    })
}
#[cfg(test)]
mod tests;
