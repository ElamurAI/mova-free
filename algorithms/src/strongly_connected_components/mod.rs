//! Canonical strongly connected components of a directed graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidEdge {
        from: usize,
        to: usize,
        vertices: usize,
    },
}

/// Iterative Kosaraju traversal with global endpoint validation.
/// Each component is sorted; components are ordered by their smallest vertex.
/// Empty input yields an empty partition. Parallel arcs and self loops are allowed.
/// Time O(V+E+V log(V+1)); auxiliary space O(V+E). No recursive graph traversal.
pub fn components(graph: &[Vec<usize>]) -> Result<Vec<Vec<usize>>, Error> {
    let n = graph.len();
    for (from, edges) in graph.iter().enumerate() {
        for &to in edges {
            if to >= n {
                return Err(Error::InvalidEdge {
                    from,
                    to,
                    vertices: n,
                });
            }
        }
    }
    let mut reverse = vec![vec![]; n];
    for (from, edges) in graph.iter().enumerate() {
        for &to in edges {
            reverse[to].push(from);
        }
    }
    let mut seen = vec![false; n];
    let mut finished = Vec::with_capacity(n);
    for start in 0..n {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        let mut stack = vec![(start, 0)];
        while let Some((vertex, next)) = stack.last_mut() {
            if *next < graph[*vertex].len() {
                let to = graph[*vertex][*next];
                *next += 1;
                if !seen[to] {
                    seen[to] = true;
                    stack.push((to, 0));
                }
            } else {
                finished.push(*vertex);
                stack.pop();
            }
        }
    }
    seen.fill(false);
    let mut result = Vec::new();
    while let Some(start) = finished.pop() {
        if seen[start] {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(vertex) = stack.pop() {
            component.push(vertex);
            for &to in &reverse[vertex] {
                if !seen[to] {
                    seen[to] = true;
                    stack.push(to);
                }
            }
        }
        component.sort_unstable();
        result.push(component);
    }
    result.sort_unstable_by_key(|component| component[0]);
    Ok(result)
}
#[cfg(test)]
mod tests;
