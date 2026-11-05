//! Unit-cost edit distance over caller-chosen sequence elements.

/// Minimum insertions, deletions and substitutions transforming a into b.
/// Two rows use O(min(a.len(), b.len())) space; time is O(1 + a.len()*b.len()).
/// Bytes count byte edits; collect Unicode scalar values with chars() for
/// character edits. Grapheme segmentation, if needed, is the caller's choice.
/// Panics if both slices are nonempty and either length is usize::MAX.
/// Allocation failure follows standard Vec behavior.
pub fn distance<T: Eq>(a: &[T], b: &[T]) -> usize {
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if short.is_empty() {
        return long.len();
    }
    assert!(long.len() < usize::MAX, "edit-distance dimension overflow");
    let mut previous: Vec<usize> = (0..=short.len()).collect();
    let mut current = vec![0; short.len() + 1];
    for (i, left) in long.iter().enumerate() {
        current[0] = i + 1;
        for (j, right) in short.iter().enumerate() {
            current[j + 1] = (previous[j + 1] + 1)
                .min(current[j] + 1)
                .min(previous[j] + usize::from(left != right));
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[short.len()]
}

#[cfg(test)]
mod tests;
