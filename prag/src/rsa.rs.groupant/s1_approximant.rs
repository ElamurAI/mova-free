//! Bounded binary64 surrogate for the actual Game::s1 rational kernel.
//! Input truth is [utterance][world]; output is [world][utterance].
use mova_approx::ErrorBudget;

pub const MAX_WORLDS: usize = 8;
pub const MAX_UTTERANCES: usize = 8;
pub const MAX_PRIOR: u64 = 8;
pub const MAX_COST: u32 = 4;
pub const RELATIVE_ERROR_BOUND: f64 = 1e-12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainError {
    WorldCount,
    UtteranceCount,
    RaggedTruth,
    PriorRange,
    CostCount,
    CostRange,
    AlphaRange,
}

/// All four budgets share the tighter analytical positive-cell relative cap.
/// Exact zero support stays zero. Domain validation and allocation are included.
pub fn approximate_s1(
    truth: &[Vec<bool>],
    prior: &[u64],
    cost: &[u32],
    alpha: u32,
    budget: ErrorBudget,
) -> Result<Vec<Vec<f64>>, DomainError> {
    if prior.is_empty() || prior.len() > MAX_WORLDS {
        return Err(DomainError::WorldCount);
    }
    if truth.is_empty() || truth.len() > MAX_UTTERANCES {
        return Err(DomainError::UtteranceCount);
    }
    if truth.iter().any(|row| row.len() != prior.len()) {
        return Err(DomainError::RaggedTruth);
    }
    if prior.iter().any(|&p| p == 0 || p > MAX_PRIOR) {
        return Err(DomainError::PriorRange);
    }
    if cost.len() != truth.len() {
        return Err(DomainError::CostCount);
    }
    if cost.iter().any(|&c| c > MAX_COST) {
        return Err(DomainError::CostRange);
    }
    if !(1..=2).contains(&alpha) {
        return Err(DomainError::AlphaRange);
    }
    debug_assert!(RELATIVE_ERROR_BOUND <= budget.max_fraction());
    let sums: Vec<u64> = truth
        .iter()
        .map(|row| {
            row.iter()
                .zip(prior)
                .filter_map(|(&t, &p)| t.then_some(p))
                .sum()
        })
        .collect();
    Ok(prior
        .iter()
        .enumerate()
        .map(|(w, &p)| {
            let mut row: Vec<f64> = truth
                .iter()
                .enumerate()
                .map(|(u, t)| {
                    if !t[w] {
                        return 0.0;
                    }
                    let l0 = p as f64 / sums[u] as f64;
                    let powered = if alpha == 2 { l0 * l0 } else { l0 };
                    powered / (1_u64 << (alpha * cost[u])) as f64
                })
                .collect();
            let sum: f64 = row.iter().sum();
            if sum != 0.0 {
                for x in &mut row {
                    *x /= sum;
                }
            }
            row
        })
        .collect())
}

#[cfg(test)]
#[path = "../rsa.rs"]
mod production_reference;

#[cfg(test)]
mod tests {
    use super::production_reference::{Game, Ratio};
    use super::*;

    fn accepts(reference: &[Vec<Ratio>], actual: &[Vec<f64>], cap: f64) -> bool {
        reference.len() == actual.len()
            && reference.iter().zip(actual).all(|(r, a)| {
                r.len() == a.len()
                    && r.iter().zip(a).all(|(r, &x)| {
                        let y = r.num as f64 / r.den as f64;
                        x.is_finite()
                            && x >= 0.0
                            && if r.num == 0 {
                                x == 0.0
                            } else {
                                (x - y).abs() / y <= cap
                            }
                    })
            })
    }

    fn game(truth: Vec<Vec<bool>>, prior: Vec<u64>, cost: Vec<u32>, alpha: u32) -> Game {
        Game {
            worlds: (0..prior.len()).map(|i| i.to_string()).collect(),
            utterances: (0..truth.len()).map(|i| i.to_string()).collect(),
            truth,
            prior,
            cost,
            alpha,
        }
    }

    fn check(g: &Game) {
        let exact = g.s1();
        for b in ErrorBudget::ALL {
            let actual = approximate_s1(&g.truth, &g.prior, &g.cost, g.alpha, b).unwrap();
            assert!(accepts(&exact, &actual, RELATIVE_ERROR_BOUND));
            assert!(accepts(&exact, &actual, b.max_fraction()));
            for row in &actual {
                let sum: f64 = row.iter().sum();
                assert!(sum == 0.0 || (sum - 1.0).abs() <= RELATIVE_ERROR_BOUND);
            }
        }
    }

