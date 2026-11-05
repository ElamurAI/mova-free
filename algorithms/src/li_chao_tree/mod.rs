//! Discrete minimum Li Chao tree, as described in CP-Algorithms,
//! "Convex hull trick and Li Chao tree". Original implementation.
//! Construction sorts/deduplicates supplied coordinates in O(n log n).
//! Insert and exact registered-coordinate query cost O(log n); storage O(n).
//! Products of i64 coefficients/coordinates plus i64 intercept fit i128.
//! Caps bound storage and insertion IDs; allocator exhaustion follows std.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub slope: i64,
    pub intercept: i64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Minimum {
    pub value: i128,
    pub line_id: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    EmptyCoordinates,
    TooManyCoordinates,
    TooManyLines,
    UnknownCoordinate,
    NoLines,
}
pub const MAX_COORDINATES: usize = 1 << 18;
pub const MAX_LINES: usize = 1 << 20;
#[derive(Debug)]
pub struct LiChaoTree {
    xs: Vec<i64>,
    nodes: Vec<Option<(Line, usize)>>,
    len: usize,
}
fn evaluate((line, line_id): (Line, usize), x: i64) -> Minimum {
    Minimum {
        value: i128::from(line.slope) * i128::from(x) + i128::from(line.intercept),
        line_id,
    }
}
impl LiChaoTree {
    /// Raw supplied length is capped before deduplication; empty input is invalid.
    pub fn new(coordinates: &[i64]) -> Result<Self, Error> {
        if coordinates.len() > MAX_COORDINATES {
            return Err(Error::TooManyCoordinates);
        }
        if coordinates.is_empty() {
            return Err(Error::EmptyCoordinates);
        }
        let mut xs = coordinates.to_vec();
        xs.sort_unstable();
        xs.dedup();
        let nodes = vec![None; 4 * xs.len()];
        Ok(Self { xs, nodes, len: 0 })
    }
    /// Returns the zero-based insertion ID. Equal values prefer the earliest ID.
    pub fn insert(&mut self, line: Line) -> Result<usize, Error> {
        if self.len == MAX_LINES {
            return Err(Error::TooManyLines);
        }
        let id = self.len;
        self.place(1, 0, self.xs.len(), (line, id));
        self.len += 1;
        Ok(id)
    }
    fn place(&mut self, node: usize, lo: usize, hi: usize, mut incoming: (Line, usize)) {
        let Some(mut current) = self.nodes[node] else {
            self.nodes[node] = Some(incoming);
            return;
        };
        let mid = lo + (hi - lo) / 2;
        if evaluate(incoming, self.xs[mid]) < evaluate(current, self.xs[mid]) {
            std::mem::swap(&mut incoming, &mut current);
        }
        self.nodes[node] = Some(current);
        if hi - lo == 1 {
            return;
        }
        if evaluate(incoming, self.xs[lo]) < evaluate(current, self.xs[lo]) {
            self.place(node * 2, lo, mid, incoming);
        } else if evaluate(incoming, self.xs[hi - 1]) < evaluate(current, self.xs[hi - 1]) {
            self.place(node * 2 + 1, mid, hi, incoming);
        }
    }
    /// Unknown-coordinate validation precedes the empty-line error.
    pub fn query(&self, x: i64) -> Result<Minimum, Error> {
        let index = self
            .xs
            .binary_search(&x)
            .map_err(|_| Error::UnknownCoordinate)?;
        let (mut node, mut lo, mut hi) = (1, 0, self.xs.len());
        let mut best = None;
        loop {
            if let Some(line) = self.nodes[node] {
                let value = evaluate(line, x);
                best = Some(best.map_or(value, |old: Minimum| old.min(value)));
            }
            if hi - lo == 1 {
                break;
            }
            let mid = lo + (hi - lo) / 2;
            if index < mid {
                node *= 2;
                hi = mid;
            } else {
                node = node * 2 + 1;
                lo = mid;
            }
        }
        best.ok_or(Error::NoLines)
    }
}
#[cfg(test)]
mod tests;
