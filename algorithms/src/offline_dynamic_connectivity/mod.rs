//! Offline undirected simple-edge connectivity via time segment tree and
//! rollback union-by-size DSU (no path compression). See CP-Algorithms,
//! "Deleting from a data structure in O(T(n) log n)". Original implementation.
//! For n vertices and q operations: O(q log q log(n+1) + n) time,
//! O(q log q + n) space. BTreeMap lifecycle tracking adds O(q log q).
//! Caps bound integer/index arithmetic; allocation failure follows std.
use std::collections::BTreeMap;
pub const MAX_VERTICES: usize = 1 << 18;
pub const MAX_OPERATIONS: usize = 1 << 18;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add(usize, usize),
    Remove(usize, usize),
    Connected(usize, usize),
    Components,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Connected(bool),
    Components(usize),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TooManyVertices,
    TooManyOperations,
    InvalidVertex { time: usize, vertex: usize },
    DuplicateEdge { time: usize },
    AbsentEdge { time: usize },
}
type Edge = (usize, usize);
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    history: Vec<(usize, usize, usize)>,
    components: usize,
}
impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            history: Vec::new(),
            components: n,
        }
    }
    fn root(&self, mut v: usize) -> usize {
        while v != self.parent[v] {
            v = self.parent[v];
        }
        v
    }
    fn union(&mut self, (a, b): Edge) {
        let (mut a, mut b) = (self.root(a), self.root(b));
        if a == b {
            return;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.history.push((b, a, self.size[a]));
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.components -= 1;
    }
    fn rollback(&mut self, mark: usize) {
        while self.history.len() > mark {
            let (b, a, size) = self.history.pop().unwrap();
            self.parent[b] = b;
            self.size[a] = size;
            self.components += 1;
        }
    }
}
fn interval(tree: &mut [Vec<Edge>], base: usize, mut lo: usize, mut hi: usize, edge: Edge) {
    lo += base;
    hi += base;
    while lo < hi {
        if lo % 2 == 1 {
            tree[lo].push(edge);
            lo += 1;
        }
        if hi % 2 == 1 {
            hi -= 1;
            tree[hi].push(edge);
        }
        lo /= 2;
        hi /= 2;
    }
}
fn visit(
    node: usize,
    tree: &[Vec<Edge>],
    base: usize,
    ops: &[Operation],
    dsu: &mut Dsu,
    out: &mut Vec<Answer>,
) {
    let mark = dsu.history.len();
    for &edge in &tree[node] {
        dsu.union(edge);
    }
    if node >= base {
        if let Some(op) = ops.get(node - base) {
            match *op {
                Operation::Connected(a, b) => {
                    out.push(Answer::Connected(dsu.root(a) == dsu.root(b)))
                }
                Operation::Components => out.push(Answer::Components(dsu.components)),
                _ => {}
            }
        }
    } else {
        visit(node * 2, tree, base, ops, dsu, out);
        visit(node * 2 + 1, tree, base, ops, dsu, out);
    }
    dsu.rollback(mark);
}
/// Answers only queries, in timeline order. Add activates [add,remove).
/// Endpoints are canonicalized; self-loops obey the same lifecycle and never
/// alter connectivity. Validation scans chronologically before solving.
pub fn solve(vertices: usize, operations: &[Operation]) -> Result<Vec<Answer>, Error> {
    if vertices > MAX_VERTICES {
        return Err(Error::TooManyVertices);
    }
    if operations.len() > MAX_OPERATIONS {
        return Err(Error::TooManyOperations);
    }
    if operations.is_empty() {
        return Ok(Vec::new());
    }
    let base = operations.len().next_power_of_two();
    let mut tree = vec![Vec::new(); 2 * base];
    let mut active = BTreeMap::new();
    for (time, &op) in operations.iter().enumerate() {
        let (a, b) = match op {
            Operation::Add(a, b) | Operation::Remove(a, b) | Operation::Connected(a, b) => (a, b),
            Operation::Components => continue,
        };
        for vertex in [a, b] {
            if vertex >= vertices {
                return Err(Error::InvalidVertex { time, vertex });
            }
        }
        let edge = (a.min(b), a.max(b));
        match op {
            Operation::Add(..) => {
                if active.insert(edge, time).is_some() {
                    return Err(Error::DuplicateEdge { time });
                }
            }
            Operation::Remove(..) => {
                let lo = active.remove(&edge).ok_or(Error::AbsentEdge { time })?;
                interval(&mut tree, base, lo, time, edge);
            }
            _ => {}
        }
    }
    for (edge, lo) in active {
        interval(&mut tree, base, lo, operations.len(), edge);
    }
    let mut out = Vec::new();
    visit(
        1,
        &tree,
        base,
        operations,
        &mut Dsu::new(vertices),
        &mut out,
    );
    Ok(out)
}
#[cfg(test)]
mod tests;
