//! Dynamic sums over i64 points accumulated exactly in i128.
//! Supports targets with at most 64-bit usize; allocation failure is not handled.
const _: () = assert!(usize::BITS <= 64);
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidIndex {
        index: usize,
        len: usize,
    },
    InvalidRange {
        start: usize,
        end: usize,
        len: usize,
    },
    PointOverflow {
        index: usize,
        value: i64,
        delta: i64,
    },
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
/// Current bounded points and exact Fenwick block sums.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fenwick {
    values: Vec<i64>,
    tree: Vec<i128>,
}
impl Fenwick {
    /// Builds in O(n) time.
    pub fn from_slice(values: &[i64]) -> Self {
        let mut tree: Vec<i128> = values.iter().map(|&x| i128::from(x)).collect();
        for i in 0..tree.len() {
            let parent = i | (i + 1);
            if parent < tree.len() {
                tree[parent] += tree[i];
            }
        }
        Self {
            values: values.to_vec(),
            tree,
        }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn value(&self, index: usize) -> Option<i64> {
        self.values.get(index).copied()
    }
    /// Invalid indices and i64 point overflow preserve the complete state.
    pub fn add(&mut self, index: usize, delta: i64) -> Result<(), Error> {
        let value = self.value(index).ok_or(Error::InvalidIndex {
            index,
            len: self.len(),
        })?;
        let updated = value.checked_add(delta).ok_or(Error::PointOverflow {
            index,
            value,
            delta,
        })?;
        self.values[index] = updated;
        let mut i = index;
        while i < self.len() {
            self.tree[i] += i128::from(delta);
            i |= i + 1;
        }
        Ok(())
    }
    /// Sum of [0,end); zero is valid even for an empty tree.
    pub fn prefix_sum(&self, end: usize) -> Result<i128, Error> {
        if end > self.len() {
            return Err(Error::InvalidRange {
                start: 0,
                end,
                len: self.len(),
            });
        }
        Ok(self.prefix(end))
    }
    fn prefix(&self, mut end: usize) -> i128 {
        let mut sum = 0;
        while end > 0 {
            sum += self.tree[end - 1];
            end &= end - 1;
        }
        sum
    }
    /// Sum of [start,end); empty ranges valid, reversed/out-of-bounds are errors.
    pub fn range_sum(&self, start: usize, end: usize) -> Result<i128, Error> {
        if start > end || end > self.len() {
            return Err(Error::InvalidRange {
                start,
                end,
                len: self.len(),
            });
        }
        Ok(self.prefix(end) - self.prefix(start))
    }
}
#[cfg(test)]
mod tests;
