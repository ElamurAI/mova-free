//! Horspool's byte-alphabet shift search (Horspool, 1980).
/// All exact byte offsets, increasing, including overlaps. Empty patterns match
/// all boundaries. Bytes need not encode Unicode. Worst-case O(n*m) time;
/// O(256) auxiliary space plus result. Allocation follows Vec behavior.
/// A remaining-distance guard prevents offset overflow.
pub fn find_all(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    let m = pattern.len();
    if m > text.len() {
        return Vec::new();
    }
    let mut shift = [m; 256];
    for (i, &byte) in pattern[..m - 1].iter().enumerate() {
        shift[usize::from(byte)] = m - 1 - i;
    }
    let last = text.len() - m;
    let mut start = 0;
    let mut result = Vec::new();
    loop {
        let mut matched = m;
        while matched > 0 && text[start + matched - 1] == pattern[matched - 1] {
            matched -= 1;
        }
        if matched == 0 {
            result.push(start);
        }
        let step = shift[usize::from(text[start + m - 1])];
        if step > last - start {
            break;
        }
        start += step;
    }
    result
}
#[cfg(test)]
mod tests;