    #[test]
    fn exhaustive_small_truth_tables_against_actual_s1() {
        for mask in 0..512 {
            let truth: Vec<Vec<bool>> = (0..3)
                .map(|u| (0..3).map(|w| mask & (1 << (3 * u + w)) != 0).collect())
                .collect();
            for alpha in 1..=2 {
                check(&game(truth.clone(), vec![1, 3, 8], vec![0, 2, 4], alpha));
            }
        }
    }

    #[test]
    fn bounded_shapes_extrema_and_mixed_priors() {
        for w in 1..=8 {
            for u in 1..=8 {
                for seed in 0..16 {
                    let truth = (0..u)
                        .map(|i| {
                            (0..w)
                                .map(|j| (i * 13 + j * 7 + seed * 3) % 11 < seed % 12)
                                .collect()
                        })
                        .collect();
                    let prior = (0..w).map(|j| 1 + ((j * 3 + seed) % 8) as u64).collect();
                    let cost = (0..u).map(|i| ((i + seed) % 5) as u32).collect();
                    check(&game(truth, prior, cost, 1 + (seed % 2) as u32));
                }
            }
        }
        for alpha in 1..=2 {
            check(&game(vec![vec![true; 8]; 8], vec![8; 8], vec![4; 8], alpha));
            check(&game(
                vec![vec![false; 8]; 8],
                vec![1; 8],
                vec![0; 8],
                alpha,
            ));
            let mut truth = vec![vec![true; 8]; 8];
            truth[0] = vec![false; 8];
            for row in &mut truth {
                row[7] = false;
            }
            check(&game(
                truth,
                vec![1, 8, 8, 8, 8, 8, 8, 8],
                vec![0, 4, 0, 4, 0, 4, 0, 4],
                alpha,
            ));
        }
    }

    #[test]
    fn corruptions_are_rejected_for_every_profile() {
        let g = game(
            vec![vec![true, false], vec![true, true]],
            vec![1, 8],
            vec![0, 4],
            2,
        );
        let exact = g.s1();
        for b in ErrorBudget::ALL {
            let a = approximate_s1(&g.truth, &g.prior, &g.cost, g.alpha, b).unwrap();
            for bad in [f64::NAN, f64::INFINITY, -1.0, a[0][0] * 1.5] {
                let mut corrupt = a.clone();
                corrupt[0][0] = bad;
                assert!(!accepts(&exact, &corrupt, b.max_fraction()));
            }
            let mut corrupt = a.clone();
            corrupt[1][0] = 0.001;
            assert!(!accepts(&exact, &corrupt, b.max_fraction()));
            assert!(!accepts(&exact, &a[..1], b.max_fraction()));
        }
    }

    #[test]
    fn invalid_domains_are_typed() {
        for b in ErrorBudget::ALL {
            assert_eq!(
                approximate_s1(&[], &[], &[], 1, b),
                Err(DomainError::WorldCount)
            );
            assert_eq!(
                approximate_s1(&[vec![true; 9]], &[1; 9], &[0], 1, b),
                Err(DomainError::WorldCount)
            );
            assert_eq!(
                approximate_s1(&[], &[1], &[], 1, b),
                Err(DomainError::UtteranceCount)
            );
            assert_eq!(
                approximate_s1(&vec![vec![true]; 9], &[1], &[0; 9], 1, b),
                Err(DomainError::UtteranceCount)
            );
            assert_eq!(
                approximate_s1(&[vec![]], &[1], &[0], 1, b),
                Err(DomainError::RaggedTruth)
            );
            for p in [0, 9] {
                assert_eq!(
                    approximate_s1(&[vec![true]], &[p], &[0], 1, b),
                    Err(DomainError::PriorRange)
                );
            }
            assert_eq!(
                approximate_s1(&[vec![true]], &[1], &[], 1, b),
                Err(DomainError::CostCount)
            );
            assert_eq!(
                approximate_s1(&[vec![true]], &[1], &[5], 1, b),
                Err(DomainError::CostRange)
            );
            for alpha in [0, 3, u32::MAX] {
                assert_eq!(
                    approximate_s1(&[vec![true]], &[1], &[0], alpha, b),
                    Err(DomainError::AlphaRange)
                );
            }
        }
    }
}
