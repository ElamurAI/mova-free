use super::*;

fn paths(n: usize, t: usize) -> Vec<Vec<usize>> {
    (0..n.pow(t as u32))
        .map(|mut k| {
            (0..t)
                .map(|_| {
                    let s = k % n;
                    k /= n;
                    s
                })
                .collect()
        })
        .collect()
}
fn probability(path: &[usize], initial: &[f64], a: &[Vec<f64>], b: &[Vec<f64>]) -> f64 {
    let mut p = initial[path[0]] * b[0][path[0]];
    for t in 1..path.len() {
        p *= a[path[t - 1]][path[t]] * b[t][path[t]];
    }
    p
}

#[test]
fn exhaustive_paths_oracle() {
    for q in 0..=4 {
        for r in 0..=4 {
            let initial = [0.25, 0.75];
            let a = vec![
                vec![q as f64 / 4.0, 1.0 - q as f64 / 4.0],
                vec![r as f64 / 4.0, 1.0 - r as f64 / 4.0],
            ];
            for mask in 0..64 {
                let b: Vec<Vec<f64>> = (0..3)
                    .map(|t| {
                        (0..2)
                            .map(|s| {
                                if mask & (1 << (2 * t + s)) == 0 {
                                    0.25
                                } else {
                                    0.75
                                }
                            })
                            .collect()
                    })
                    .collect();
                let mut total = 0.0;
                let mut masses = [[0.0; 2]; 3];
                for path in paths(2, 3) {
                    let p = probability(&path, &initial, &a, &b);
                    total += p;
                    for t in 0..3 {
                        masses[t][path[t]] += p;
                    }
                }
                let out = forward_backward(&initial, &a, &b).unwrap().unwrap();
                assert!(
                    (out.log_likelihood.exp() - total).abs() < 1e-14,
                    "enumeration likelihood"
                );
                for (t, row) in masses.iter().enumerate() {
                    for (s, mass) in row.iter().enumerate() {
                        assert!(
                            (out.states[t][s] - mass / total).abs() < 1e-13,
                            "enumeration posterior"
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn empty_impossible_and_underflow() {
    let a = vec![vec![0.5, 0.5]; 2];
    let out = forward_backward(&[0.5, 0.5], &a, &vec![vec![1.0, 1.0]; 4])
        .unwrap()
        .unwrap();
    assert_eq!(out.states, vec![vec![0.5, 0.5]; 4]);
    assert_eq!(
        forward_backward(&[0.5, 0.5], &a, &[])
            .unwrap()
            .unwrap()
            .log_likelihood,
        0.0
    );
    assert_eq!(
        forward_backward(&[0.5, 0.5], &a, &[vec![0.0, 0.0]]),
        Ok(None)
    );
    let out = forward_backward(&[1.0], &[vec![1.0]], &vec![vec![1e-200]; 4096])
        .unwrap()
        .unwrap();
    assert!(out.log_likelihood.is_finite());
    assert!((out.log_likelihood - 4096.0 * 1e-200f64.ln()).abs() < 1e-6);
    assert!(out.states.iter().all(|row| row == &[1.0]));
}
#[test]
fn validation_precedence() {
    assert_eq!(forward_backward(&[], &[], &[]), Err(Error::Dimension));
    assert_eq!(
        forward_backward(&[f64::NAN], &[], &[]),
        Err(Error::Dimension)
    );
    assert_eq!(
        forward_backward(&[f64::NAN], &[vec![1.0]], &[]),
        Err(Error::Probability)
    );
    assert_eq!(
        forward_backward(&[0.5], &[vec![1.0]], &[]),
        Err(Error::Normalization)
    );
    assert_eq!(
        forward_backward(&[1.0], &[vec![1.0]], &vec![vec![1.0]; 4097]),
        Err(Error::Dimension)
    );
    assert_eq!(
        forward_backward(&[1.0], &[vec![1.0]], &[vec![-1.0]]),
        Err(Error::Probability)
    );
}
