//! Exact monotone-chain hull; original implementation of Andrew's classical method.
//! Reference: A. M. Andrew, "Another efficient algorithm for convex hulls in
//! two dimensions", Information Processing Letters 9(5), 216–219 (1979).

/// Integer point ordered lexicographically by (x, y).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn turn(a: Point, b: Point, c: Point) -> i128 {
    let dx = i128::from(b.x) - i128::from(a.x);
    let dy = i128::from(b.y) - i128::from(a.y);
    dx * (i128::from(c.y) - i128::from(a.y)) - dy * (i128::from(c.x) - i128::from(a.x))
}

/// Returns extreme vertices counterclockwise from the lexicographic minimum.
/// Duplicates and interior collinear points are omitted; no closing repetition.
/// Empty and singleton inputs retain their distinct points; a line has two ends.
/// O(n log n) time and O(n) extra memory. Full i32 coordinates are exact.
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut sorted = points.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.len() < 3 {
        return sorted;
    }
    let mut lower = Vec::new();
    let mut upper = Vec::new();
    for &point in &sorted {
        append(&mut lower, point);
    }
    for &point in sorted.iter().rev() {
        append(&mut upper, point);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn append(chain: &mut Vec<Point>, point: Point) {
    while chain.len() >= 2 && turn(chain[chain.len() - 2], chain[chain.len() - 1], point) <= 0 {
        chain.pop();
    }
    chain.push(point);
}

#[cfg(test)]
mod tests;
