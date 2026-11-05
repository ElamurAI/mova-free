//! Linear odd/even palindrome radii over bytes, not Unicode characters.
/// Maximal radii at every byte position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PalindromeRadii {
    /// Includes the center: span `i + 1 - odd[i]..i + odd[i]`.
    pub odd: Vec<usize>,
    /// Centered immediately before i: span `i - even[i]..i + even[i]`.
    pub even: Vec<usize>,
}
/// Computes maximal palindrome radii in O(n) time and O(n) output space.
/// Empty input has two empty vectors; every nonempty odd radius is at least one.
/// UTF-8 callers can pass `text.as_bytes()`; returned offsets are byte offsets.
pub fn manacher(input: &[u8]) -> PalindromeRadii {
    let n = input.len();
    let mut odd = vec![0; n];
    let (mut left, mut right) = (0, 0);
    for i in 0..n {
        let mut radius = if i < right {
            odd[left + (right - 1 - i)].min(right - i)
        } else {
            1
        };
        while radius <= i && radius < n - i && input[i - radius] == input[i + radius] {
            radius += 1;
        }
        odd[i] = radius;
        if i + radius > right {
            left = i + 1 - radius;
            right = i + radius;
        }
    }
    let mut even = vec![0; n];
    (left, right) = (0, 0);
    for i in 0..n {
        let mut radius = if i < right {
            even[left + (right - i)].min(right - i)
        } else {
            0
        };
        while radius < i && radius < n - i && input[i - 1 - radius] == input[i + radius] {
            radius += 1;
        }
        even[i] = radius;
        if i + radius > right {
            left = i - radius;
            right = i + radius;
        }
    }
    PalindromeRadii { odd, even }
}
#[cfg(test)]
mod tests;
