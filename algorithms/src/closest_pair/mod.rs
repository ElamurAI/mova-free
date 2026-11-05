//! Exact planar closest pair with canonical original-index ties.
//! Original divide-and-conquer implementation; classical reference:
//! Shamos and Hoey, "Closest-point problems", FOCS (1975), 151–162.
//! This version re-sorts each strip and therefore costs O(n log² n).
use crate::convex_hull::Point;

/// The lexicographically first original index pair at minimum squared distance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClosestPair {
    pub indices: (usize, usize),
    pub distance_squared: u128,
}

type Indexed = (Point, usize);

fn square_delta(a: i32, b: i32) -> u128 {
    let delta = i128::from(a) - i128::from(b);
    (delta * delta) as u128
}

fn candidate(a: Indexed, b: Indexed) -> ClosestPair {
    ClosestPair {
        indices: (a.1.min(b.1), a.1.max(b.1)),
        distance_squared: square_delta(a.0.x, b.0.x) + square_delta(a.0.y, b.0.y),
    }
}

fn better(a: ClosestPair, b: ClosestPair) -> ClosestPair {
    if (a.distance_squared, a.indices) <= (b.distance_squared, b.indices) {
        a
    } else {
        b
    }
}

/// Returns None exactly when fewer than two input points exist.
/// Exact for full i32 coordinates, including duplicates; input is unchanged.
/// O(n log² n) time, O(n) peak auxiliary storage and O(log n) recursion depth.
pub fn closest_pair(points: &[Point]) -> Option<ClosestPair> {
    if points.len() < 2 {
        return None;
    }
    let mut sorted: Vec<_> = points.iter().copied().zip(0..).collect();
    sorted.sort_unstable();
    // Each duplicate run starts with its smallest two original indices.
    let zero = sorted
        .windows(2)
        .filter(|p| p[0].0 == p[1].0)
        .map(|p| candidate(p[0], p[1]))
        .reduce(better);
    zero.or_else(|| Some(solve(&sorted)))
}

fn solve(points: &[Indexed]) -> ClosestPair {
    if points.len() <= 3 {
        let mut best = candidate(points[0], points[1]);
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                best = better(best, candidate(points[i], points[j]));
            }
        }
        return best;
    }
    let mid = points.len() / 2;
    let split_x = points[mid].0.x;
    let mut best = better(solve(&points[..mid]), solve(&points[mid..]));
    let mut strip: Vec<_> = points
        .iter()
        .copied()
        .filter(|p| square_delta(p.0.x, split_x) <= best.distance_squared)
        .collect();
    strip.sort_unstable_by_key(|p| (p.0.y, p.0.x, p.1));
    for i in 0..strip.len() {
        for j in i + 1..strip.len() {
            if square_delta(strip[j].0.y, strip[i].0.y) > best.distance_squared {
                break;
            }
            best = better(best, candidate(strip[i], strip[j]));
        }
    }
    best
}

#[cfg(test)]
mod tests;
