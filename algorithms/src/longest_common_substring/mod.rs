//! Longest contiguous common slice by rolling dynamic programming.
/// A contiguous witness in caller-selected element units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Match {
    pub a_start: usize,
    pub b_start: usize,
    pub len: usize,
}
/// Maximizes length, then minimizes (a_start,b_start). No common element returns
/// (0,0,0). Bytes give byte offsets; chars give scalar offsets. No normalization
/// or grapheme segmentation. O(a.len()*b.len()) time, O(b.len()) space.
/// Panics if b.len()+1 overflows; allocation failure follows Vec behavior.
pub fn longest_common_substring<T: Eq>(a: &[T], b: &[T]) -> Match {
    if a.is_empty() || b.is_empty() {
        return Match::default();
    }
    let width = b
        .len()
        .checked_add(1)
        .expect("substring dimension overflow");
    let mut row = vec![0usize; width];
    let mut best = Match::default();
    for (i, left) in a.iter().enumerate() {
        for j in (0..b.len()).rev() {
            row[j + 1] = if left == &b[j] { row[j] + 1 } else { 0 };
            let len = row[j + 1];
            let candidate = Match {
                a_start: i + 1 - len,
                b_start: j + 1 - len,
                len,
            };
            if len > best.len
                || (len == best.len
                    && len > 0
                    && (candidate.a_start, candidate.b_start) < (best.a_start, best.b_start))
            {
                best = candidate;
            }
        }
    }
    best
}
#[cfg(test)]
mod tests;
