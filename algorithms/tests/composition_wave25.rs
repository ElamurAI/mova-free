//! Bounded composition witnesses. Shared observations do not equate trajectories,
//! certify arbitrary f64 accuracy, or justify the explicitly rejected transfers.
use mova_algorithms::*;
fn close(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance, "{a} != {b}");
}
#[test]
fn diagonal_stationarity_join() {
    let d = [1.0, 3.0, 9.0];
    let b = [2.0, -3.0, 4.5];
    let initial = [0.0; 3];
    let reports = [
        heavy_ball_quadratic::minimize_diagonal(&d, &b, &initial, 1e-9, 1000).unwrap(),
        nesterov_quadratic::minimize_diagonal(&d, &b, &initial, 1e-9, 1000).unwrap(),
        barzilai_borwein::minimize_diagonal(&d, &b, &initial, 1e-9, 1000).unwrap(),
    ];
    for r in reports {
        assert_eq!(r.status, gradient_descent_quadratic::Status::Converged);
        for i in 0..3 {
            close(r.x[i], b[i] / d[i], 1e-8);
        }
        let residual =
            r.x.iter()
                .enumerate()
                .map(|(i, x)| (d[i] * x - b[i]).powi(2))
                .sum::<f64>()
                .sqrt();
        close(r.residual_norm, residual, 1e-14);
    }
}
#[test]
fn lasso_soft_threshold_join() {
    let design = vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]];
    let b = [3., -2., 0.25];
    let expected = soft_threshold::soft_threshold(&b, 0.5).unwrap();
    for r in [
        proximal_gradient_lasso::minimize(&design, &b, 0.5, 1e-9, 1000).unwrap(),
        fista_lasso::minimize(&design, &b, 0.5, 1e-9, 1000).unwrap(),
    ] {
        assert_eq!(r.status, projected_gradient_box::Status::Converged);
        for (x, y) in r.x.iter().zip(&expected) {
            close(*x, *y, 1e-8);
        }
        let obj = 0.5 * r.x.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f64>()
            + 0.5 * r.x.iter().map(|x| x.abs()).sum::<f64>();
        close(r.value, obj, 1e-12);
    }
}
#[test]
fn reduction_reachability_rejects_dominator_transfer() {
    for bits in 0u32..64 {
        let mut g = vec![vec![]; 4];
        let mut bit = 0;
        for (u, row) in g.iter_mut().enumerate() {
            for v in u + 1..4 {
                if bits & (1 << bit) != 0 {
                    row.push(v);
                }
                bit += 1;
            }
        }
        let reduced = transitive_reduction_dag::transitive_reduction_dag(&g).unwrap();
        for start in 0..4 {
            let before = bfs::bfs(&g, start).unwrap();
            let after = bfs::bfs(&reduced, start).unwrap();
            assert_eq!(
                before.iter().map(Option::is_some).collect::<Vec<_>>(),
                after.iter().map(Option::is_some).collect::<Vec<_>>()
            );
        }
    }
    let g = vec![vec![1, 2], vec![2], vec![]];
    let reduced = transitive_reduction_dag::transitive_reduction_dag(&g).unwrap();
    let before = dominators::dominators(&g, 0).unwrap();
    let after = dominators::dominators(&reduced, 0).unwrap();
    assert_eq!(before[2], vec![0, 2]);
    assert_eq!(after[2], vec![0, 1, 2]);
    assert_ne!(before, after);
}
#[test]
fn unit_flow_bfs_join() {
    for bits in 0u32..64 {
        let mut g = vec![vec![]; 4];
        let mut edges = vec![];
        let mut bit = 0;
        for (u, row) in g.iter_mut().enumerate() {
            for v in u + 1..4 {
                if bits & (1 << bit) != 0 {
                    row.push(v);
                    edges.push(minimum_cost_flow::Edge {
                        from: u,
                        to: v,
                        capacity: 1,
                        cost: 1,
                    });
                }
                bit += 1;
            }
        }
        let distance = bfs::bfs(&g, 0).unwrap()[3];
        let flow = minimum_cost_flow::minimum_cost_flow(4, &edges, 0, 3, 1).unwrap();
        assert_eq!(flow.value, u64::from(distance.is_some()));
        assert_eq!(flow.cost, distance.unwrap_or(0) as i128);
        let mut balance = [0i64; 4];
        for (e, f) in edges.iter().zip(&flow.edge_flows) {
            assert!(*f <= 1);
            balance[e.from] -= *f as i64;
            balance[e.to] += *f as i64;
        }
        assert_eq!(balance, [-(flow.value as i64), 0, 0, flow.value as i64]);
    }
}
#[test]
fn clique_order_core_coordinates_join() {
    for n in 1..=8 {
        let graph: Vec<Vec<usize>> = (0..n)
            .map(|u| (0..n).filter(|v| *v != u).collect())
            .collect();
        let order = maximum_cardinality_search::maximum_cardinality_search(&graph).unwrap();
        assert_eq!(order, (0..n).collect::<Vec<_>>());
        assert_eq!(k_core::k_core(&graph, n - 1).unwrap(), order);
        assert!(k_core::k_core(&graph, n).unwrap().is_empty());
    }
    // A four-cycle still has a 2-core and an MCS order; those facts do not imply chordality.
    let cycle = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    assert_eq!(k_core::k_core(&cycle, 2).unwrap(), vec![0, 1, 2, 3]);
    assert_eq!(
        maximum_cardinality_search::maximum_cardinality_search(&cycle).unwrap(),
        vec![0, 1, 2, 3]
    );
    assert!(!cycle[0].contains(&2) && !cycle[1].contains(&3));
}
#[test]
fn interpolation_polynomial_and_spline_affine_join() {
    let xs = [-2.0_f64, -0.5, 1.0, 3.0];
    for degree in 0..=3 {
        let ys = xs.map(|x| x.powi(degree));
        let p = newton_divided_differences::NewtonPolynomial::new(&xs, &ys).unwrap();
        for k in -8..=12 {
            let q = k as f64 / 4.0;
            let n = neville_interpolation::neville_interpolation(&xs, &ys, q).unwrap();
            close(n, q.powi(degree), 1e-10);
            close(p.evaluate(q).unwrap(), n, 1e-10);
        }
    }
    let ys = xs.map(|x| 2.0 * x + 1.0);
    let s = natural_cubic_spline::NaturalCubicSpline::new(&xs, &ys).unwrap();
    let p = newton_divided_differences::NewtonPolynomial::new(&xs, &ys).unwrap();
    for k in -8..=12 {
        let q = k as f64 / 4.0;
        close(s.evaluate(q).unwrap(), p.evaluate(q).unwrap(), 1e-12);
    }
}
#[test]
fn quadrature_analytic_join() {
    for degree in 0..=3 {
        let a = adaptive_trapezoid::integrate(|x| x.powi(degree), 0., 1., 1e-8, 100000).unwrap();
        let b = tanh_sinh_quadrature::integrate(|x| x.powi(degree), 0., 1., 1e-10, 10000).unwrap();
        assert_eq!(a.status, adaptive_trapezoid::Status::EstimatedTolerance);
        assert_eq!(b.status, tanh_sinh_quadrature::Status::EstimatedTolerance);
        let integral = 1.0 / (degree + 1) as f64;
        close(a.value, integral, 2e-8);
        close(b.value, integral, 1e-10);
        close(a.value, b.value, 2e-8);
    }
}
fn fft_product(a: &[u64], b: &[u64]) -> Vec<f64> {
    use fast_fourier_transform::{Complex, transform};
    let n = (a.len() + b.len() - 1).next_power_of_two();
    let pack = |v: &[u64]| {
        (0..n)
            .map(|i| Complex {
                re: v.get(i).copied().unwrap_or(0) as f64,
                im: 0.0,
            })
            .collect::<Vec<_>>()
    };
    let fa = transform(&pack(a), false).unwrap();
    let fb = transform(&pack(b), false).unwrap();
    let product: Vec<_> = fa
        .iter()
        .zip(fb)
        .map(|(a, b)| Complex {
            re: a.re * b.re - a.im * b.im,
            im: a.re * b.im + a.im * b.re,
        })
        .collect();
    transform(&product, true)
        .unwrap()
        .iter()
        .take(a.len() + b.len() - 1)
        .map(|z| {
            close(z.im, 0.0, 1e-8);
            z.re
        })
        .collect()
}
#[test]
fn fft_exact_field_fixture_join() {
    for n in [1, 3, 17] {
        let a: Vec<u64> = (0..n).map(|i| (i % 5) as u64).collect();
        let b: Vec<u64> = (0..n + 1).map(|i| (i % 3) as u64).collect();
        let exact = karatsuba_polynomial::multiply(&a, &b).unwrap();
        let transformed = fft_product(&a, &b);
        let mut oracle = vec![0u64; a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                oracle[i + j] += x * y;
            }
        }
        assert_eq!(exact, oracle);
        for (x, y) in transformed.iter().zip(exact) {
            close(*x, y as f64, 1e-8);
        }
    }
}
#[test]
fn subset_triangular_reconstruction_rejects_or_join() {
    let a = [2, 3, 5, 7, 11, 13, 17, 19];
    let b = [23, 29, 31, 37, 41, 43, 47, 53];
    let c = subset_convolution::convolve(&a, &b).unwrap();
    let matrix: Vec<Vec<u64>> = (0..8)
        .map(|s| {
            (0..8)
                .map(|t| if t & s == t { a[s ^ t] } else { 0 })
                .collect()
        })
        .collect();
    let solution = gaussian_elimination_modular::solve(&matrix, &c, 8)
        .unwrap()
        .unwrap();
    assert_eq!(solution.particular, b);
    assert!(solution.nullspace.is_empty());
    // Disjoint convolution cannot include a singleton in both operands; OR convolution does.
    assert_eq!(
        subset_convolution::convolve(&[0, 1], &[0, 1]).unwrap(),
        vec![0, 0]
    );
    let mut or_product = [0u64; 2];
    for (i, x) in [0, 1].iter().enumerate() {
        for (j, y) in [0, 1].iter().enumerate() {
            or_product[i | j] += x * y;
        }
    }
    assert_eq!(or_product, [0, 1]);
}
fn modular_power(mut base: u64, mut power: u64) -> u64 {
    let modulus = baby_step_giant_step::MODULUS;
    let mut out = 1;
    while power > 0 {
        if power & 1 != 0 {
            out = out * base % modulus;
        }
        base = base * base % modulus;
        power >>= 1;
    }
    out
}
#[test]
fn discrete_log_target_join() {
    for exponent in [0, 1, 2, 7, 31] {
        let target = modular_power(3, exponent);
        let found = baby_step_giant_step::discrete_log(3, target)
            .unwrap()
            .unwrap();
        assert_eq!(found, exponent);
        assert_eq!(modular_power(3, found), target);
        for earlier in 0..found {
            assert_ne!(modular_power(3, earlier), target);
        }
    }
    assert_eq!(baby_step_giant_step::discrete_log(0, 1).unwrap(), Some(0));
    assert_eq!(baby_step_giant_step::discrete_log(0, 0).unwrap(), Some(1));
    assert_eq!(baby_step_giant_step::discrete_log(1, 2).unwrap(), None);
}
fn token_bytes(tokens: &[u16]) -> Vec<u8> {
    tokens.iter().flat_map(|t| t.to_le_bytes()).collect()
}
fn bytes_tokens(bytes: &[u8]) -> Vec<u16> {
    assert_eq!(bytes.len() % 2, 0);
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| u16::from_le_bytes([p[0], p[1]]))
        .collect()
}
#[test]
fn bpe_codec_model_bound_join() {
    let data = b"abababababab xyxyxyxy abab";
    let model = byte_pair_encoding::Model::train(data, 8).unwrap();
    let tokens = model.encode(data).unwrap();
    assert!(tokens.iter().any(|x| *x > 255));
    let packed = token_bytes(&tokens);
    let z = lz77::encode(&packed, 32, 32).unwrap();
    let w = lzw::encode(&packed).unwrap();
    for decoded in [
        lz77::decode(&z, packed.len()).unwrap(),
        lzw::decode(&w, packed.len()).unwrap(),
    ] {
        let restored = bytes_tokens(&decoded);
        assert_eq!(restored, tokens);
        assert_eq!(model.decode(&restored, data.len()).unwrap(), data);
        assert_eq!(
            model.decode(&restored, data.len() - 1),
            Err(byte_pair_encoding::Error::OutputLimit)
        );
    }
    let wrong = byte_pair_encoding::Model::train(b"", 0).unwrap();
    assert_eq!(
        wrong.decode(&tokens, data.len()),
        Err(byte_pair_encoding::Error::InvalidToken)
    );
}
#[test]
fn patience_myers_edit_observations_join() {
    for (old, new) in [
        (b"abc".as_slice(), b"axc".as_slice()),
        (b"abab", b"baba"),
        (b"", b"xyz"),
        (b"same", b"same"),
    ] {
        let patch = patience_diff::diff(old, new).unwrap();
        let rebuilt = patience_diff::apply(old, &patch, new.len()).unwrap();
        assert_eq!(rebuilt, new);
        let distance = myers_bitvector::distance(old, &rebuilt).unwrap();
        assert_eq!(distance, levenshtein::distance(old, new));
        let edits = patch
            .iter()
            .filter(|e| !matches!(e, patience_diff::Edit::Equal(_)))
            .count();
        assert!(edits >= distance);
    }
    let patch = patience_diff::diff(b"abc", b"axc").unwrap();
    let edits = patch
        .iter()
        .filter(|e| !matches!(e, patience_diff::Edit::Equal(_)))
        .count();
    assert_eq!(edits, 2);
    assert_eq!(myers_bitvector::distance(b"abc", b"axc").unwrap(), 1);
}
