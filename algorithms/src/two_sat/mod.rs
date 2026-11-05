//! Deterministic 2-SAT using an implication graph and existing SCC primitives.
use crate::{strongly_connected_components, topological_sort};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    pub variable: usize,
    pub positive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    TooManyVariables,
    InvalidVariable {
        clause: usize,
        variable: usize,
        variables: usize,
    },
}

/// Return a satisfying assignment, or None for an unsatisfiable formula.
/// Each pair denotes a disjunction; duplicate literals encode unit clauses.
/// Check doubled size first, then literals in clause order, left before right.
/// Canonical SCC numbers are not topological ranks: the condensation graph is
/// sorted explicitly before choosing the later component of each literal pair.
/// Fixed input has a deterministic assignment, not necessarily lexicographic.
/// Time O(V+C+V log(V+1)); space O(V+C), for V variables and C clauses.
/// Allocation failure is outside this API's errors; no recursive traversal.
pub fn solve(variables: usize, clauses: &[(Literal, Literal)]) -> Result<Option<Vec<bool>>, Error> {
    let count = variables.checked_mul(2).ok_or(Error::TooManyVariables)?;
    for (clause, &(left, right)) in clauses.iter().enumerate() {
        for literal in [left, right] {
            if literal.variable >= variables {
                return Err(Error::InvalidVariable {
                    clause,
                    variable: literal.variable,
                    variables,
                });
            }
        }
    }
    let index = |literal: Literal| 2 * literal.variable + usize::from(literal.positive);
    let mut graph = vec![Vec::new(); count];
    for &(left, right) in clauses {
        let (a, b) = (index(left), index(right));
        graph[a ^ 1].push(b);
        graph[b ^ 1].push(a);
    }
    let components = strongly_connected_components::components(&graph)
        .expect("validated literals construct in-range implication edges");
    let mut owner = vec![0; count];
    for (id, component) in components.iter().enumerate() {
        for &vertex in component {
            owner[vertex] = id;
        }
    }
    for variable in 0..variables {
        if owner[2 * variable] == owner[2 * variable + 1] {
            return Ok(None);
        }
    }
    let mut dag = vec![Vec::new(); components.len()];
    for (from, edges) in graph.iter().enumerate() {
        for &to in edges {
            if owner[from] != owner[to] {
                dag[owner[from]].push(owner[to]);
            }
        }
    }
    let order = topological_sort::topological_sort(&dag)
        .expect("SCC condensation is an in-range acyclic graph");
    let mut rank = vec![0; components.len()];
    for (position, component) in order.into_iter().enumerate() {
        rank[component] = position;
    }
    Ok(Some(
        (0..variables)
            .map(|v| rank[owner[2 * v + 1]] > rank[owner[2 * v]])
            .collect(),
    ))
}

#[cfg(test)]
mod tests;
