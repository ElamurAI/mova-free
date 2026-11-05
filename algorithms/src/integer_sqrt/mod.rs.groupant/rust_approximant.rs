//! Executable reduced-precision square-root approximant; production code is unchanged.
use mova_approx::ErrorBudget;

/// Lower approximation to floor(sqrt(n)), for every u64.
/// Error is (exact - result)/exact, with zero mapped exactly to zero.
pub fn integer_sqrt(n: u64, budget: ErrorBudget) -> u64 {
    if n == 0 {
        return 0;
    }
    // Keep enough root bits that q >= 1 / budget whenever shifting.
    let kept_log = match budget {
        ErrorBudget::OnePercent => 7,
        ErrorBudget::TwoPercent => 6,
        ErrorBudget::FivePercent => 5,
        ErrorBudget::TenPercent => 4,
    };
    let root_log = (63 - n.leading_zeros()) / 2;
    let shift = root_log.saturating_sub(kept_log);
    let reduced = n >> (2 * shift);
    // This seed is cheap. Integer certification, not floating point accuracy,
    // establishes floor(sqrt(reduced)); reduced <= 65535.
    let mut q = (reduced as f64).sqrt() as u64;
    while q * q > reduced {
        q -= 1;
    }
    while (q + 1) * (q + 1) <= reduced {
        q += 1;
    }
    q << shift
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: u64, budget: ErrorBudget) {
        let exact = n.isqrt();
        let got = integer_sqrt(n, budget);
        assert!(got <= exact);
        assert!(u128::from(exact - got) * 100 <= u128::from(exact) * u128::from(budget.percent()));
    }
    #[test]
    fn all_budgets_extremes_squares_and_random() {
        for b in ErrorBudget::ALL {
            for n in 0..100_000 {
                check(n, b);
            }
            for n in [u64::MAX, u64::MAX - 1, 1 << 63, (1 << 63) - 1] {
                check(n, b);
            }
            for r in [
                1u64,
                2,
                3,
                4,
                7,
                8,
                31,
                32,
                127,
                128,
                255,
                256,
                65535,
                65536,
                u32::MAX as u64,
            ] {
                for n in [r * r - 1, r * r, r * r + 1] {
                    check(n, b);
                }
            }
            let mut n = 1927u64;
            for _ in 0..100_000 {
                n ^= n << 13;
                n ^= n >> 7;
                n ^= n << 17;
                check(n, b);
            }
        }
    }
    #[test]
    fn negative_control_and_nonexact_execution() {
        let exact = 10_000u64.isqrt();
        assert!((exact - 0) * 100 > exact * ErrorBudget::TenPercent.percent() as u64);
        for b in ErrorBudget::ALL {
            assert!(integer_sqrt(u64::MAX, b) < u64::MAX.isqrt());
        }
    }
}
