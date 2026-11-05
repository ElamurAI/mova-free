//! Restricted adjacent-transposition edit distance; not a metric.
/// Insertions, deletions, substitutions and adjacent transpositions cost one;
/// no substring can be edited twice. This is OSA, not unrestricted Damerau.
/// Eq elements can be bytes, Unicode scalars or caller-segmented graphemes.
/// O(a.len()*b.len()) time, O(min(a.len(),b.len())) space. Empty input costs the
/// other's length. Nonempty dimensions must be below usize::MAX; candidates
/// saturate to prevent irrelevant overflow. Allocation follows Vec behavior.
pub fn distance<T: Eq>(a: &[T], b: &[T]) -> usize {
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if short.is_empty() {
        return long.len();
    }
    assert!(long.len() < usize::MAX, "alignment dimension overflow");
    let mut older = vec![0usize; short.len() + 1];
    let mut previous: Vec<usize> = (0..=short.len()).collect();
    let mut current = vec![0usize; short.len() + 1];
    for i in 1..=long.len() {
        current[0] = i;
        for j in 1..=short.len() {
            current[j] = previous[j]
                .saturating_add(1)
                .min(current[j - 1].saturating_add(1))
                .min(previous[j - 1].saturating_add(usize::from(long[i - 1] != short[j - 1])));
            if i > 1 && j > 1 && long[i - 1] == short[j - 2] && long[i - 2] == short[j - 1] {
                current[j] = current[j].min(older[j - 2].saturating_add(1));
            }
        }
        std::mem::swap(&mut older, &mut previous);
        std::mem::swap(&mut previous, &mut current);
    }
    previous[short.len()]
}
#[cfg(test)]
mod tests;
