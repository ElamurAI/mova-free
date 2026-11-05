//! Balanced augmented interval tree; see Cormen et al., Introduction to Algorithms.
#[derive(Debug)]
struct Node {
    interval: (i64, i64),
    index: usize,
    max_end: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
/// Immutable index of <=4096 closed i64 intervals, retaining duplicates.
/// Allocations follow standard library behavior; no callbacks or endpoint arithmetic.
#[derive(Debug, Default)]
pub struct IntervalTree {
    root: Option<Box<Node>>,
}
impl IntervalTree {
    /// Count errors precede reversed endpoints. O(n log n) time, O(n) storage.
    pub fn new(intervals: &[(i64, i64)]) -> Result<Self, &'static str> {
        if intervals.len() > 4096 {
            return Err("too many intervals");
        }
        if intervals.iter().any(|&(a, b)| a > b) {
            return Err("reversed interval");
        }
        let mut entries: Vec<_> = intervals
            .iter()
            .copied()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        entries.sort_unstable();
        fn build(entries: &[((i64, i64), usize)]) -> Option<Box<Node>> {
            if entries.is_empty() {
                return None;
            }
            let m = entries.len() / 2;
            let (interval, index) = entries[m];
            let left = build(&entries[..m]);
            let right = build(&entries[m + 1..]);
            let max_end = interval
                .1
                .max(left.as_ref().map_or(i64::MIN, |n| n.max_end))
                .max(right.as_ref().map_or(i64::MIN, |n| n.max_end));
            Some(Box::new(Node {
                interval,
                index,
                max_end,
                left,
                right,
            }))
        }
        Ok(Self {
            root: build(&entries),
        })
    }
    /// Matching original indexes sorted ascending. Touching endpoints overlap.
    /// Reversed query errors. O(n+k log k) worst time, O(k+log n) scratch.
    pub fn overlapping(&self, query: (i64, i64)) -> Result<Vec<usize>, &'static str> {
        if query.0 > query.1 {
            return Err("reversed interval");
        }
        fn visit(node: &Option<Box<Node>>, q: (i64, i64), out: &mut Vec<usize>) {
            let Some(n) = node else {
                return;
            };
            if n.max_end < q.0 {
                return;
            }
            visit(&n.left, q, out);
            if n.interval.0 > q.1 {
                return;
            }
            if q.0 <= n.interval.1 {
                out.push(n.index);
            }
            visit(&n.right, q, out);
        }
        let mut out = Vec::new();
        visit(&self.root, query, &mut out);
        out.sort_unstable();
        Ok(out)
    }
}
#[cfg(test)]
mod tests;
