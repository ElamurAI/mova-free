//! Disjoint sets with union by size and iterative full path compression.
//! Equal-size roots choose the smaller root index. Representatives are not
//! necessarily the smallest member after unequal-size unions.

/// An index outside the fixed universe `0..len`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IndexError {
    pub index: usize,
    pub len: usize,
}

impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "index {} outside disjoint-set length {}",
            self.index, self.len
        )
    }
}
impl std::error::Error for IndexError {}

/// A partition of a fixed, densely indexed universe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DisjointSet {
    /// Creates one singleton component per index.
    pub fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
            size: vec![1; len],
            components: len,
        }
    }
    pub fn len(&self) -> usize {
        self.parent.len()
    }
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
    pub fn components(&self) -> usize {
        self.components
    }

    fn validate(&self, index: usize) -> Result<(), IndexError> {
        if index < self.len() {
            Ok(())
        } else {
            Err(IndexError {
                index,
                len: self.len(),
            })
        }
    }

    fn root(&mut self, mut index: usize) -> usize {
        let mut root = index;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[index] != index {
            let next = self.parent[index];
            self.parent[index] = root;
            index = next;
        }
        root
    }

    /// Returns the representative, compressing the traversed path.
    /// An invalid index leaves the complete representation unchanged.
    pub fn find(&mut self, index: usize) -> Result<usize, IndexError> {
        self.validate(index)?;
        Ok(self.root(index))
    }

    /// Merges components and returns whether the partition changed.
    /// Both indices are validated before any compression or merge.
    /// If both are invalid, the error reports `a`.
    pub fn union(&mut self, a: usize, b: usize) -> Result<bool, IndexError> {
        self.validate(a)?;
        self.validate(b)?;
        let mut a = self.root(a);
        let mut b = self.root(b);
        if a == b {
            return Ok(false);
        }
        if self.size[a] < self.size[b] || (self.size[a] == self.size[b] && a > b) {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.components -= 1;
        Ok(true)
    }

    /// Tests membership in the same component, compressing both paths.
    /// An invalid argument leaves the complete representation unchanged.
    pub fn connected(&mut self, a: usize, b: usize) -> Result<bool, IndexError> {
        self.validate(a)?;
        self.validate(b)?;
        Ok(self.root(a) == self.root(b))
    }
}

#[cfg(test)]
mod tests;
