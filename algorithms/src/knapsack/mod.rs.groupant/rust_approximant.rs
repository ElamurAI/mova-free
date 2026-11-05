//! Certified density-greedy knapsack, with exact fallback.
//! Model: L is a feasible integral value, OPT <= U is a rounded-up fractional
//! relaxation. Accept only U-L <= floor(p*U/100), implying loss/OPT <= p/100.
//! Prediction: O(n log n) accepted path beats O(n*C) at large capacity;
//! uncertified instances add sorting overhead before the exact implementation.
use mova_algorithms::knapsack::{self, Error, Item};
use mova_approx::ErrorBudget;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Estimate {
    pub value: u128,
    pub upper_bound: u128,
    pub used_exact_fallback: bool,
}

/// Integer certificate, avoiding overflow even for arbitrary u128 bounds.
pub fn certified(lower: u128, upper: u128, budget: ErrorBudget) -> bool {
    if lower > upper {
        return false;
    }
    let p = u128::from(budget.percent());
    upper - lower <= (upper / 100) * p + ((upper % 100) * p) / 100
}

/// Values are attainable subset objectives. No approximate capacity arithmetic.
/// Supports the same usize<=64 target domain as the exact reference.
/// Unlike the exact row solver, a certified result needs no capacity-sized row,
/// so large capacities can succeed even when the exact reference rejects them.
pub fn max_value(items: &[Item], capacity: usize, budget: ErrorBudget) -> Result<Estimate, Error> {
    let mut ranked = Vec::new();
    ranked
        .try_reserve_exact(items.len())
        .map_err(|_| Error::AllocationFailed)?;
    let mut free = 0u128;
    for &item in items {
        if item.weight == 0 {
            free += u128::from(item.value);
        } else if item.weight <= capacity && item.value != 0 {
            ranked.push(item);
        }
    }
    // Cross products fit u128 for usize<=64, u64 values. Tie-breaks are total
    // and deterministic; unstable sort requires no further heap allocation.
    ranked.sort_unstable_by(|a, b| {
        (u128::from(b.value) * a.weight as u128)
            .cmp(&(u128::from(a.value) * b.weight as u128))
            .then_with(|| a.weight.cmp(&b.weight))
            .then_with(|| a.value.cmp(&b.value))
    });
    let best_single = ranked
        .iter()
        .map(|item| u128::from(item.value))
        .max()
        .unwrap_or(0)
        + free;
    let mut remaining = capacity;
    let mut lower = free;
    for item in &ranked {
        if item.weight <= remaining {
            lower += u128::from(item.value);
            remaining -= item.weight;
        }
    }
    lower = lower.max(best_single);
    // Fractional relaxation stops at its first split item (unlike the lower
    // bound, which may skip an item and continue with smaller items).
    let mut remaining = capacity;
    let mut upper = free;
    for item in &ranked {
        if item.weight <= remaining {
            upper += u128::from(item.value);
            remaining -= item.weight;
        } else {
            let numerator = u128::from(item.value) * remaining as u128;
            let denominator = item.weight as u128;
            upper += numerator / denominator + u128::from(!numerator.is_multiple_of(denominator));
            break;
        }
    }
    if certified(lower, upper, budget) {
        return Ok(Estimate {
            value: lower,
            upper_bound: upper,
            used_exact_fallback: false,
        });
    }
    let value = knapsack::max_value(items, capacity)?;
    Ok(Estimate {
        value,
        upper_bound: value,
        used_exact_fallback: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const BUDGETS: [ErrorBudget; 4] = ErrorBudget::ALL;
    fn brute(items: &[Item], capacity: usize) -> u128 {
        (0..(1usize << items.len()))
            .filter_map(|mask| {
                let mut w = 0u128;
                let mut v = 0u128;
                for (i, item) in items.iter().enumerate() {
                    if mask & (1 << i) != 0 {
                        w += item.weight as u128;
                        v += u128::from(item.value);
                    }
                }
                (w <= capacity as u128).then_some(v)
            })
            .max()
            .unwrap()
    }
    #[test]
    fn differential_all_budgets() {
        let mut seed = 193u64;
        for n in 0..=10 {
            for _ in 0..100 {
                let items: Vec<_> = (0..n)
                    .map(|_| {
                        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        Item {
                            weight: (seed % 17) as usize,
                            value: (seed >> 32) % 101,
                        }
                    })
                    .collect();
                let capacity = (seed % 51) as usize;
                let exact = brute(&items, capacity);
                assert_eq!(knapsack::max_value(&items, capacity).unwrap(), exact);
                for budget in BUDGETS {
                    let result = max_value(&items, capacity, budget).unwrap();
                    assert!(result.value <= exact && exact <= result.upper_bound);
                    assert!((exact - result.value) * 100 <= exact * u128::from(budget.percent()));
                    assert_eq!(result, max_value(&items, capacity, budget).unwrap());
                }
            }
        }
    }
    #[test]
    fn budgets_change_certified_dispatch() {
        let items = [
            Item {
                weight: 6,
                value: 60,
            },
            Item {
                weight: 5,
                value: 5,
            },
        ];
        for budget in BUDGETS {
            let result = max_value(&items, 10, budget).unwrap();
            assert_eq!(result.value, 60);
            assert_eq!(result.used_exact_fallback, budget.percent() < 10);
        }
        let items = [
            Item {
                weight: 6,
                value: 60,
            },
            Item {
                weight: 5,
                value: 49,
            },
            Item {
                weight: 5,
                value: 49,
            },
        ];
        for budget in BUDGETS {
            let result = max_value(&items, 10, budget).unwrap();
            assert!(result.used_exact_fallback);
            assert_eq!(result.value, 98);
        }
    }
    #[test]
    fn nonexact_certified_value() {
        let items = [
            Item {
                weight: 0,
                value: 500,
            },
            Item {
                weight: 6,
                value: 100,
            },
            Item {
                weight: 5,
                value: 65,
            },
            Item {
                weight: 5,
                value: 65,
            },
        ];
        for budget in BUDGETS {
            let result = max_value(&items, 10, budget).unwrap();
            assert_eq!(brute(&items, 10), 630);
            assert_eq!(result.value, if budget.percent() < 10 { 630 } else { 600 });
            assert_eq!(result.used_exact_fallback, budget.percent() < 10);
            assert_eq!(
                result.upper_bound,
                if budget.percent() < 10 { 630 } else { 652 }
            );
            assert!(certified(result.value, result.upper_bound, budget));
            assert!((630 - result.value) * 100 <= 630 * u128::from(budget.percent()));
        }
    }
    #[test]
    fn certificate_thresholds_and_unsupported_budgets() {
        for budget in BUDGETS {
            for upper in 0..=1000u128 {
                for lower in 0..=upper {
                    assert_eq!(
                        certified(lower, upper, budget),
                        (upper - lower) * 100 <= upper * u128::from(budget.percent())
                    );
                }
            }
            let allowed = (u128::MAX / 100) * u128::from(budget.percent())
                + (u128::MAX % 100) * u128::from(budget.percent()) / 100;
            assert!(certified(u128::MAX - allowed, u128::MAX, budget));
            assert!(!certified(u128::MAX - allowed - 1, u128::MAX, budget));
        }
        for unsupported in [11, 15, 25, 100, u32::MAX] {
            assert!(ErrorBudget::from_percent(unsupported).is_err());
        }
        for unsupported in [0.1000001, 0.15, 0.25, 1.0, f64::INFINITY, f64::NAN] {
            assert!(ErrorBudget::from_fraction(unsupported).is_err());
        }
    }

    #[test]
    fn zeros_boundaries_and_negative_control() {
        for budget in BUDGETS {
            assert_eq!(max_value(&[], 0, budget).unwrap().value, 0);
            let free = [Item {
                weight: 0,
                value: u64::MAX,
            }; 2];
            assert_eq!(
                max_value(&free, 0, budget).unwrap().value,
                2 * u128::from(u64::MAX)
            );
            assert_eq!(
                max_value(&free, usize::MAX, budget).unwrap().value,
                2 * u128::from(u64::MAX)
            );
            assert!(!certified(0, 1, budget)); // A zero-output mutant fails every budget.
            assert!(!certified(2, 1, budget));
            assert!(certified(u128::MAX, u128::MAX, budget));
            assert!(!certified(0, u128::MAX, budget));
            let huge = [
                Item {
                    weight: usize::MAX,
                    value: u64::MAX,
                },
                Item {
                    weight: 1,
                    value: 1,
                },
            ];
            let result = max_value(&huge, usize::MAX, budget).unwrap();
            assert!(certified(result.value, u128::from(u64::MAX), budget));
        }
    }
}
