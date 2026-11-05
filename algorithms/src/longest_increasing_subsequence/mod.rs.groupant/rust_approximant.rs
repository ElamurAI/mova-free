//! Greedy increasing witness with an integer interval certificate and exact fallback.
//! L <= OPT <= n; accept n-L <= floor(p*n/100), hence (OPT-L)/OPT <= p/100.
use mova_algorithms::longest_increasing_subsequence;
use mova_approx::ErrorBudget;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Estimate {
    pub indices: Vec<usize>,
    pub upper_bound: usize,
    pub used_exact_fallback: bool,
}

/// Overflow-safe floor(p*upper/100), with no floating-point comparison.
pub fn certified(lower: usize, upper: usize, budget: ErrorBudget) -> bool {
    let p = budget.percent() as usize;
    lower <= upper && upper - lower <= (upper / 100) * p + (upper % 100) * p / 100
}

/// Return a valid strictly increasing witness; accepted tie choices may differ
/// from production. Empty input has exact zero objective and zero loss.
pub fn indices<T: Ord>(values: &[T], budget: ErrorBudget) -> Estimate {
    let mut witness: Vec<usize> = Vec::new();
    for (i, value) in values.iter().enumerate() {
        if witness.last().is_none_or(|&j| values[j] < *value) {
            witness.push(i);
        }
    }
    if certified(witness.len(), values.len(), budget) {
        return Estimate {
            indices: witness,
            upper_bound: values.len(),
            used_exact_fallback: false,
        };
    }
    drop(witness);
    Estimate {
        indices: longest_increasing_subsequence::indices(values),
        upper_bound: values.len(),
        used_exact_fallback: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute(values: &[i64]) -> usize {
        (0..1usize << values.len())
            .map(|mask| {
                let selected: Vec<_> = (0..values.len()).filter(|i| mask & (1 << i) != 0).collect();
                if selected.windows(2).all(|w| values[w[0]] < values[w[1]]) {
                    selected.len()
                } else {
                    0
                }
            })
            .max()
            .unwrap()
    }

    fn check(values: &[i64], optimum: usize, budget: ErrorBudget) -> Estimate {
        let result = indices(values, budget);
        assert!(result.indices.iter().all(|&i| i < values.len()));
        assert!(
            result
                .indices
                .windows(2)
                .all(|w| w[0] < w[1] && values[w[0]] < values[w[1]])
        );
        assert!(result.indices.len() <= optimum && optimum <= result.upper_bound);
        assert!((optimum - result.indices.len()) * 100 <= optimum * budget.percent() as usize);
        assert_eq!(result, indices(values, budget));
        if result.used_exact_fallback {
            assert_eq!(
                result.indices,
                longest_increasing_subsequence::indices(values)
            );
        }
        result
    }

    #[test]
    fn enumerated_masks_independent_oracle() {
        for n in 0..=7u32 {
            for mut encoding in 0..3usize.pow(n) {
                let values: Vec<i64> = (0..n)
                    .map(|_| {
                        let v = (encoding % 3) as i64 - 1;
                        encoding /= 3;
                        v
                    })
                    .collect();
                let optimum = brute(&values);
                assert_eq!(
                    longest_increasing_subsequence::indices(&values).len(),
                    optimum
                );
                for budget in ErrorBudget::ALL {
                    check(&values, optimum, budget);
                }
            }
        }
    }

    #[test]
    fn greedy_trap_requires_exact_fallback() {
        let values = [100, 0, 1, 2, 3, 4];
        for budget in ErrorBudget::ALL {
            assert!(check(&values, 5, budget).used_exact_fallback);
        }
    }

    #[test]
    fn near_sorted_nonexact_certified_each_budget() {
        for budget in ErrorBudget::ALL {
            let n = 200usize.div_ceil(budget.percent() as usize);
            let mut values: Vec<i64> = (0..n as i64).collect();
            values[..3].copy_from_slice(&[2, 0, 1]);
            let result = check(&values, n - 1, budget);
            assert!(!result.used_exact_fallback);
            assert_eq!(result.indices.len(), n - 2);
            assert_eq!(
                longest_increasing_subsequence::indices(&values).len(),
                n - 1
            );
        }
    }

    #[test]
    fn zeros_duplicates_extremes_and_order() {
        for budget in ErrorBudget::ALL {
            assert!(!check(&[], 0, budget).used_exact_fallback);
            for values in [
                vec![i64::MIN],
                vec![7; 20],
                vec![i64::MAX, i64::MIN, 0, i64::MAX],
                vec![i64::MIN, 0, i64::MAX],
                (0..10).rev().collect(),
            ] {
                check(&values, brute(&values), budget);
            }
        }
    }

    #[test]
    fn certificate_boundaries_and_overflow() {
        for budget in ErrorBudget::ALL {
            let p = budget.percent() as usize;
            for upper in 0..=200usize {
                for lower in 0..=upper + 1 {
                    assert_eq!(
                        certified(lower, upper, budget),
                        lower <= upper && (upper - lower) * 100 <= upper * p
                    );
                }
            }
            let allowed = (usize::MAX / 100) * p + (usize::MAX % 100) * p / 100;
            assert!(certified(usize::MAX - allowed, usize::MAX, budget));
            assert!(!certified(usize::MAX - allowed - 1, usize::MAX, budget));
            assert!(!certified(0, usize::MAX, budget));
        }
    }
}
