//! Linear prefix matches over bytes; UTF-8 text is compared as bytes.
/// Returns the longest common-prefix length of `input` and each suffix.
/// The first entry is the input length. Empty input returns an empty vector.
/// Runs in O(n) time and O(n) output space, with constant auxiliary space.
pub fn z_algorithm(input: &[u8]) -> Vec<usize> {
    let n = input.len();
    let mut z = vec![0; n];
    if n == 0 {
        return z;
    }
    z[0] = n;
    let (mut left, mut right) = (0, 0);
    for i in 1..n {
        if i < right {
            z[i] = z[i - left].min(right - i);
        }
        while z[i] < n - i && input[z[i]] == input[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > right {
            left = i;
            right = i + z[i];
        }
    }
    z
}
#[cfg(test)]
mod tests;
