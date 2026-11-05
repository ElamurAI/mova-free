use super::*;
#[test]
fn dense_known_solution_oracle() {
    for n in 1..=12 {
        let mut a = vec![vec![1.0; n]; n];
        for (i, row) in a.iter_mut().enumerate() {
            row[i] += f64::from(u32::try_from(i + 1).unwrap());
        }
        let expected: Vec<f64> = (0..n)
            .map(|i| f64::from(u32::try_from(i).unwrap()) - 3.0)
            .collect();
        let b: Vec<f64> = a
            .iter()
            .map(|row| row.iter().zip(&expected).map(|(a, x)| a * x).sum())
            .collect();
        let r = solve(&a, &b, &vec![0.0; n], 1e-9, 100).unwrap();
        assert_eq!(r.status, Status::Converged);
        for (x, y) in r.x.iter().zip(expected) {
            assert!((x - y).abs() < 1e-8, "dense coordinate oracle");
        }
        let residual = a
            .iter()
            .zip(&b)
            .map(|(row, b)| (row.iter().zip(&r.x).map(|(a, x)| a * x).sum::<f64>() - b).powi(2))
            .sum::<f64>()
            .sqrt();
        assert!((residual - r.residual_norm).abs() < 1e-15);
    }
}
#[test]
fn budget_zero_and_underflow_stagnation() {
    assert_eq!(
        solve(&[vec![1.0]], &[0.0], &[0.0], 1e-10, 0)
            .unwrap()
            .status,
        Status::Converged
    );
    assert_eq!(
        solve(&[vec![1.0]], &[1.0], &[0.0], 1e-10, 0)
            .unwrap()
            .status,
        Status::BudgetExhausted
    );
    assert_eq!(
        solve(&[vec![1.0]], &[1e-200], &[0.0], 1e-250, 10)
            .unwrap()
            .status,
        Status::Stagnated
    );
}
#[test]
fn validation_curvature_and_arithmetic() {
    assert_eq!(
        solve(
            &[vec![1.0, 2.0], vec![0.0, 1.0]],
            &[1.0, 1.0],
            &[0.0; 2],
            0.1,
            10
        ),
        Err(Error::NotSymmetric)
    );
    assert_eq!(
        solve(&[vec![-1.0]], &[1.0], &[0.0], 0.1, 10),
        Err(Error::NonPositiveCurvature)
    );
    assert_eq!(
        solve(&[vec![1e100]], &[0.0], &[1e100], 0.1, 10),
        Err(Error::NonFiniteArithmetic)
    );
    assert_eq!(solve(&[], &[], &[], 0.1, 10), Err(Error::InvalidInput));
    assert_eq!(
        solve(&[vec![1.0]], &[1.0], &[0.0], f64::NAN, 10),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        solve(&[vec![1.0]], &[1.0], &[0.0], 0.1, 100001),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        solve(&[vec![f64::INFINITY]], &[1.0], &[0.0], 0.1, 10),
        Err(Error::InvalidInput)
    );
}
