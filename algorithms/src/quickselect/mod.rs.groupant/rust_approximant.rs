//! Sampled order statistic with a deterministic full-input rank certificate.
use mova_approx::ErrorBudget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RankEstimate {
    pub value: u64,
    /// Distance of requested index to this value's occupied sorted-rank interval.
    pub rank_error: usize,
    pub used_exact_fallback: bool,
}

/// Approximate zero-based order statistic of an immutable slice.
/// Metric: rank_error / values.len(), not numerical value error.
/// Does not provide the production select_nth in-place partition postcondition.
/// Invalid index (including an empty slice) returns None.
pub fn select_nth(values: &[u64], index: usize, budget: ErrorBudget) -> Option<RankEstimate> {
    let n = values.len();
    if index >= n {
        return None;
    }
    let sample_size = match budget {
        ErrorBudget::OnePercent => 257,
        ErrorBudget::TwoPercent => 129,
        ErrorBudget::FivePercent => 65,
        ErrorBudget::TenPercent => 33,
    }
    .min(n);
    let mut sample = [0u64; 257];
    for (i, slot) in sample[..sample_size].iter_mut().enumerate() {
        // u128 avoids overflow in index arithmetic on 64-bit targets.
        let at = (i as u128 * n as u128 / sample_size as u128) as usize;
        *slot = values[at];
    }
    let sample_index = (index as u128 * sample_size as u128 / n as u128) as usize;
    let candidate = *sample[..sample_size].select_nth_unstable(sample_index).1;
    let mut less = 0;
    let mut equal = 0;
    for &value in values {
        less += usize::from(value < candidate);
        equal += usize::from(value == candidate);
    }
    let distance = if index < less {
        less - index
    } else {
        index.saturating_sub(less + equal - 1)
    };
    if distance as u128 * 100 <= n as u128 * budget.percent() as u128 {
        return Some(RankEstimate {
            value: candidate,
            rank_error: distance,
            used_exact_fallback: false,
        });
    }
    let mut copy = values.to_vec();
    let value = *copy.select_nth_unstable(index).1;
    Some(RankEstimate {
        value,
        rank_error: 0,
        used_exact_fallback: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check(values: &[u64], k: usize, b: ErrorBudget) {
        let got = select_nth(values, k, b).unwrap();
        // Independent sorted-vector reference, including duplicate rank intervals.
        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        let lo = sorted.partition_point(|v| *v < got.value);
        let hi = sorted.partition_point(|v| *v <= got.value);
        assert!(hi > lo);
        let distance = if k < lo {
            lo - k
        } else {
            k.saturating_sub(hi - 1)
        };
        assert_eq!(distance, got.rank_error);
        assert!(distance as u128 * 100 <= values.len() as u128 * b.percent() as u128);
        if got.used_exact_fallback {
            assert_eq!(got.value, sorted[k]);
        }
    }
    #[test]
    fn budgets_domains_duplicates_and_extremes() {
        for b in ErrorBudget::ALL {
            assert_eq!(select_nth(&[], 0, b), None);
            assert_eq!(select_nth(&[4], 1, b), None);
            for n in [1, 2, 3, 8, 31, 64, 257, 1024] {
                let mut seed = 9187u64;
                let random: Vec<_> = (0..n)
                    .map(|_| {
                        seed ^= seed << 13;
                        seed ^= seed >> 7;
                        seed ^= seed << 17;
                        seed
                    })
                    .collect();
                for values in [
                    random,
                    (0..n as u64).collect(),
                    (0..n as u64).rev().collect(),
                    vec![u64::MAX; n],
                    (0..n).map(|i| (i % 3) as u64).collect(),
                ] {
                    for k in [0, n / 4, n / 2, n - 1] {
                        check(&values, k, b);
                    }
                }
            }
        }
    }
    #[test]
    fn all_profiles_accept_nonexact_certified_rank() {
        let values: Vec<_> = (0..4096u64).collect();
        for budget in ErrorBudget::ALL {
            let estimate = select_nth(&values, 2048, budget).unwrap();
            assert!(!estimate.used_exact_fallback);
            assert!(estimate.rank_error > 0);
            check(&values, 2048, budget);
        }
    }

    #[test]
    fn adversarial_sampling_falls_back_and_bad_rank_is_rejected() {
        for b in ErrorBudget::ALL {
            let mut values = vec![1u64; 4096];
            let count = match b {
                ErrorBudget::OnePercent => 257,
                ErrorBudget::TwoPercent => 129,
                ErrorBudget::FivePercent => 65,
                ErrorBudget::TenPercent => 33,
            };
            for i in 0..count {
                values[i * 4096 / count] = 0;
            }
            let result = select_nth(&values, 2048, b).unwrap();
            assert!(result.used_exact_fallback);
            assert_eq!(result.value, 1);
            assert!(2048usize.saturating_sub(count - 1) * 100 > 4096 * b.percent() as usize);
        }
    }
}
