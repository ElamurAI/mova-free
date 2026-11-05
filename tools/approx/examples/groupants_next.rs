//! Second bounded executable groupant runner: byte edits, LIS length, scalar minima and S1.
use mova_algorithms::{
    levenshtein, longest_increasing_subsequence as lis, sparse_table::SparseTable,
};
use mova_approx::{
    AssuranceKind, ErrorBudget, MethodKind, MetricKind, observed_certificate, relative_error,
};
use std::{hint::black_box, time::Instant};
#[rustfmt::skip]
#[path = "../../../algorithms/src/levenshtein/mod.rs.groupant/rust_approximant.rs"]
mod edit_approx;
#[rustfmt::skip]
#[path = "../../../algorithms/src/longest_increasing_subsequence/mod.rs.groupant/rust_approximant.rs"]
mod lis_approx;
#[rustfmt::skip]
#[path = "../../../algorithms/src/sparse_table/mod.rs.groupant/rust_approximant.rs"]
mod range_approx;
#[rustfmt::skip]
#[path = "../../../prag/src/rsa.rs.groupant/s1_approximant.rs"]
mod s1_approx;
#[allow(dead_code)]
#[rustfmt::skip]
#[path = "../../../prag/src/rsa.rs"]
mod rsa;

fn classification_records(b: ErrorBudget) -> String {
    [
        ("edit", MetricKind::ScalarRelative, MethodKind::IntervalCertificate, AssuranceKind::PerInputCertificate),
        ("lis", MetricKind::ObjectiveLoss, MethodKind::IntervalCertificate, AssuranceKind::PerInputCertificate),
        ("range", MetricKind::ScalarRelative, MethodKind::IntervalCertificate, AssuranceKind::PerInputCertificate),
        ("s1", MetricKind::ProbabilityRelative, MethodKind::FloatingNormalization, AssuranceKind::Analytical),
    ].map(|(name,metric,method,assurance)| format!("CLASSIFICATION budget={} case={} metric_kind={} method_kind={} assurance_kind={} observation_assurance_kind=finite_observation",b.percent(),name,metric.as_str(),method.as_str(),assurance.as_str())).join("\n")
}
fn edit_inputs(fallback: bool) -> (Vec<u8>, Vec<u8>) {
    if fallback {
        return (b"ab".repeat(128), b"ba".repeat(128));
    }
    let mut a = vec![0; 1000];
    let mut c = vec![1; 1000];
    a.extend([2, 3, 4]);
    c.extend([3, 4, 2]);
    (a, c)
}
fn lis_inputs(fallback: bool) -> Vec<u64> {
    if fallback {
        return (0..8192).rev().collect();
    }
    let mut v: Vec<_> = (0..8192).collect();
    v[..3].copy_from_slice(&[2, 0, 1]);
    v
}
fn range_inputs(fallback: bool) -> Vec<u64> {
    (0..8192)
        .map(|i| if fallback { i % 101 } else { 1000 + i % 5 })
        .collect()
}
fn game(sparse: bool) -> rsa::Game {
    rsa::Game {
        worlds: (0..8).map(|i| i.to_string()).collect(),
        utterances: (0..8).map(|i| i.to_string()).collect(),
        truth: (0..8)
            .map(|u| (0..8).map(|w| !sparse || (u + w) % 3 == 0).collect())
            .collect(),
        prior: (1..=8).collect(),
        cost: (0..8).map(|i| i % 5).collect(),
        alpha: 2,
    }
}
fn scalar_gate(reference: u64, actual: u64, b: ErrorBudget) -> Result<f64, String> {
    let e = relative_error(reference as f64, actual as f64).map_err(|e| e.to_string())?;
    observed_certificate(b, &[e]).map_err(|e| e.to_string())?;
    Ok(e)
}
fn lis_gate(v: &[u64], indices: &[usize], exact: usize, b: ErrorBudget) -> Result<f64, String> {
    if indices.iter().any(|&i| i >= v.len())
        || indices
            .windows(2)
            .any(|w| w[0] >= w[1] || v[w[0]] >= v[w[1]])
        || indices.len() > exact
    {
        return Err("infeasible subsequence".into());
    }
    scalar_gate(exact as u64, indices.len() as u64, b)
}
fn s1_gate(
    reference: &[Vec<rsa::Ratio>],
    actual: &[Vec<f64>],
    b: ErrorBudget,
) -> Result<f64, String> {
    if reference.len() != actual.len() {
        return Err("row count".into());
    }
    let mut maximum: f64 = 0.0;
    for (r, a) in reference.iter().zip(actual) {
        if r.len() != a.len() {
            return Err("column count".into());
        }
        for (r, &a) in r.iter().zip(a) {
            if a < 0.0 {
                return Err("negative probability".into());
            }
            maximum = maximum
                .max(relative_error(r.num as f64 / r.den as f64, a).map_err(|e| e.to_string())?);
        }
        let sum: f64 = a.iter().sum();
        if r.iter().any(|r| r.num != 0) && (sum - 1.0).abs() > 1e-12 {
            return Err("normalization".into());
        }
    }
    observed_certificate(b, &[maximum]).map_err(|e| e.to_string())?;
    Ok(maximum)
}
fn check(b: ErrorBudget) -> String {
    let mut lines = Vec::new();
    for fallback in [false, true] {
        let (a, c) = edit_inputs(fallback);
        let exact = levenshtein::distance(&a, &c);
        let e = edit_approx::distance(&a, &c, b);
        assert!(e.lower_bound <= exact && exact <= e.upper_bound);
        assert_eq!(e.used_exact_fallback, fallback);
        let edit_error = scalar_gate(exact as u64, e.distance as u64, b).unwrap();
        let v = lis_inputs(fallback);
        let exact = lis::indices(&v).len();
        let l = lis_approx::indices(&v, b);
        assert!(exact <= l.upper_bound);
        assert_eq!(l.used_exact_fallback, fallback);
        let loss = lis_gate(&v, &l.indices, exact, b).unwrap();
        let v = range_inputs(fallback);
        let table = range_approx::ApproxMin::from_slice(&v, b);
        assert_eq!(table.len(), v.len());
        assert!(!table.is_empty());
        assert_eq!(table.uses_exact_fallback(), fallback);
        let reference = SparseTable::from_slice(&v);
        let mut range_error: f64 = 0.0;
        for (start, end) in [(0, 8192), (1, 2), (57, 803), (8191, 8192)] {
            let expected = *reference.query(start, end).unwrap();
            assert_eq!(expected, *v[start..end].iter().min().unwrap());
            range_error = range_error
                .max(scalar_gate(expected, table.query(start, end).unwrap(), b).unwrap());
        }
        for (start, end) in [(0, 0), (2, 1), (0, 8193)] {
            assert_eq!(table.query(start, end), None);
        }
        let g = game(fallback);
        let actual = s1_approx::approximate_s1(&g.truth, &g.prior, &g.cost, g.alpha, b).unwrap();
        let s1_error = s1_gate(&g.s1(), &actual, b).unwrap();
        let bits = actual
            .iter()
            .flatten()
            .fold(0u64, |s, x| s.wrapping_add(x.to_bits()));
        lines.push(format!("CHECK budget={} fallback={} edit={} edit_error={:.12e} lis_length={} lis_loss={:.12e} range_error={:.12e} s1_bits={} s1_error={:.12e}",b.percent(),fallback,e.distance,edit_error,l.indices.len(),loss,range_error,bits,s1_error));
    }
    lines.join("\n")
}
fn measure(f: &mut dyn FnMut() -> u64, n: usize) -> f64 {
    let t = Instant::now();
    for _ in 0..n {
        black_box(f());
    }
    t.elapsed().as_secs_f64() * 1e6 / n as f64
}
fn timing(
    name: &str,
    b: ErrorBudget,
    n: usize,
    mut reference: impl FnMut() -> u64,
    mut surrogate: impl FnMut() -> u64,
) {
    black_box(reference());
    black_box(surrogate());
    let mut r = Vec::new();
    let mut a = Vec::new();
    for round in 0..7 {
        if round % 2 == 0 {
            r.push(measure(&mut reference, n));
            a.push(measure(&mut surrogate, n));
        } else {
            a.push(measure(&mut surrogate, n));
            r.push(measure(&mut reference, n));
        }
    }
    r.sort_by(f64::total_cmp);
    a.sort_by(f64::total_cmp);
    println!(
        "TIME budget={} case={} ref_us={:.3}[{:.3},{:.3}] approx_us={:.3}[{:.3},{:.3}] ratio={:.3}",
        b.percent(),
        name,
        r[3],
        r[0],
        r[6],
        a[3],
        a[0],
        a[6],
        r[3] / a[3]
    );
}
fn benchmark(b: ErrorBudget) {
    for fallback in [false, true] {
        let tag = if fallback { "fallback" } else { "favorable" };
        let (a, c) = edit_inputs(fallback);
        timing(
            &format!("edit_{tag}"),
            b,
            2,
            || levenshtein::distance(black_box(&a), black_box(&c)) as u64,
            || edit_approx::distance(black_box(&a), black_box(&c), b).distance as u64,
        );
        let v = lis_inputs(fallback);
        timing(
            &format!("lis_{tag}"),
            b,
            8,
            || black_box(lis::indices(black_box(&v))).len() as u64,
            || {
                black_box(lis_approx::indices(black_box(&v), b))
                    .indices
                    .len() as u64
            },
        );
        let v = range_inputs(fallback);
        timing(
            &format!("range_build_{tag}"),
            b,
            8,
            || black_box(SparseTable::from_slice(black_box(&v))).len() as u64,
            || black_box(range_approx::ApproxMin::from_slice(black_box(&v), b)).len() as u64,
        );
        let r = SparseTable::from_slice(&v);
        let a = range_approx::ApproxMin::from_slice(&v, b);
        timing(
            &format!("range_query128_{tag}"),
            b,
            128,
            || {
                (0..128)
                    .map(|i| *r.query(black_box(i), black_box(i + 64)).unwrap())
                    .sum()
            },
            || {
                (0..128)
                    .map(|i| a.query(black_box(i), black_box(i + 64)).unwrap())
                    .sum()
            },
        );
        let g = game(fallback);
        // S1 has no exact dispatch: the second case exercises sparse truth and zeros.
        let name = if fallback { "s1_sparse" } else { "s1_dense" };
        timing(
            name,
            b,
            4,
            || {
                black_box(&g)
                    .s1()
                    .iter()
                    .flatten()
                    .fold(0u64, |s, r| s.wrapping_add(r.num as u64))
            },
            || {
                s1_approx::approximate_s1(
                    black_box(&g.truth),
                    black_box(&g.prior),
                    black_box(&g.cost),
                    black_box(g.alpha),
                    b,
                )
                .unwrap()
                .iter()
                .flatten()
                .fold(0u64, |s, x| s.wrapping_add(x.to_bits()))
            },
        );
    }
}
fn main() {
    for b in ErrorBudget::ALL {
        println!("{}", classification_records(b));
        println!("{}", check(b));
        benchmark(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_checks_and_classification() {
        for b in ErrorBudget::ALL {
            assert_eq!(check(b), check(b));
            let records = classification_records(b);
            assert_eq!(records.lines().count(), 4);
            assert_eq!(
                records.matches("method_kind=interval_certificate").count(),
                3
            );
            assert!(records.contains("case=s1 metric_kind=probability_relative method_kind=floating_normalization assurance_kind=analytical"));
        }
    }
    #[test]
    fn corruption_controls_reject_bad_outputs() {
        for b in ErrorBudget::ALL {
            assert!(scalar_gate(0, 1, b).is_err());
            assert!(scalar_gate(100, 125, b).is_err());
            assert!(lis_gate(&[1, 2, 3], &[0, 0, 2], 3, b).is_err());
            assert!(lis_gate(&[1, 2, 3], &[0], 3, b).is_err());
            let g = game(true);
            let original =
                s1_approx::approximate_s1(&g.truth, &g.prior, &g.cost, g.alpha, b).unwrap();
            let mut corrupt = original.clone();
            for row in &mut corrupt {
                for x in row {
                    *x *= 0.5;
                }
            }
            assert!(s1_gate(&g.s1(), &corrupt, b).is_err());
            let mut corrupt = original;
            corrupt[0][0] = f64::NAN;
            assert!(s1_gate(&g.s1(), &corrupt, b).is_err());
        }
    }
    #[test]
    fn independent_small_oracles() {
        for b in ErrorBudget::ALL {
            for v in [vec![], vec![2, 0, 1, 3], vec![3, 2, 1], vec![1, 1, 1]] {
                let optimum = (0..1usize << v.len())
                    .map(|mask| {
                        let subseq: Vec<_> =
                            (0..v.len()).filter(|i| mask & (1 << i) != 0).collect();
                        if subseq.windows(2).all(|w| v[w[0]] < v[w[1]]) {
                            subseq.len()
                        } else {
                            0
                        }
                    })
                    .max()
                    .unwrap();
                assert_eq!(lis::indices(&v).len(), optimum);
                lis_gate(&v, &lis_approx::indices(&v, b).indices, optimum, b).unwrap();
            }
            // Byte semantics: the two-byte UTF-8 sequence is not one character.
            let a = "é".as_bytes();
            let c = b"e";
            assert_eq!(levenshtein::distance(a, c), 2);
            assert_eq!(edit_approx::distance(a, c, b).distance, 2);
            let g = rsa::Game {
                worlds: vec!["a".into(), "b".into()],
                utterances: vec!["x".into(), "y".into()],
                truth: vec![vec![true, true], vec![true, false]],
                prior: vec![1, 1],
                cost: vec![0, 0],
                alpha: 1,
            };
            let actual =
                s1_approx::approximate_s1(&g.truth, &g.prior, &g.cost, g.alpha, b).unwrap();
            assert!((actual[0][0] - 1.0 / 3.0).abs() < 1e-14);
            assert!((actual[0][1] - 2.0 / 3.0).abs() < 1e-14);
            assert_eq!(actual[1], vec![1.0, 0.0]);
        }
    }
}
