//! Extended Euclidean algorithm with full-width unsigned inputs.

/// A gcd and signed coefficients satisfying a*x + b*y = gcd.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bezout {
    pub gcd: u64,
    pub x: i128,
    pub y: i128,
}

/// Returns deterministic Bezout coefficients; (0,0) yields all zero fields.
/// Takes O(log(max(a,b)+1)) divisions and O(1) space.
/// Coefficients, including the terminal zero row, have magnitude at most
/// max(a,b); quotient products fit i128 for all u64 inputs.
pub fn extended_gcd(a: u64, b: u64) -> Bezout {
    if a == 0 && b == 0 {
        return Bezout { gcd: 0, x: 0, y: 0 };
    }
    let (mut old_r, mut r) = (a, b);
    let (mut old_x, mut x) = (1_i128, 0_i128);
    let (mut old_y, mut y) = (0_i128, 1_i128);
    while r != 0 {
        let quotient = i128::from(old_r / r);
        (old_r, r) = (r, old_r % r);
        (old_x, x) = (x, old_x - quotient * x);
        (old_y, y) = (y, old_y - quotient * y);
    }
    Bezout {
        gcd: old_r,
        x: old_x,
        y: old_y,
    }
}

#[cfg(test)]
mod tests;
