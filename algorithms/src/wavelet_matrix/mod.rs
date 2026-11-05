//! Static unsigned range order statistics.
//! Origin: Claude, Navarro and Ordóñez, "The wavelet matrix" (2015).
//! This implementation uses usize prefixes, not succinct rank bitvectors.
#[derive(Clone, Debug)]
pub struct WaveletMatrix {
    len: usize,
    levels: Vec<Vec<usize>>,
    zeros: Vec<usize>,
}
impl WaveletMatrix {
    /// O(64n) work; 64(n+1) usize prefixes plus 64 totals.
    /// Allocation failure is outside the query Option contract.
    pub fn from_slice(values: &[u64]) -> Self {
        let mut current = values.to_vec();
        let mut levels = Vec::with_capacity(64);
        let mut zeros = Vec::with_capacity(64);
        for bit in (0..64).rev() {
            let mut prefix =
                Vec::with_capacity(values.len().checked_add(1).expect("length overflow"));
            prefix.push(0);
            for &value in &current {
                prefix.push(prefix.last().unwrap() + usize::from(value & (1u64 << bit) == 0));
            }
            let zero = *prefix.last().unwrap();
            let mut next = Vec::with_capacity(values.len());
            next.extend(current.iter().copied().filter(|x| x & (1u64 << bit) == 0));
            next.extend(current.iter().copied().filter(|x| x & (1u64 << bit) != 0));
            current = next;
            levels.push(prefix);
            zeros.push(zero);
        }
        Self {
            len: values.len(),
            levels,
            zeros,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn valid(&self, start: usize, end: usize) -> bool {
        start <= end && end <= self.len
    }
    /// Number equal to value in [start,end); empty ranges return Some(0).
    pub fn rank(&self, mut start: usize, mut end: usize, value: u64) -> Option<usize> {
        if !self.valid(start, end) {
            return None;
        }
        for (level, prefix) in self.levels.iter().enumerate() {
            let (a, b) = (prefix[start], prefix[end]);
            if value & (1u64 << (63 - level)) == 0 {
                start = a;
                end = b;
            } else {
                start = self.zeros[level] + start - a;
                end = self.zeros[level] + end - b;
            }
        }
        Some(end - start)
    }
    /// Number strictly below upper in [start,end), using unsigned order.
    pub fn count_less(&self, mut start: usize, mut end: usize, upper: u64) -> Option<usize> {
        if !self.valid(start, end) {
            return None;
        }
        let mut count = 0;
        for (level, prefix) in self.levels.iter().enumerate() {
            let (a, b) = (prefix[start], prefix[end]);
            if upper & (1u64 << (63 - level)) == 0 {
                start = a;
                end = b;
            } else {
                count += b - a;
                start = self.zeros[level] + start - a;
                end = self.zeros[level] + end - b;
            }
        }
        Some(count)
    }
    /// Count values in [lower,upper) within [start,end).
    /// Reversed index or value bounds return None. To include u64::MAX use rank
    /// separately or subtract count_less(lower) from the interval length.
    pub fn range_count(&self, start: usize, end: usize, lower: u64, upper: u64) -> Option<usize> {
        if lower > upper {
            return None;
        }
        Some(self.count_less(start, end, upper)? - self.count_less(start, end, lower)?)
    }
    /// Zero-based kth smallest in [start,end), counting duplicates separately.
    /// Invalid bounds, empty ranges and k >= end-start return None.
    pub fn kth(&self, mut start: usize, mut end: usize, mut k: usize) -> Option<u64> {
        if !self.valid(start, end) || k >= end - start {
            return None;
        }
        let mut value = 0;
        for (level, prefix) in self.levels.iter().enumerate() {
            let (a, b) = (prefix[start], prefix[end]);
            if k < b - a {
                start = a;
                end = b;
            } else {
                k -= b - a;
                value |= 1u64 << (63 - level);
                start = self.zeros[level] + start - a;
                end = self.zeros[level] + end - b;
            }
        }
        Some(value)
    }
}
#[cfg(test)]
mod tests;
