//! Bounded numerical model of the production RSA Game::l0 kernel.
//! See the adjacent source groupant descriptant for domain and error derivation.
use mova_approx::ErrorBudget;

pub const MAX_WORLDS: usize = 64;
pub const MAX_UTTERANCES: usize = 64;
pub const MAX_PRIOR: u64 = 1_000_000;
pub const RELATIVE_ERROR_BOUND: f64 = 1e-12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainError {
    WorldCount,
    UtteranceCount,
    RaggedTruth,
    PriorRange,
}

/// Positive-cell relative error <= 1e-12; reference zero cells stay exactly zero.
/// All profiles share the same tighter implementation. Validation and allocation
/// are part of this call; the production rational API remains unchanged.
pub fn approximate_l0(
    truth: &[Vec<bool>],
    prior: &[u64],
    budget: ErrorBudget,
) -> Result<Vec<Vec<f64>>, DomainError> {
    if prior.is_empty() || prior.len() > MAX_WORLDS {
        return Err(DomainError::WorldCount);
    }
    if truth.len() > MAX_UTTERANCES {
        return Err(DomainError::UtteranceCount);
    }
    if truth.iter().any(|row| row.len() != prior.len()) {
        return Err(DomainError::RaggedTruth);
    }
    if prior.iter().any(|&p| p == 0 || p > MAX_PRIOR) {
        return Err(DomainError::PriorRange);
    }
    debug_assert!(RELATIVE_ERROR_BOUND <= budget.max_fraction());
    Ok(truth.iter().map(|row| {
        let sum: u64 = row.iter().zip(prior).filter_map(|(&t, &p)| t.then_some(p)).sum();
        let reciprocal = if sum == 0 { 0.0 } else { 1.0 / sum as f64 };
        row.iter().zip(prior).map(|(&t, &p)| {
            if t { p as f64 * reciprocal } else { 0.0 }
        }).collect()
    }).collect())
}

#[cfg(test)]
#[path = "../rsa.rs"]
mod production_reference;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boundaries_and_all_profiles() {
        for budget in ErrorBudget::ALL {
            assert_eq!(approximate_l0(&[vec![true]], &[MAX_PRIOR], budget).unwrap(), vec![vec![1.0]]);
            assert_eq!(approximate_l0(&[vec![false]], &[1], budget).unwrap(), vec![vec![0.0]]);
            assert!(approximate_l0(&[], &[1], budget).unwrap().is_empty());
            let a = approximate_l0(&vec![vec![true; 64]; 64], &[MAX_PRIOR; 64], budget).unwrap();
            assert!(a.iter().flatten().all(|&x| x == 1.0 / 64.0));
        }
    }
    fn accepts(reference: &[Vec<super::production_reference::Ratio>], actual: &[Vec<f64>], cap: f64) -> bool {
        reference.len() == actual.len() && reference.iter().zip(actual).all(|(r, a)| {
            r.len() == a.len() && r.iter().zip(a).all(|(r, &x)| {
                let y = r.num as f64 / r.den as f64;
                x.is_finite() && if y == 0.0 { x == 0.0 } else { (x-y).abs()/y <= cap }
            })
        })
    }
    #[test]
    fn production_reference_and_negative_control() {
        for b in ErrorBudget::ALL {
            for seed in 0..64_u64 {
                let mut g = super::production_reference::reference();
                g.prior = vec![1 + seed, MAX_PRIOR - seed, 7919];
                g.truth.push(vec![false; 3]);
                let reference = g.l0();
                let a = approximate_l0(&g.truth, &g.prior, b).unwrap();
                assert!(accepts(&reference, &a, RELATIVE_ERROR_BOUND));
                assert!(accepts(&reference, &a, b.max_fraction()));
                let mut corrupt = a.clone();
                corrupt[0][0] *= 1.5;
                assert!(!accepts(&reference, &corrupt, b.max_fraction()));
                corrupt = a.clone();
                corrupt.last_mut().unwrap()[0] = 0.01;
                assert!(!accepts(&reference, &corrupt, b.max_fraction()));
            }
        }
    }
    #[test]
    fn invalid_domains_are_rejected() {
        for b in ErrorBudget::ALL {
            assert_eq!(approximate_l0(&[], &[], b), Err(DomainError::WorldCount));
            assert_eq!(approximate_l0(&[], &[1; 65], b), Err(DomainError::WorldCount));
            assert_eq!(approximate_l0(&vec![vec![true]; 65], &[1], b), Err(DomainError::UtteranceCount));
            assert_eq!(approximate_l0(&[vec![]], &[1], b), Err(DomainError::RaggedTruth));
            assert_eq!(approximate_l0(&[], &[0], b), Err(DomainError::PriorRange));
            assert_eq!(approximate_l0(&[], &[MAX_PRIOR + 1], b), Err(DomainError::PriorRange));
        }
    }
}
