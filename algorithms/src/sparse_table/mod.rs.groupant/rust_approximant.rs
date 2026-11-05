//! Numerical range-minimum surrogate with an input-wide interval certificate.
//! For global extrema m,M and each valid range minimum R, m<=R<=M.
//! Accept (M-m)*100 <= p*M: then (R-m)/R <= (M-m)/M <= p/100.
//! Zero is exact. Returned values need not occur inside the queried range.
//! Prediction: compact O(n) build / O(1) storage beats table construction on
//! narrow inputs; fallback adds a scan to the original O(n log(n+1)) build.
use mova_algorithms::sparse_table::SparseTable;
use mova_approx::ErrorBudget;

#[derive(Clone, Debug)]
pub struct ApproxMin {
    len: usize,
    minimum: u64,
    exact: Option<SparseTable<u64>>,
}

fn certified(minimum: u64, maximum: u64, budget: ErrorBudget) -> bool {
    minimum <= maximum
        && u128::from(maximum - minimum) * 100 <= u128::from(maximum) * u128::from(budget.percent())
}

impl ApproxMin {
    /// All finite u64 slices; allocation behavior follows the reference on fallback.
    /// The compact representation does not retain input values or reference identity.
    pub fn from_slice(values: &[u64], budget: ErrorBudget) -> Self {
        let (mut minimum, mut maximum) = (u64::MAX, 0);
        for &value in values {
            minimum = minimum.min(value);
            maximum = maximum.max(value);
        }
        if values.is_empty() {
            minimum = 0;
        }
        let exact = if certified(minimum, maximum, budget) {
            None
        } else {
            Some(SparseTable::from_slice(values))
        };
        Self {
            len: values.len(),
            minimum,
            exact,
        }
    }

    /// Minimum value estimate for nonempty [start,end); invalid ranges return None.
    /// This value model does not provide a leftmost index or an in-range witness.
    pub fn query(&self, start: usize, end: usize) -> Option<u64> {
        if start >= end || end > self.len {
            return None;
        }
        match &self.exact {
            Some(table) => table.query(start, end).copied(),
            None => Some(self.minimum),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn uses_exact_fallback(&self) -> bool {
        self.exact.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(values: &[u64]) {
        let exact = SparseTable::from_slice(values);
        for budget in ErrorBudget::ALL {
            let approx = ApproxMin::from_slice(values, budget);
            assert_eq!(approx.len(), values.len());
            assert_eq!(approx.is_empty(), values.is_empty());
            for start in 0..=values.len() + 1 {
                for end in 0..=values.len() + 1 {
                    let oracle = if start < end && end <= values.len() {
                        values[start..end].iter().copied().min()
                    } else {
                        None
                    };
                    assert_eq!(exact.query(start, end).copied(), oracle);
                    let result = approx.query(start, end);
                    assert_eq!(result.is_some(), oracle.is_some());
                    if let (Some(actual), Some(reference)) = (result, oracle) {
                        assert!(actual <= reference);
                        assert!(
                            u128::from(reference - actual) * 100
                                <= u128::from(reference) * u128::from(budget.percent())
                        );
                        if approx.uses_exact_fallback() {
                            assert_eq!(actual, reference);
                        }
                    }
                }
            }
            assert_eq!(approx.query(0, usize::MAX), None);
            assert_eq!(approx.query(usize::MAX, 0), None);
        }
    }

    #[test]
    fn exhaustive_small_range_oracle() {
        let alphabet = [0, 1, 90, 95, 98, 99, 100, u64::MAX];
        for n in 0..=4 {
            for mut code in 0..alphabet.len().pow(n) {
                let mut values = Vec::new();
                for _ in 0..n {
                    values.push(alphabet[code % alphabet.len()]);
                    code /= alphabet.len();
                }
                check(&values);
            }
        }
    }

    #[test]
    fn extreme_zero_and_invalid_ranges() {
        for values in [
            vec![],
            vec![0; 8],
            vec![u64::MAX; 8],
            vec![0, u64::MAX],
            vec![u64::MAX - 1, u64::MAX],
        ] {
            check(&values);
        }
    }

    #[test]
    fn accepted_nonexact_and_threshold_monotonicity() {
        for budget in ErrorBudget::ALL {
            let lower = 100 - u64::from(budget.percent());
            let approx = ApproxMin::from_slice(&[lower, 100], budget);
            assert!(!approx.uses_exact_fallback());
            assert_eq!(approx.query(1, 2), Some(lower));
            assert_ne!(approx.query(1, 2), Some(100));
            let rejected = ApproxMin::from_slice(&[lower - 1, 100], budget);
            assert!(rejected.uses_exact_fallback());
            assert_eq!(rejected.query(1, 2), Some(100));
        }
        for minimum in 0..=100 {
            let mut accepted = false;
            for budget in ErrorBudget::ALL {
                let current = !ApproxMin::from_slice(&[minimum, 100], budget).uses_exact_fallback();
                assert!(!accepted || current);
                accepted = current;
            }
        }
    }

    #[test]
    fn integer_certificate_extremes() {
        for budget in ErrorBudget::ALL {
            let allowed = (u128::from(u64::MAX) * u128::from(budget.percent()) / 100) as u64;
            assert!(certified(u64::MAX - allowed, u64::MAX, budget));
            assert!(!certified(u64::MAX - allowed - 1, u64::MAX, budget));
            assert!(!certified(1, 0, budget));
            assert!(certified(0, 0, budget));
            assert!(!certified(0, 1, budget));
        }
    }
}
