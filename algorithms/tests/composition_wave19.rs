//! Bounded independent client relations; numerical samples are not universal bounds.
use mova_algorithms::{
    adaptive_simpson, bisection_root, chebyshev_clenshaw, compensated_sum, conjugate_gradient,
    golden_section_search, gradient_descent_quadratic, histogram_quantiles, horner,
    isotonic_regression, l1_ball_projection, misra_gries, simplex_projection, soft_threshold,
    space_saving, streaming_moments,
};
fn close(a: f64, b: f64, tol: f64) {
    assert!(
        a.is_finite() && b.is_finite() && (a - b).abs() <= tol,
        "{a} != {b}, tolerance {tol}"
    );
}
#[test]
fn quadratic_solvers_join_scalar_roots_and_polynomial_values() {
    for n in 1..=4 {
        let d: Vec<f64> = (1..=n).map(|i| i as f64).collect();
        let expected: Vec<f64> = (0..n).map(|i| i as f64 - 1.5).collect();
        let b: Vec<f64> = d.iter().zip(&expected).map(|(d, x)| d * x).collect();
        let matrix: Vec<Vec<f64>> = (0..n)
            .map(|i| (0..n).map(|j| if i == j { d[i] } else { 0.0 }).collect())
            .collect();
        let initial = vec![0.0; n];
        let gd =
            gradient_descent_quadratic::minimize_diagonal(&d, &b, &initial, 1e-10, 10000).unwrap();
        let cg = conjugate_gradient::solve(&matrix, &b, &initial, 1e-10, 100).unwrap();
        for i in 0..n {
            let coeff = [0.0, -b[i], 0.5 * d[i]];
            let root = bisection_root::solve(
                |x| horner::evaluate(&coeff, x).unwrap().1,
                -4.0,
                4.0,
                1e-10,
                100,
            )
            .unwrap();
            let minimum = golden_section_search::minimize(
                |x| horner::evaluate(&coeff, x).unwrap().0,
                -4.0,
                4.0,
                1e-8,
                200,
            )
            .unwrap();
            for x in [gd.x[i], cg.x[i], root.estimate, minimum.estimate] {
                close(x, expected[i], 1e-6);
            }
            close(horner::evaluate(&coeff, cg.x[i]).unwrap().1, 0.0, 1e-9);
        }
    }
}
#[test]
fn polynomial_basis_join_derivative_and_quadrature() {
    // 2 + 3T1 - 2T2 + T3 = 4 - 4x^2 + 4x^3.
    let cheb = [2.0, 3.0, -2.0, 1.0];
    let power = [4.0, 0.0, -4.0, 4.0];
    for k in -16..=16 {
        let x = k as f64 / 16.0;
        let (v, d) = horner::evaluate(&power, x).unwrap();
        close(v, 4.0 - 4.0 * x * x + 4.0 * x * x * x, 1e-13);
        close(chebyshev_clenshaw::evaluate(&cheb, x).unwrap(), v, 1e-13);
        close(d, -8.0 * x + 12.0 * x * x, 1e-13);
    }
    let integral = adaptive_simpson::integrate(
        |x| chebyshev_clenshaw::evaluate(&cheb, x).unwrap(),
        -1.0,
        1.0,
        1e-10,
        1000,
        20,
    )
    .unwrap();
    close(integral.value, 16.0 / 3.0, 1e-12);
}
#[test]
fn projections_join_threshold_reconstruction_and_mean() {
    let x = [-3.0, 1.0, -2.0, 0.0];
    let magnitudes = [3.0, 1.0, 2.0, 0.0];
    let simplex = simplex_projection::project_simplex(&magnitudes, 3.0).unwrap();
    let ball = l1_ball_projection::project_l1_ball(&x, 3.0).unwrap();
    let threshold = soft_threshold::soft_threshold(&x, 1.0).unwrap();
    assert_eq!(simplex, vec![2.0, 0.0, 1.0, 0.0]);
    assert_eq!(ball, vec![-2.0, 0.0, -1.0, 0.0]);
    assert_eq!(ball, threshold);
    for i in 0..x.len() {
        close(ball[i].abs(), simplex[i], 0.0);
    }
    for input in [
        vec![3.0, 1.0, 2.0],
        vec![-2.0, 4.0, 0.0, 2.0],
        vec![5.0, 4.0, 3.0, 2.0],
    ] {
        let projected = isotonic_regression::isotonic_regression(&input).unwrap();
        assert!(projected.windows(2).all(|w| w[0] <= w[1]));
        close(
            compensated_sum::sum(&input).unwrap(),
            compensated_sum::sum(&projected).unwrap(),
            1e-12,
        );
        let again = isotonic_regression::isotonic_regression(&projected).unwrap();
        assert_eq!(again, projected);
        // Normal-cone inequalities against independently constructed feasible constants.
        for c in -3..=6 {
            let normal: f64 = input
                .iter()
                .zip(&projected)
                .map(|(x, y)| (x - y) * (c as f64 - y))
                .sum();
            assert!(normal <= 1e-12);
        }
    }
}
#[test]
fn merged_moments_join_compensated_centered_sums() {
    for n in 1..=24 {
        let values: Vec<f64> = (0..n).map(|i| ((i * 7) % 11) as f64 - 5.0).collect();
        let mut left = streaming_moments::StreamingMoments::new();
        let mut right = streaming_moments::StreamingMoments::new();
        for &x in &values[..n / 2] {
            left.update(x).unwrap();
        }
        for &x in &values[n / 2..] {
            right.update(x).unwrap();
        }
        left.merge(&right).unwrap();
        let mean = compensated_sum::sum(&values).unwrap() / n as f64;
        let deviations: Vec<f64> = values.iter().map(|x| (x - mean) * (x - mean)).collect();
        let variance = compensated_sum::sum(&deviations).unwrap() / n as f64;
        assert_eq!(left.count(), n as u64);
        close(left.mean().unwrap(), mean, 1e-12);
        close(left.population_variance().unwrap(), variance, 1e-12);
        if n > 1 {
            close(
                left.sample_variance().unwrap(),
                variance * n as f64 / (n - 1) as f64,
                1e-12,
            );
        }
    }
}
#[test]
fn frequency_summaries_join_literal_histogram_certificates() {
    for tier in [1, 2, 5, 10] {
        let mut mg = misra_gries::MisraGries::new(tier).unwrap();
        let mut ss = space_saving::SpaceSaving::new(tier).unwrap();
        let mut counts = std::collections::BTreeMap::new();
        for i in 0..800u64 {
            let key = if i % 3 == 0 { 7 } else { i % 137 };
            mg.update(key).unwrap();
            ss.update(key).unwrap();
            *counts.entry(key).or_insert(0u64) += 1;
        }
        assert_eq!(mg.len(), 800);
        assert_eq!(ss.len(), 800);
        for key in 0..=140 {
            let true_count = *counts.get(&key).unwrap_or(&0);
            let (lo, hi) = mg.bounds(key);
            let (sl, su) = ss.bounds(key);
            assert!(lo <= true_count && true_count <= hi);
            assert!(sl <= true_count && true_count <= su);
            assert!((hi - lo) * 100 <= 800 * tier as u64);
            assert!((su - sl) * 100 <= 800 * tier as u64);
        }
    }
}
#[test]
fn histogram_intervals_join_sorted_rank_and_exact_counts() {
    let mut h = histogram_quantiles::HistogramQuantiles::new(0, 100, 7).unwrap();
    let mut exact: Vec<u64> = (0..257).map(|i| (i * 17) % 101).collect();
    for &x in &exact {
        h.update(x).unwrap();
    }
    exact.sort_unstable();
    assert_eq!(h.len(), 257);
    for (rank, &value) in exact.iter().enumerate() {
        let q = h.quantile(rank as u64).unwrap();
        assert!(q.value_min <= value && value <= q.value_max);
        assert!(q.rank_start <= rank as u64 && (rank as u64) < q.rank_end);
        assert_eq!(
            q.rank_start,
            exact.iter().filter(|&&x| x < q.value_min).count() as u64
        );
        assert_eq!(
            q.rank_end,
            exact.iter().filter(|&&x| x <= q.value_max).count() as u64
        );
    }
    assert!(h.quantile(257).is_none());
}

