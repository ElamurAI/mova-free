//! Executable groupant checks and bounded CPU benchmarks.
use mova_algorithms::{integer_sqrt, knapsack, quickselect};
use mova_approx::{
    AssuranceKind, ErrorBudget, MethodKind, MetricKind, observed_certificate, relative_error,
};

fn classifications() -> [(&'static str, MetricKind, MethodKind, AssuranceKind); 4] {
    [
        (
            "sqrt",
            MetricKind::ScalarRelative,
            MethodKind::QuantizedNumeric,
            AssuranceKind::Analytical,
        ),
        (
            "rank",
            MetricKind::NormalizedRank,
            MethodKind::CertifiedSample,
            AssuranceKind::PerInputCertificate,
        ),
        (
            "knapsack",
            MetricKind::ObjectiveLoss,
            MethodKind::RelaxationCertificate,
            AssuranceKind::PerInputCertificate,
        ),
        (
            "rsa",
            MetricKind::ProbabilityRelative,
            MethodKind::FloatingNormalization,
            AssuranceKind::Analytical,
        ),
    ]
}
fn classification_records(b: ErrorBudget) -> String {
    classifications().map(|(case, metric, method, assurance)| format!(
        "CLASSIFICATION budget={} case={} metric_kind={} method_kind={} assurance_kind={} observation_assurance_kind={}",
        b.percent(), case, metric.as_str(), method.as_str(), assurance.as_str(),
        AssuranceKind::FiniteObservation.as_str()
    )).join("\n")
}
use std::{hint::black_box, time::Instant};
#[rustfmt::skip]
#[path = "../../../algorithms/src/integer_sqrt/mod.rs.groupant/rust_approximant.rs"]
mod sqrt_approx;
#[rustfmt::skip]
#[path = "../../../algorithms/src/quickselect/mod.rs.groupant/rust_approximant.rs"]
mod rank_approx;
#[rustfmt::skip]
#[path = "../../../algorithms/src/knapsack/mod.rs.groupant/rust_approximant.rs"]
mod knapsack_approx;
#[rustfmt::skip]
#[path = "../../../prag/src/rsa.rs.groupant/rust_approximant.rs"]
mod rsa_approx;
#[allow(dead_code)]
#[rustfmt::skip]
#[path = "../../../prag/src/rsa.rs"]
mod rsa;
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn values() -> Vec<u64> {
    let mut seed = 9187;
    (0..8192).map(|_| next(&mut seed)).collect()
}
fn items() -> Vec<knapsack::Item> {
    let mut seed = 7727;
    (0..96)
        .map(|_| knapsack::Item {
            weight: (next(&mut seed) % 100 + 1) as usize,
            value: next(&mut seed) % 1000 + 1,
        })
        .collect()
}
fn game() -> rsa::Game {
    let mut seed = 1459;
    rsa::Game {
        worlds: (0..32).map(|i| i.to_string()).collect(),
        utterances: (0..32).map(|i| i.to_string()).collect(),
        truth: (0..32)
            .map(|_| {
                (0..32)
                    .map(|_| !next(&mut seed).is_multiple_of(3))
                    .collect()
            })
            .collect(),
        prior: (0..32).map(|_| next(&mut seed) % 1000 + 1).collect(),
        cost: vec![0; 32],
        alpha: 1,
    }
}
fn rank_distance(sorted: &[u64], value: u64, index: usize) -> usize {
    let lo = sorted.partition_point(|&v| v < value);
    let hi = sorted.partition_point(|&v| v <= value);
    assert!(hi > lo, "surrogate value must occur in input");
    if index < lo {
        lo - index
    } else {
        index.saturating_sub(hi - 1)
    }
}
fn rsa_error(reference: &[Vec<rsa::Ratio>], actual: &[Vec<f64>]) -> Result<f64, String> {
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
    }
    Ok(maximum)
}
fn fallback_items() -> [knapsack::Item; 3] {
    [(6, 6), (5, 4), (5, 4)].map(|(weight, value)| knapsack::Item { weight, value })
}
fn fallback_ranks(b: ErrorBudget) -> Vec<u64> {
    let size = match b {
        ErrorBudget::OnePercent => 257,
        ErrorBudget::TwoPercent => 129,
        ErrorBudget::FivePercent => 65,
        ErrorBudget::TenPercent => 33,
    };
    let mut v = vec![100; 8192];
    for i in 0..size {
        v[i * 8192 / size] = 0;
    }
    v
}
fn check(b: ErrorBudget) -> String {
    let v = values();
    let roots: Vec<_> = v.iter().map(|&n| sqrt_approx::integer_sqrt(n, b)).collect();
    let errors: Vec<_> = v
        .iter()
        .zip(&roots)
        .map(|(&n, &a)| {
            let reference = integer_sqrt::integer_sqrt(n);
            assert_eq!(reference, n.isqrt());
            relative_error(reference as f64, a as f64).unwrap()
        })
        .collect();
    let cert = observed_certificate(b, &errors).unwrap();
    let mut sorted = v.clone();
    sorted.sort_unstable();
    let rank = rank_approx::select_nth(&v, v.len() / 2, b).unwrap();
    let distance = rank_distance(&sorted, rank.value, v.len() / 2);
    assert_eq!(distance, rank.rank_error);
    assert!(distance as u128 * 100 <= v.len() as u128 * b.percent() as u128);
    let its = items();
    let exact = knapsack::max_value(&its, 1500).unwrap();
    let estimate = knapsack_approx::max_value(&its, 1500, b).unwrap();
    assert!(estimate.value <= exact && exact <= estimate.upper_bound);
    let loss = relative_error(exact as f64, estimate.value as f64).unwrap();
    observed_certificate(b, &[loss]).unwrap();
    let g = game();
    let expected = g.l0();
    let actual = rsa_approx::approximate_l0(&g.truth, &g.prior, b).unwrap();
    let rsa_max = rsa_error(&expected, &actual).unwrap();
    observed_certificate(b, &[rsa_max]).unwrap();
    let fallback = rank_approx::select_nth(&fallback_ranks(b), 4096, b).unwrap();
    assert!(fallback.used_exact_fallback && fallback.value == 100);
    let fallback_knap = knapsack_approx::max_value(&fallback_items(), 10, b).unwrap();
    assert!(fallback_knap.used_exact_fallback && fallback_knap.value == 8);
    let checksum = roots.iter().fold(0u64, |a, &x| a.wrapping_add(x));
    let rsa_checksum = actual
        .iter()
        .flatten()
        .fold(0u64, |a, &x| a.wrapping_add(x.to_bits()));
    format!(
        "CHECK budget={} sqrt_sum={} sqrt_max={:.12e} rank={} rank_error={} rank_fallback={} knapsack={}/{} knapsack_loss={:.12e} knapsack_fallback={} rsa_bits={} rsa_max={:.12e} forced_fallbacks=2/2",
        b.percent(),
        checksum,
        cert.error_fraction(),
        rank.value,
        distance,
        rank.used_exact_fallback,
        estimate.value,
        exact,
        loss,
        estimate.used_exact_fallback,
        rsa_checksum,
        rsa_max
    )
}
fn measure(f: &mut dyn FnMut() -> u64, iterations: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(f());
    }
    start.elapsed().as_secs_f64() * 1e6 / iterations as f64
}
fn timing(
    name: &str,
    b: ErrorBudget,
    iterations: usize,
    mut reference: impl FnMut() -> u64,
    mut surrogate: impl FnMut() -> u64,
) {
    black_box(reference());
    black_box(surrogate());
    let mut r = Vec::new();
    let mut a = Vec::new();
    for round in 0..7 {
        if round % 2 == 0 {
            r.push(measure(&mut reference, iterations));
            a.push(measure(&mut surrogate, iterations));
        } else {
            a.push(measure(&mut surrogate, iterations));
            r.push(measure(&mut reference, iterations));
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
    let v = values();
    let k = v.len() / 2;
    let its = items();
    let g = game();
    timing(
        "sqrt8192_original",
        b,
        2,
        || {
            v.iter()
                .map(|&n| integer_sqrt::integer_sqrt(black_box(n)))
                .sum()
        },
        || {
            v.iter()
                .map(|&n| sqrt_approx::integer_sqrt(black_box(n), b))
                .sum()
        },
    );
    timing(
        "sqrt8192_std",
        b,
        2,
        || v.iter().map(|&n| black_box(n).isqrt()).sum(),
        || {
            v.iter()
                .map(|&n| sqrt_approx::integer_sqrt(black_box(n), b))
                .sum()
        },
    );
    timing(
        "rank8192_original",
        b,
        16,
        || {
            let mut x = black_box(&v).clone();
            *quickselect::select_nth(&mut x, k).unwrap()
        },
        || rank_approx::select_nth(black_box(&v), k, b).unwrap().value,
    );
    timing(
        "rank8192_std",
        b,
        16,
        || {
            let mut x = black_box(&v).clone();
            *x.select_nth_unstable(k).1
        },
        || rank_approx::select_nth(black_box(&v), k, b).unwrap().value,
    );
    timing(
        "knapsack96_cap1500",
        b,
        4,
        || knapsack::max_value(black_box(&its), 1500).unwrap() as u64,
        || {
            knapsack_approx::max_value(black_box(&its), 1500, b)
                .unwrap()
                .value as u64
        },
    );
    timing(
        "rsa32x32",
        b,
        4,
        || black_box(g.l0()).len() as u64,
        || {
            black_box(
                rsa_approx::approximate_l0(black_box(&g.truth), black_box(&g.prior), b).unwrap(),
            )
            .len() as u64
        },
    );
    let adversarial = fallback_ranks(b);
    timing(
        "rank_forced_fallback_std",
        b,
        16,
        || {
            let mut x = black_box(&adversarial).clone();
            *x.select_nth_unstable(k).1
        },
        || {
            rank_approx::select_nth(black_box(&adversarial), k, b)
                .unwrap()
                .value
        },
    );
    let hard = fallback_items();
    timing(
        "knapsack_forced_fallback",
        b,
        64,
        || knapsack::max_value(black_box(&hard), 10).unwrap() as u64,
        || {
            knapsack_approx::max_value(black_box(&hard), 10, b)
                .unwrap()
                .value as u64
        },
    );
}
fn main() {
    println!(
        "Protocol: 7 alternating-order rounds; median[min,max] us/call; setup and compilation excluded; allocation, validation and fallback included. Rank compares value/rank, not partition. No whole-Mova speedup claim."
    );
    for b in ErrorBudget::ALL {
        println!("{}", classification_records(b));
        println!("{}", check(b));
    }
    for b in ErrorBudget::ALL {
        benchmark(b);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classifications_have_fixed_scoped_mapping() {
        assert_eq!(ErrorBudget::ALL.map(ErrorBudget::percent), [1, 2, 5, 10]);
        let expected = [
            ("sqrt", "scalar_relative", "quantized_numeric", "analytical"),
            (
                "rank",
                "normalized_rank",
                "certified_sample",
                "per_input_certificate",
            ),
            (
                "knapsack",
                "objective_loss",
                "relaxation_certificate",
                "per_input_certificate",
            ),
            (
                "rsa",
                "probability_relative",
                "floating_normalization",
                "analytical",
            ),
        ];
        assert_eq!(
            classifications().map(|(case, metric, method, assurance)| (
                case,
                metric.as_str(),
                method.as_str(),
                assurance.as_str()
            )),
            expected
        );
        for b in ErrorBudget::ALL {
            let records = classification_records(b);
            let expected_records = expected.map(|(case, metric, method, assurance)| format!(
                "CLASSIFICATION budget={} case={} metric_kind={} method_kind={} assurance_kind={} observation_assurance_kind=finite_observation",
                b.percent(), case, metric, method, assurance
            )).join("\n");
            assert_eq!(records, expected_records);
        }
    }
    #[test]
    fn deterministic_differential_checks() {
        for b in ErrorBudget::ALL {
            assert_eq!(check(b), check(b));
        }
    }
    #[test]
    fn independent_subset_oracle() {
        for seed in 1..20 {
            let mut s = seed;
            let items: Vec<_> = (0..8)
                .map(|_| knapsack::Item {
                    weight: (next(&mut s) % 12) as usize,
                    value: next(&mut s) % 30,
                })
                .collect();
            for cap in [0, 1, 7, 20, 40] {
                let optimum = (0..256)
                    .filter_map(|mask| {
                        let mut w = 0;
                        let mut value = 0u128;
                        for (i, item) in items.iter().enumerate() {
                            if mask & (1 << i) != 0 {
                                w += item.weight;
                                value += item.value as u128;
                            }
                        }
                        (w <= cap).then_some(value)
                    })
                    .max()
                    .unwrap();
                for b in ErrorBudget::ALL {
                    let a = knapsack_approx::max_value(&items, cap, b).unwrap();
                    assert!(a.value <= optimum && optimum <= a.upper_bound);
                    assert!((optimum - a.value) * 100 <= optimum * b.percent() as u128);
                }
            }
        }
    }
    #[test]
    fn corrupt_results_and_invalid_metrics_are_rejected() {
        for b in ErrorBudget::ALL {
            for invalid in [f64::NAN, f64::INFINITY, -0.1, 0.100_001, 0.15, 0.25] {
                assert!(observed_certificate(b, &[invalid]).is_err());
            }
            let g = game();
            let expected = g.l0();
            let mut actual = rsa_approx::approximate_l0(&g.truth, &g.prior, b).unwrap();
            for row in &mut actual {
                for x in row {
                    *x *= 0.5;
                }
            }
            assert!(observed_certificate(b, &[rsa_error(&expected, &actual).unwrap()]).is_err());
            actual[0][0] = f64::NAN;
            assert!(rsa_error(&expected, &actual).is_err());
        }
    }
}
