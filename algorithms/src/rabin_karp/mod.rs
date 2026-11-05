//! Verified rolling-hash byte search (Karp and Rabin, 1987).
/// All matching byte offsets including overlaps; empty patterns match every
/// boundary. No Unicode normalization. Hash arithmetic wraps modulo 2^64;
/// collisions require byte verification. O(n*m) worst-case time, O(1) auxiliary
/// space plus output. Allocation failure follows Vec behavior.
pub fn find_all(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    if pattern.len() > text.len() {
        return Vec::new();
    }
    let m = pattern.len();
    let base = 257u64;
    let mut power = 1u64;
    for _ in 1..m {
        power = power.wrapping_mul(base);
    }
    let hash = |s: &[u8]| {
        s.iter().fold(0u64, |h, &b| {
            h.wrapping_mul(base).wrapping_add(u64::from(b))
        })
    };
    let target = hash(pattern);
    let mut window = hash(&text[..m]);
    let mut result = Vec::new();
    for start in 0..=text.len() - m {
        if window == target && &text[start..start + m] == pattern {
            result.push(start);
        }
        if start < text.len() - m {
            window = window
                .wrapping_sub(u64::from(text[start]).wrapping_mul(power))
                .wrapping_mul(base)
                .wrapping_add(u64::from(text[start + m]));
        }
    }
    result
}
#[cfg(test)]
mod tests;