#[test]
fn fptas_objectives_join_exhaustive_subset_fibers() {
    use mova_algorithms::{knapsack::Item, knapsack_fptas, subset_sum_fptas};
    for n in 0..=8 {
        let values: Vec<u64> = (0..n).map(|i| ((i * 7 + 3) % 13) as u64).collect();
        let items: Vec<Item> = values
            .iter()
            .map(|&v| Item {
                weight: v as usize,
                value: v,
            })
            .collect();
        for target in [0, 1, 7, 19, 40] {
            let attainable: Vec<u64> = (0..1usize << n)
                .map(|mask| {
                    values
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, v)| v)
                        .sum()
                })
                .filter(|&s| s <= target)
                .collect();
            let optimum = *attainable.iter().max().unwrap();
            for tier in [1, 2, 5, 10] {
                let k =
                    knapsack_fptas::max_value(&items, target as usize, tier, 2_000_000).unwrap();
                let s = subset_sum_fptas::max_sum(&values, target, tier, 2_000_000).unwrap();
                assert!(attainable.contains(&(k.value as u64)));
                assert!(attainable.contains(&s.sum));
                for v in [k.value, s.sum as u128] {
                    assert!(v <= optimum as u128);
                    assert!(v * 100 >= (100 - tier as u128) * optimum as u128);
                }
                assert!((optimum as u128 - k.value) * 100 <= k.loss_numerator);
                assert!(k.loss_numerator <= tier as u128 * optimum as u128);
            }
        }
    }
}
#[test]
fn rational_certificates_join_polynomials_and_compensated_products() {
    use mova_algorithms::{
        piecewise_linear::{self, Knot},
        quantized_dot,
    };
    let knots = [
        Knot { x: -4, y: -5 },
        Knot { x: 0, y: 3 },
        Knot { x: 6, y: 15 },
    ];
    for x in -4..=6 {
        let e = piecewise_linear::interpolate(&knots, x, 0, 3).unwrap();
        assert_eq!(e.numerator, (2 * x + 3) as i128 * e.denominator as i128);
        assert_eq!(e.error_numerator, 0);
        close(
            e.numerator as f64 / e.denominator as f64,
            horner::evaluate(&[3.0, 2.0], x as f64).unwrap().0,
            0.0,
        );
    }
    let integral = adaptive_simpson::integrate(
        |x| horner::evaluate(&[3.0, 2.0], x).unwrap().0,
        -4.0,
        6.0,
        1e-10,
        100,
        10,
    )
    .unwrap();
    close(integral.value, 50.0, 1e-12);
    for step in [1, 2, 4] {
        for sign in [-1, 1] {
            let a = [101i64, -103, 108, -100];
            let b = [100 * sign, 100 * sign, 100, -100];
            let q = quantized_dot::dot(&a, &b, step, 10, 4).unwrap();
            let exact: i128 = a.iter().zip(b).map(|(&a, b)| a as i128 * b as i128).sum();
            let products: Vec<f64> = a.iter().zip(b).map(|(&a, b)| (a * b) as f64).collect();
            close(compensated_sum::sum(&products).unwrap(), exact as f64, 0.0);
            assert!((q.value - exact).unsigned_abs() <= q.absolute_error);
            assert!(q.absolute_error * 100 <= 10 * q.l1_products);
            assert_eq!(
                q.l1_products,
                a.iter()
                    .zip(b)
                    .map(|(&a, b)| (a as i128 * b as i128).unsigned_abs())
                    .sum()
            );
        }
    }
    let zero = quantized_dot::dot(&[0, 0], &[3, -9], 4, 1, 2).unwrap();
    assert_eq!(
        (zero.value, zero.absolute_error, zero.l1_products),
        (0, 0, 0)
    );
}
