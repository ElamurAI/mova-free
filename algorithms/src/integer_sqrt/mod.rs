//! Integer square root using integer-only binary search.

/// Returns floor(sqrt(n)) for every u64, without floating-point arithmetic.
/// Uses at most 33 iterations and constant space; u128 squares cannot overflow.
pub fn integer_sqrt(n: u64) -> u64 {
    let (mut lo, mut hi) = (0u64, 1u64 << 32);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if u128::from(mid) * u128::from(mid) <= u128::from(n) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo - 1
}

#[cfg(test)]
mod tests;
