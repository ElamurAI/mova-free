//! Deterministic Miller–Rabin primality on all u64 inputs.
//! Strong probable-prime testing: M. O. Rabin, "Probabilistic Algorithm for
//! Testing Primality", Journal of Number Theory 12 (1980), 128–138.
//! The seven-base full-u64 set is attributed to Jim Sinclair in the
//! deterministic variants table at https://miller-rabin.appspot.com/.
//! That finite-range witness result, not Rabin's randomized error bound,
//! supplies the deterministic range guarantee used here.

/// Returns whether n is prime, using seven fixed witnesses and exact u128
/// modular products. Values below two are not prime. O(log n) modular
/// operations for this fixed witness count, O(1) auxiliary storage.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n.is_multiple_of(2) {
        return n == 2;
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    for base in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        let a = base % n;
        if a == 0 {
            continue;
        }
        let mut x =
            crate::modular_power::modular_power(a, d, n).expect("odd candidate is at least three");
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut passes = false;
        for _ in 1..s {
            x = (u128::from(x) * u128::from(x) % u128::from(n)) as u64;
            if x == n - 1 {
                passes = true;
                break;
            }
        }
        if !passes {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests;
