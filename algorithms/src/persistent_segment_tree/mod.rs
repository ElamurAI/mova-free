//! Path-copy persistent point replacement and half-open range sums.
//! Origin: Driscoll et al., "Making Data Structures Persistent" (1989).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidVersion,
    InvalidIndex,
    InvalidRange,
    Overflow,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Node {
    sum: i128,
    left: usize,
    right: usize,
}
/// Arena nodes are immutable after insertion. Roots select historical versions.
/// No configured memory cap; allocation failure is outside Result semantics.
#[derive(Clone, Debug)]
pub struct PersistentSegmentTree {
    len: usize,
    nodes: Vec<Node>,
    roots: Vec<Option<usize>>,
}
impl PersistentSegmentTree {
    /// Build version zero. Empty input has one empty version and no nodes.
    /// Sums are i128; every sum of a realizable i64 slice fits on <=64-bit targets.
    pub fn from_slice(values: &[i64]) -> Result<Self, Error> {
        let mut tree = Self {
            len: values.len(),
            nodes: Vec::new(),
            roots: Vec::new(),
        };
        let root = if values.is_empty() {
            None
        } else {
            Some(tree.build(values)?)
        };
        tree.roots.push(root);
        Ok(tree)
    }
    fn push(&mut self, node: Node) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }
    fn build(&mut self, values: &[i64]) -> Result<usize, Error> {
        if values.len() == 1 {
            return Ok(self.push(Node {
                sum: i128::from(values[0]),
                left: 0,
                right: 0,
            }));
        }
        let mid = values.len() / 2;
        let left = self.build(&values[..mid])?;
        let right = self.build(&values[mid..])?;
        let sum = self.nodes[left]
            .sum
            .checked_add(self.nodes[right].sum)
            .ok_or(Error::Overflow)?;
        Ok(self.push(Node { sum, left, right }))
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn version_count(&self) -> usize {
        self.roots.len()
    }
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// Replace one element of any existing version; return the new version ID.
    /// Validate version before index. Every returned error preserves roots and
    /// the exact node arena. Successful updates copy at most ceil(log2(n))+1 nodes.
    pub fn set(&mut self, version: usize, index: usize, value: i64) -> Result<usize, Error> {
        let root = *self.roots.get(version).ok_or(Error::InvalidVersion)?;
        if index >= self.len {
            return Err(Error::InvalidIndex);
        }
        let checkpoint = self.nodes.len();
        match self.replace(root.unwrap(), 0, self.len, index, value) {
            Ok(root) => {
                let version = self.roots.len();
                self.roots.push(Some(root));
                Ok(version)
            }
            Err(error) => {
                self.nodes.truncate(checkpoint);
                Err(error)
            }
        }
    }
    fn replace(
        &mut self,
        id: usize,
        start: usize,
        end: usize,
        index: usize,
        value: i64,
    ) -> Result<usize, Error> {
        if end - start == 1 {
            return Ok(self.push(Node {
                sum: i128::from(value),
                left: 0,
                right: 0,
            }));
        }
        let mut node = self.nodes[id];
        let mid = start + (end - start) / 2;
        if index < mid {
            node.left = self.replace(node.left, start, mid, index, value)?;
        } else {
            node.right = self.replace(node.right, mid, end, index, value)?;
        }
        node.sum = self.nodes[node.left]
            .sum
            .checked_add(self.nodes[node.right].sum)
            .ok_or(Error::Overflow)?;
        Ok(self.push(node))
    }
    /// Mathematical sum in [start,end). Empty ranges return zero.
    /// Validate version before bounds; reversed/out-of-bounds intervals fail.
    pub fn range_sum(&self, version: usize, start: usize, end: usize) -> Result<i128, Error> {
        let root = *self.roots.get(version).ok_or(Error::InvalidVersion)?;
        if start > end || end > self.len {
            return Err(Error::InvalidRange);
        }
        if start == end {
            return Ok(0);
        }
        self.query(root.unwrap(), 0, self.len, start, end)
    }
    fn query(
        &self,
        id: usize,
        start: usize,
        end: usize,
        a: usize,
        b: usize,
    ) -> Result<i128, Error> {
        if b <= start || end <= a {
            return Ok(0);
        }
        if a <= start && end <= b {
            return Ok(self.nodes[id].sum);
        }
        let mid = start + (end - start) / 2;
        self.query(self.nodes[id].left, start, mid, a, b)?
            .checked_add(self.query(self.nodes[id].right, mid, end, a, b)?)
            .ok_or(Error::Overflow)
    }
}
#[cfg(test)]
mod tests;
