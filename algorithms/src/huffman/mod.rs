//! Deterministic optimal binary prefix codes.
use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// One code per original symbol, including zero weights. Empty gives none;
/// singleton gives [false]. Queue ties use creation order (indices then merges).
/// First popped child gets false. Sums use u128. Cost is sum(weight * length).
pub fn codes(weights: &[u64]) -> Vec<Vec<bool>> {
    let n = weights.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![vec![false]];
    }
    let mut queue = BinaryHeap::new();
    let mut parents: Vec<Option<(usize, bool)>> = vec![None; n];
    for (i, &weight) in weights.iter().enumerate() {
        queue.push(Reverse((u128::from(weight), i)));
    }
    while queue.len() > 1 {
        let Reverse((a, left)) = queue.pop().expect("two nodes");
        let Reverse((b, right)) = queue.pop().expect("second node");
        let parent = parents.len();
        parents.push(None);
        parents[left] = Some((parent, false));
        parents[right] = Some((parent, true));
        queue.push(Reverse((a + b, parent)));
    }
    (0..n)
        .map(|symbol| {
            let mut code = Vec::new();
            let mut node = symbol;
            while let Some((parent, bit)) = parents[node] {
                code.push(bit);
                node = parent;
            }
            code.reverse();
            code
        })
        .collect()
}
#[cfg(test)]
mod tests;
