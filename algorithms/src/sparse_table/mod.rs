//! Immutable range minima with integer logarithms and leftmost tie selection.
/// Overlapping power-of-two minima stored as original indices.
/// T must obey Ord laws; construction clones each input value once.
#[derive(Clone, Debug)]
pub struct SparseTable<T> {
    values: Vec<T>,
    levels: Vec<Vec<usize>>,
}
impl<T: Ord + Clone> SparseTable<T> {
    /// Builds O(n log(n+1)) indices and clones n values.
    pub fn from_slice(values: &[T]) -> Self {
        let values = values.to_vec();
        let n = values.len();
        let mut levels: Vec<Vec<usize>> = Vec::new();
        if n > 0 {
            levels.push((0..n).collect());
            let mut width = 2;
            while width <= n {
                let half = width / 2;
                let previous = levels.last().unwrap();
                let row = (0..=n - width)
                    .map(|start| Self::choose(&values, previous[start], previous[start + half]))
                    .collect();
                levels.push(row);
                let Some(next) = width.checked_mul(2) else {
                    break;
                };
                width = next;
            }
        }
        Self { values, levels }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    fn choose(values: &[T], a: usize, b: usize) -> usize {
        match values[a].cmp(&values[b]) {
            std::cmp::Ordering::Less => a,
            std::cmp::Ordering::Greater => b,
            std::cmp::Ordering::Equal => a.min(b),
        }
    }
    /// Minimum of nonempty [start,end), with the leftmost input winning ties.
    /// Returns None for empty, reversed, or out-of-bounds ranges.
    pub fn query(&self, start: usize, end: usize) -> Option<&T> {
        if start >= end || end > self.len() {
            return None;
        }
        let length = end - start;
        let level = (usize::BITS - 1 - length.leading_zeros()) as usize;
        let width = 1usize << level;
        let a = self.levels[level][start];
        let b = self.levels[level][end - width];
        Some(&self.values[Self::choose(&self.values, a, b)])
    }
}
#[cfg(test)]
mod tests;
