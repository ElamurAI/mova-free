//! Lowest common ancestors in a validated rooted forest, using binary lifting.
//! Original std-only implementation of the classical doubling technique.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidParent { vertex: usize, parent: usize },
    Cycle { vertex: usize },
    InvalidVertex { vertex: usize, vertices: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lca {
    root: Vec<usize>,
    depth: Vec<usize>,
    up: Vec<Vec<usize>>,
}

impl Lca {
    /// None denotes a root. Validates indices in order, then detects cycles
    /// iteratively from increasing vertex indices. Empty forests are valid.
    /// O(n log(n+1)) time and space; allocation failure is outside Result.
    pub fn new(parents: &[Option<usize>]) -> Result<Self, Error> {
        let n = parents.len();
        for (vertex, parent) in parents.iter().enumerate() {
            if let Some(parent) = *parent
                && parent >= n
            {
                return Err(Error::InvalidParent { vertex, parent });
            }
        }
        let mut state = vec![0u8; n];
        let mut root = vec![0; n];
        let mut depth = vec![0; n];
        for start in 0..n {
            if state[start] != 0 {
                continue;
            }
            let mut path = Vec::new();
            let mut next = Some(start);
            while let Some(vertex) = next {
                if state[vertex] == 1 {
                    return Err(Error::Cycle { vertex });
                }
                if state[vertex] == 2 {
                    break;
                }
                state[vertex] = 1;
                path.push(vertex);
                next = parents[vertex];
            }
            while let Some(vertex) = path.pop() {
                if let Some(parent) = parents[vertex] {
                    root[vertex] = root[parent];
                    depth[vertex] = depth[parent] + 1;
                } else {
                    root[vertex] = vertex;
                }
                state[vertex] = 2;
            }
        }
        let mut up = Vec::new();
        if n > 0 {
            up.push(
                parents
                    .iter()
                    .enumerate()
                    .map(|(v, p)| p.unwrap_or(v))
                    .collect::<Vec<_>>(),
            );
            let levels = usize::BITS - n.leading_zeros();
            for level in 1..levels as usize {
                let previous = &up[level - 1];
                up.push(previous.iter().map(|&v| previous[v]).collect());
            }
        }
        Ok(Self { root, depth, up })
    }

    /// Returns None for distinct trees. O(log(n+1)) time, O(1) extra space.
    /// Invalid endpoints are checked left to right, even across components.
    pub fn query(&self, mut a: usize, mut b: usize) -> Result<Option<usize>, Error> {
        for vertex in [a, b] {
            if vertex >= self.root.len() {
                return Err(Error::InvalidVertex {
                    vertex,
                    vertices: self.root.len(),
                });
            }
        }
        if self.root[a] != self.root[b] {
            return Ok(None);
        }
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        let difference = self.depth[a] - self.depth[b];
        for (level, ancestors) in self.up.iter().enumerate() {
            if difference & (1usize << level) != 0 {
                a = ancestors[a];
            }
        }
        if a == b {
            return Ok(Some(a));
        }
        for ancestors in self.up.iter().rev() {
            if ancestors[a] != ancestors[b] {
                a = ancestors[a];
                b = ancestors[b];
            }
        }
        Ok(Some(self.up[0][a]))
    }
}

#[cfg(test)]
mod tests;
