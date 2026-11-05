//! Bounded production joins; independent fixture identities are not global numeric guarantees.
use mova_algorithms::*;
fn close(a: f64, b: f64, t: f64) {
    assert!((a - b).abs() <= t, "{a} != {b}");
}
#[test]
fn factor_triangular_and_stationary_solvers_join_spd_system() {
    let a = vec![vec![4., 1.], vec![1., 3.]];
    let b = [6., 7.];
    let c = cholesky::factor(&a).unwrap();
    let y = triangular_solve::solve(&c.lower, &b, triangular_solve::Triangle::Lower).unwrap();
    let u: Vec<Vec<f64>> = (0..2)
        .map(|i| (0..2).map(|j| c.lower[j][i]).collect())
        .collect();
    let x = triangular_solve::solve(&u, &y.x, triangular_solve::Triangle::Upper).unwrap();
    let g = gauss_seidel::solve(&a, &b, &[0., 0.], 1e-11, 1000).unwrap();
    let j = jacobi_iteration::solve(&a, &b, &[0., 0.], 1e-11, 1000).unwrap();
    assert_eq!(g.status, gauss_seidel::Status::Converged);
    assert_eq!(j.status, jacobi_iteration::Status::Converged);
    for v in [&x.x, &g.x, &j.x] {
        assert_eq!(v.len(), 2);
        for (i, &value) in v.iter().enumerate() {
            close(value, (i + 1) as f64, 1e-9);
        }
    }
    close(c.reconstruction_residual, 0., 1e-12);
}
#[test]
fn qr_and_constrained_optimizers_join_known_quadratic() {
    let design = vec![vec![1., 0.], vec![0., 1.], vec![1., 1.]];
    let q = householder_qr::least_squares(&design, &[0.25, 0.75, 1.]).unwrap();
    close(q.x[0], 0.25, 1e-12);
    close(q.x[1], 0.75, 1e-12);
    let h = vec![vec![2., 1.], vec![1., 2.]];
    let linear = [1.25, 1.75];
    let p =
        projected_gradient_box::minimize(&h, &linear, &[0., 0.], &[1., 1.], &[0., 0.], 1e-9, 10000)
            .unwrap();
    let f = frank_wolfe::minimize(&h, &linear, &[0.5, 0.5], 1e-9, 10000).unwrap();
    for v in [&p.x, &f.x] {
        close(v[0], q.x[0], 1e-4);
        close(v[1], q.x[1], 1e-4);
    }
}
#[test]
fn brent_and_armijo_join_scalar_quadratic() {
    let r = brent_minimize::minimize(|x| (x - 2.) * (x - 2.), -4., 5., 1e-9, 200).unwrap();
    close(r.estimate, 2., 1e-6);
    let mut calls = 0;
    let a = armijo_line_search::search(
        |t| {
            calls += 1;
            (4. * t - 2.).powi(2)
        },
        4.,
        -16.,
        1.,
        0.5,
        0.1,
        30,
    )
    .unwrap();
    assert_eq!(a.status, armijo_line_search::Status::Converged);
    close(a.step, 0.5, 0.);
    close(4. * a.step, r.estimate, 1e-6);
    assert_eq!(a.evaluations, calls);
    assert!(a.value <= 4. - 1.6 * a.step);
}
#[test]
fn lasso_and_triangular_identity_join_soft_threshold_solution() {
    let id = vec![vec![1., 0.], vec![0., 1.]];
    let ls = triangular_solve::solve(&id, &[3., -2.], triangular_solve::Triangle::Lower).unwrap();
    let r = coordinate_descent_lasso::minimize(&id, &ls.x, 0.5, 1e-10, 1000).unwrap();
    close(r.x[0], 2.5, 1e-8);
    close(r.x[1], -1.5, 1e-8);
}
#[test]
fn barycentric_bezier_and_gauss_join_polynomial_identity() {
    let nodes = [(0., 1.), (0.5, 2.), (1., 4.)];
    let controls = [1., 1.5, 4.];
    for k in 0..=32 {
        let t = k as f64 / 32.;
        let oracle = 1. + t + 2. * t * t;
        close(
            barycentric_lagrange::interpolate(&nodes, t).unwrap(),
            oracle,
            1e-12,
        );
        close(
            bernstein_de_casteljau::evaluate(&controls, t).unwrap(),
            oracle,
            1e-12,
        );
    }
    let integral = gauss_legendre::integrate(
        |x| barycentric_lagrange::interpolate(&nodes, x).unwrap(),
        0.,
        1.,
        3,
    )
    .unwrap();
    close(integral, 13. / 6., 1e-12);
}
#[test]
fn monotone_cubic_romberg_and_gauss_join_affine_integral() {
    let nodes = [(-1., -1.), (0., 2.), (1., 5.)];
    let mut last = f64::NEG_INFINITY;
    for k in 0..=32 {
        let x = -1. + k as f64 / 16.;
        let y = monotone_cubic::interpolate(&nodes, x).unwrap();
        assert!(y >= last);
        last = y;
        close(y, 3. * x + 2., 1e-12);
    }
    let r = romberg_integration::integrate(
        |x| monotone_cubic::interpolate(&nodes, x).unwrap(),
        -1.,
        1.,
        1e-10,
        12,
        10000,
    )
    .unwrap();
    let g = gauss_legendre::integrate(|x| 3. * x + 2., -1., 1., 2).unwrap();
    close(r.value, 4., 1e-10);
    close(g, r.value, 1e-10);
    assert_eq!(r.status, romberg_integration::Status::Converged);
}
#[test]
fn bitset_and_mitm_join_exhaustive_attainable_objectives() {
    for n in 0..=8 {
        let values: Vec<usize> = (0..n).map(|i| (7 * i + 3) % 13).collect();
        let items: Vec<knapsack::Item> = values
            .iter()
            .map(|&v| knapsack::Item {
                weight: v,
                value: v as u64,
            })
            .collect();
        for cap in 0..=32 {
            let sums: Vec<usize> = (0..1usize << n)
                .map(|mask| {
                    values
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, v)| v)
                        .sum()
                })
                .collect();
            assert_eq!(
                bitset_subset_sum::reachable(&values, cap, 100).unwrap(),
                sums.contains(&cap)
            );
            let best = *sums.iter().filter(|&&s| s <= cap).max().unwrap();
            assert_eq!(
                meet_in_middle_knapsack::max_value(&items, cap, 1024).unwrap(),
                best as u128
            );
        }
    }
}
#[test]
fn smawk_partition_and_knuth_join_independent_dp_oracles() {
    for n in 1usize..=12 {
        let mins = smawk::row_minima(n, n, 10000, |i, j| {
            let d = i as i128 - j as i128;
            d * d
        })
        .unwrap();
        assert_eq!(mins, (0..n).collect::<Vec<_>>());
        for layers in 1..=n {
            let mut dp = vec![u128::MAX; n + 1];
            dp[0] = 0;
            for _ in 0..layers {
                let mut next = vec![u128::MAX; n + 1];
                for (j, output) in next.iter_mut().enumerate().skip(1) {
                    for (k, &previous) in dp.iter().enumerate().take(j) {
                        if previous != u128::MAX {
                            *output = (*output).min(previous + ((j - k) * (j - k)) as u128);
                        }
                    }
                }
                dp = next;
            }
            assert_eq!(
                divide_conquer_dp::partition(n, layers, 10000, |i, j| {
                    let d = mins[j - 1] + 1 - i;
                    (d * d) as u128
                })
                .unwrap(),
                Some(dp[n])
            );
        }
        let weights: Vec<u64> = mins.iter().map(|i| (i % 4 + 1) as u64).collect();
        let mut dp = vec![vec![0u128; n + 1]; n + 1];
        for len in 1..=n {
            for i in 0..=n - len {
                let j = i + len;
                let sum: u128 = weights[i..j].iter().map(|&v| v as u128).sum();
                dp[i][j] = (i..j).map(|k| dp[i][k] + dp[k + 1][j] + sum).min().unwrap();
            }
        }
        assert_eq!(
            knuth_optimal_bst::min_cost(&weights, 10000).unwrap(),
            dp[0][n]
        );
    }
}
