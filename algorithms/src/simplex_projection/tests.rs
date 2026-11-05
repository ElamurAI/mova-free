use super::*;
fn oracle(a: &[f64], r: f64) -> Vec<f64> {
    for mask in 1..1usize << a.len() {
        let k = mask.count_ones() as f64;
        let theta = (a
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, x)| x)
            .sum::<f64>()
            - r)
            / k;
        let y: Vec<_> = a.iter().map(|x| (x - theta).max(0.0)).collect();
        if a.iter().enumerate().all(|(i, x)| {
            if mask & (1 << i) != 0 {
                *x > theta
            } else {
                *x <= theta
            }
        }) {
            return y;
        }
    }
    panic!("no active set")
}
#[test]
fn active_set_oracle() {
    for code in 0..125 {
        let mut c = code;
        let a: Vec<f64> = (0..3)
            .map(|_| {
                let x = (c % 5) as f64 - 2.0;
                c /= 5;
                x
            })
            .collect();
        for r in [0.25, 1.0, 4.0] {
            let y = project_simplex(&a, r).unwrap();
            let expected = oracle(&a, r);
            assert!(
                y.iter().zip(expected).all(|(x, y)| (x - y).abs() < 1e-12),
                "active set oracle"
            );
            assert!((y.iter().sum::<f64>() - r).abs() < 1e-12);
        }
    }
}
#[test]
fn boundaries_and_fibers() {
    assert_eq!(project_simplex(&[3.0, 3.0], 2.0), Ok(vec![1.0, 1.0]));
    assert_eq!(project_simplex(&[9.0, 9.0], 2.0), Ok(vec![1.0, 1.0]));
    assert_eq!(
        project_simplex(&[f64::MAX, f64::MAX], 2.0),
        Ok(vec![1.0, 1.0])
    );
    assert_eq!(
        project_simplex(&[f64::MAX, -f64::MAX], 1.0),
        Err(ProjectionError::Arithmetic)
    );
    assert_eq!(project_simplex(&[1.0, 2.0], 0.0), Ok(vec![0.0, 0.0]));
    assert_eq!(project_simplex(&[], 0.0), Ok(vec![]));
    assert_eq!(
        project_simplex(&[], 1.0),
        Err(ProjectionError::EmptySimplex)
    );
    assert_eq!(
        project_simplex(&[f64::NAN], 0.0),
        Err(ProjectionError::NonFinite)
    );
    assert_eq!(
        project_simplex(&[1.0], f64::INFINITY),
        Err(ProjectionError::NonFinite)
    );
    assert_eq!(
        project_simplex(&[1.0], -1.0),
        Err(ProjectionError::NegativeRadius)
    );
    assert_eq!(
        project_simplex(&[0.0, 0.0], f64::from_bits(1)),
        Err(ProjectionError::Arithmetic)
    );
    assert_eq!(
        project_simplex(&vec![0.0; MAX_LEN + 1], 1.0),
        Err(ProjectionError::TooLarge)
    );
}
