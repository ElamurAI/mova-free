//! Byte edit-distance interval certificate with exact production fallback.
//! Prospective cost: O(n+m+256) scans, constant histogram storage; fallback
//! adds O(n'*m') time and O(min(n',m')) storage for trimmed middle lengths.
use mova_approx::ErrorBudget;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Estimate {
    pub distance: usize,
    pub lower_bound: usize,
    pub upper_bound: usize,
    pub used_exact_fallback: bool,
}

/// Overflow-safe comparison (upper-lower)/lower <= p/100, zero exact.
fn certified(lower: usize, upper: usize, budget: ErrorBudget) -> bool {
    if lower > upper {
        return false;
    }
    let p = budget.percent() as usize;
    upper - lower <= (lower / 100) * p + ((lower % 100) * p) / 100
}

fn trim<'a>(mut a: &'a [u8], mut b: &'a [u8]) -> (&'a [u8], &'a [u8]) {
    let prefix = a.iter().zip(b).take_while(|(x, y)| x == y).count();
    a = &a[prefix..];
    b = &b[prefix..];
    let suffix = a
        .iter()
        .rev()
        .zip(b.iter().rev())
        .take_while(|(x, y)| x == y)
        .count();
    (&a[..a.len() - suffix], &b[..b.len() - suffix])
}

/// Overestimates unit-cost byte Levenshtein distance by at most the budget.
/// Byte semantics only; allocation failure in exact fallback follows Vec.
/// Bounds enclose the true distance; fallback collapses them to the exact value.
pub fn distance(a: &[u8], b: &[u8], budget: ErrorBudget) -> Estimate {
    let (a, b) = trim(a, b);
    let length_gap = a.len().abs_diff(b.len());
    let upper = length_gap + a.iter().zip(b).filter(|(x, y)| x != y).count();
    let mut counts_a = [0usize; 256];
    let mut counts_b = [0usize; 256];
    for &byte in a {
        counts_a[byte as usize] += 1;
    }
    for &byte in b {
        counts_b[byte as usize] += 1;
    }
    let mut excess_a = 0usize;
    let mut excess_b = 0usize;
    for (left, right) in counts_a.into_iter().zip(counts_b) {
        excess_a += left.saturating_sub(right);
        excess_b += right.saturating_sub(left);
    }
    let lower = length_gap.max(excess_a).max(excess_b);
    if certified(lower, upper, budget) {
        Estimate {
            distance: upper,
            lower_bound: lower,
            upper_bound: upper,
            used_exact_fallback: false,
        }
    } else {
        let exact = mova_algorithms::levenshtein::distance(a, b);
        Estimate {
            distance: exact,
            lower_bound: exact,
            upper_bound: exact,
            used_exact_fallback: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Independent full matrix oracle, no trimming or production row recurrence reuse.
    fn reference(a: &[u8], b: &[u8]) -> usize {
        let mut d = vec![vec![0usize; b.len() + 1]; a.len() + 1];
        for (i, row) in d.iter_mut().enumerate() {
            row[0] = i;
        }
        for j in 0..=b.len() {
            d[0][j] = j;
        }
        for i in 1..=a.len() {
            for j in 1..=b.len() {
                d[i][j] = [
                    d[i - 1][j] + 1,
                    d[i][j - 1] + 1,
                    d[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]),
                ]
                .into_iter()
                .min()
                .unwrap();
            }
        }
        d[a.len()][b.len()]
    }
    fn check(a: &[u8], b: &[u8]) {
        let exact = reference(a, b);
        assert_eq!(mova_algorithms::levenshtein::distance(a, b), exact);
        let (ta, tb) = trim(a, b);
        assert_eq!(reference(ta, tb), exact);
        for budget in ErrorBudget::ALL {
            let e = distance(a, b, budget);
            assert!(e.lower_bound <= exact && exact <= e.upper_bound);
            assert!(e.distance >= exact);
            assert!(
                (e.distance - exact) as u128 * 100 <= exact as u128 * u128::from(budget.percent())
            );
            assert_eq!(e, distance(b, a, budget));
            assert_eq!(e, distance(a, b, budget));
            if exact == 0 {
                assert_eq!(e.distance, 0);
            }
            if e.used_exact_fallback {
                assert_eq!(
                    (e.lower_bound, e.upper_bound, e.distance),
                    (exact, exact, exact)
                );
            }
        }
    }
    #[test]
    fn exhaustive_binary_strings_and_trimming() {
        let mut words = vec![vec![]];
        for length in 1..=6 {
            for bits in 0..(1usize << length) {
                words.push((0..length).map(|i| ((bits >> i) & 1) as u8).collect());
            }
        }
        for a in &words {
            for b in &words {
                check(a, b);
            }
        }
    }
    #[test]
    fn zero_full_byte_range_and_adversarial() {
        let all: Vec<u8> = (0..=255).collect();
        check(&all, &all);
        check(&all, &all.iter().rev().copied().collect::<Vec<_>>());
        check(&[], &all);
        for (a, b) in [
            (b"kitten".as_slice(), b"sitting".as_slice()),
            (b"abababababab", b"babababababa"),
            (b"prefix-abc-suffix", b"prefix-bca-suffix"),
            (b"aaa", b"aaaa"),
            (b"ab", b"ba"),
        ] {
            check(a, b);
        }
        for budget in ErrorBudget::ALL {
            assert!(distance(b"ab", b"ba", budget).used_exact_fallback);
            assert!(!distance(&[], &all, budget).used_exact_fallback);
        }
    }
    #[test]
    fn nonexact_certification_and_budget_dispatch() {
        for n in [100, 300] {
            let mut a = vec![0; n];
            a.extend([2, 3, 4]);
            let mut b = vec![1; n];
            b.extend([3, 4, 2]);
            assert_eq!(reference(&a, &b), n + 2);
            check(&a, &b);
            for budget in ErrorBudget::ALL {
                let e = distance(&a, &b, budget);
                let accepted = n * budget.percent() as usize >= 300;
                assert_eq!(e.used_exact_fallback, !accepted);
                assert_eq!(e.distance, n + if accepted { 3 } else { 2 });
            }
        }
    }
    #[test]
    fn certificate_boundaries_and_integer_extremes() {
        for budget in ErrorBudget::ALL {
            let p = budget.percent() as usize;
            assert!(certified(0, 0, budget));
            assert!(!certified(0, 1, budget));
            assert!(!certified(2, 1, budget));
            for lower in 0..=500usize {
                for gap in 0..=60 {
                    assert_eq!(
                        certified(lower, lower + gap, budget),
                        gap * 100 <= lower * p
                    );
                }
            }
            let lower = usize::MAX / 2;
            let allowed = (lower / 100) * p + (lower % 100) * p / 100;
            assert!(certified(lower, lower + allowed, budget));
            assert!(!certified(lower, lower + allowed + 1, budget));
            assert!(certified(usize::MAX, usize::MAX, budget));
        }
    }
}
