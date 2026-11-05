//! Exact median KD tree (Bentley, CACM 1975).
#[derive(Debug)]
struct Node {
    point: [i32; 2],
    index: usize,
    axis: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
/// Immutable <=4096 points with coordinates in +/-1,000,000.
/// Allocations follow standard library behavior; no callbacks.
#[derive(Debug, Default)]
pub struct KdTree {
    root: Option<Box<Node>>,
}
impl KdTree {
    /// Count errors precede coordinate errors. Duplicate indexes are retained.
    /// O(n log² n) construction and O(n) storage.
    pub fn new(points: &[[i32; 2]]) -> Result<Self, &'static str> {
        if points.len() > 4096 {
            return Err("too many points");
        }
        if points
            .iter()
            .flatten()
            .any(|&x| !(-1_000_000..=1_000_000).contains(&x))
        {
            return Err("coordinate out of range");
        }
        fn build(mut points: Vec<([i32; 2], usize)>, depth: usize) -> Option<Box<Node>> {
            if points.is_empty() {
                return None;
            }
            let axis = depth % 2;
            points.sort_unstable_by_key(|&(p, i)| (p[axis], i));
            let right = points.split_off(points.len() / 2 + 1);
            let (point, index) = points.pop().unwrap();
            Some(Box::new(Node {
                point,
                index,
                axis,
                left: build(points, depth + 1),
                right: build(right, depth + 1),
            }))
        }
        Ok(Self {
            root: build(
                points
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, p)| (p, i))
                    .collect(),
                0,
            ),
        })
    }
    /// (Original index, squared distance), ties choose smallest index.
    /// Any i32 query is valid: 2*(2^31+10^6)^2 < u64::MAX.
    /// Empty => None. Worst O(n) time, O(log n) stack.
    pub fn nearest(&self, query: [i32; 2]) -> Option<(usize, u64)> {
        fn visit(node: &Option<Box<Node>>, q: [i32; 2], best: &mut Option<(u64, usize)>) {
            let Some(n) = node else {
                return;
            };
            let dx = (i64::from(q[0]) - i64::from(n.point[0])).unsigned_abs();
            let dy = (i64::from(q[1]) - i64::from(n.point[1])).unsigned_abs();
            let candidate = (dx * dx + dy * dy, n.index);
            if best.is_none_or(|b| candidate < b) {
                *best = Some(candidate);
            }
            let delta = i64::from(q[n.axis]) - i64::from(n.point[n.axis]);
            let (near, far) = if delta < 0 {
                (&n.left, &n.right)
            } else {
                (&n.right, &n.left)
            };
            visit(near, q, best);
            if delta.unsigned_abs().pow(2) <= best.unwrap().0 {
                visit(far, q, best);
            }
        }
        let mut best = None;
        visit(&self.root, query, &mut best);
        best.map(|(d, i)| (i, d))
    }
}
#[cfg(test)]
mod tests;
