//! Exact byte search by critical factorization and periodic memory.
//! Origin: Crochemore and Perrin, "Two-way string-matching", JACM 38 (1991).
//! Original implementation; byte ordering, no Unicode normalization.

/// All matching byte offsets, ascending, including overlaps.
/// Empty patterns match every boundary. O(n+m+z) time, O(1) auxiliary space
/// excluding z returned offsets. Output allocation failure may panic/abort.
pub fn find_all(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();
    if m == 0 {
        return (0..=n).collect();
    }
    if m > n {
        return Vec::new();
    }
    let (a, ap) = maximal_suffix(pattern, false);
    let (b, bp) = maximal_suffix(pattern, true);
    let (cut, mut period) = if a > b { (a, ap) } else { (b, bp) };
    let periodic = period <= m - cut && pattern[..cut] == pattern[period..period + cut];
    let remembered = if periodic {
        m - period
    } else {
        period = cut.max(m - cut + 1);
        0
    };
    let mut memory = 0;
    let mut position = 0;
    let mut result = Vec::new();
    while position <= n - m {
        let mut k = cut.max(memory);
        while k < m && pattern[k] == text[position + k] {
            k += 1;
        }
        if k < m {
            position += k - cut + 1;
            memory = 0;
        } else {
            k = cut;
            while k > memory && pattern[k - 1] == text[position + k - 1] {
                k -= 1;
            }
            if k <= memory {
                result.push(position);
            }
            position += period;
            memory = remembered;
        }
    }
    result
}

// The suffix start is represented as ms+1, avoiding a negative index sentinel.
// At each step j is the competing suffix and p its candidate period.
fn maximal_suffix(x: &[u8], reverse: bool) -> (usize, usize) {
    let (mut start, mut j, mut k, mut p) = (0, 0, 1, 1);
    while j + k < x.len() {
        let left = x[j + k];
        let right = x[start + k - 1];
        if left == right {
            if k == p {
                j += p;
                k = 1;
            } else {
                k += 1;
            }
        } else if (left < right) != reverse {
            j += k;
            k = 1;
            p = j + 1 - start;
        } else {
            start = j + 1;
            j = start;
            k = 1;
            p = 1;
        }
    }
    (start, p)
}
#[cfg(test)]
mod tests;
