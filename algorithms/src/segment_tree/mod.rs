//! Dynamic half-open range minima with deterministic leftmost ties.
//! Values must obey Ord laws and Clone must preserve ordering and value identity.
//! Allocation/capacity failure and panicking user comparisons are outside the
//! error contract; invalid-index errors alone guarantee state preservation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidIndex {
    pub index: usize,
    pub len: usize,
}
impl std::fmt::Display for InvalidIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index {} outside length {}", self.index, self.len)
    }
}
impl std::error::Error for InvalidIndex {}
/// Compact iterative tree: leaves occupy [n,2n), nodes hold source indices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentTree<T> {
    values: Vec<T>,
    tree: Vec<usize>,
}
impl<T: Ord + Clone> SegmentTree<T> {
    /// O(n) comparisons, n clones and O(n) index storage.
    pub fn from_slice(values: &[T]) -> Self {
        let n = values.len();
        let size = n.checked_mul(2).expect("segment tree capacity overflow");
        let mut result = Self {
            values: values.to_vec(),
            tree: vec![0; size],
        };
        for i in 0..n {
            result.tree[n + i] = i;
        }
        for i in (1..n).rev() {
            result.tree[i] = result.choose(result.tree[2 * i], result.tree[2 * i + 1]);
        }
        result
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn value(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }
    fn choose(&self, a: usize, b: usize) -> usize {
        match self.values[a].cmp(&self.values[b]) {
            std::cmp::Ordering::Less => a,
            std::cmp::Ordering::Greater => b,
            std::cmp::Ordering::Equal => a.min(b),
        }
    }
    /// Replaces one point in O(log(n+1)); invalid indices preserve all state.
    pub fn update(&mut self, index: usize, value: T) -> Result<(), InvalidIndex> {
        if index >= self.len() {
            return Err(InvalidIndex {
                index,
                len: self.len(),
            });
        }
        self.values[index] = value;
        let mut node = (self.len() + index) / 2;
        while node > 0 {
            self.tree[node] = self.choose(self.tree[2 * node], self.tree[2 * node + 1]);
            node /= 2;
        }
        Ok(())
    }
    /// Leftmost minimum index in [start,end); None for empty or invalid ranges.
    pub fn query_index(&self, start: usize, end: usize) -> Option<usize> {
        if start >= end || end > self.len() {
            return None;
        }
        let mut left = start + self.len();
        let mut right = end + self.len();
        let mut best = None;
        while left < right {
            if left % 2 == 1 {
                best = Some(best.map_or(self.tree[left], |i| self.choose(i, self.tree[left])));
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                best = Some(best.map_or(self.tree[right], |i| self.choose(i, self.tree[right])));
            }
            left /= 2;
            right /= 2;
        }
        best
    }
    /// Minimum reference with the same range contract as query_index.
    pub fn query(&self, start: usize, end: usize) -> Option<&T> {
        self.query_index(start, end).map(|i| &self.values[i])
    }
}
#[cfg(test)]
mod tests;
