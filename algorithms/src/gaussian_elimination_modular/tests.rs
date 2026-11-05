use super::*;
fn dot(a: &[u64], b: &[u64]) -> u64 {
    a.iter()
        .zip(b)
        .fold(0, |s, (&x, &y)| (s + (x % MODULUS) * y) % MODULUS)
}
#[test]
fn direct_oracle() {
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                for w in 0..3 {
                    let a = vec![vec![x, y], vec![z, w]];
                    let b = vec![(2 * x + 3 * y) % MODULUS, (2 * z + 3 * w) % MODULUS];
                    let sol = solve(&a, &b, 2).unwrap().unwrap();
                    for (r, &rhs) in a.iter().zip(&b) {
                        assert_eq!(dot(r, &sol.particular), rhs, "particular residual");
                        for v in &sol.nullspace {
                            assert_eq!(dot(r, v), 0, "nullspace residual");
                        }
                    }
                    let det = (x * w + MODULUS - y * z) % MODULUS;
                    let rank = if det != 0 {
                        2
                    } else if x + y + z + w != 0 {
                        1
                    } else {
                        0
                    };
                    assert_eq!(sol.nullspace.len(), 2 - rank, "determinant oracle");
                    if rank == 2 {
                        assert_eq!(sol.particular, vec![2, 3]);
                    }
                }
            }
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(solve(&[vec![0]], &[1], 1), Ok(None));
    assert_eq!(solve(&[vec![1], vec![1]], &[1, 2], 1), Ok(None));
    assert_eq!(
        solve(&[vec![]], &[0], 0).unwrap().unwrap().particular,
        vec![]
    );
    assert_eq!(
        solve(&[], &[], 2).unwrap().unwrap().nullspace,
        vec![vec![1, 0], vec![0, 1]]
    );
    assert_eq!(solve(&[vec![1]], &[], 1), Err(Error::Shape));
    assert_eq!(solve(&[], &[], 129), Err(Error::TooLarge));
    let a = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(
        solve(&a, &[3, 2], 2).unwrap().unwrap().particular,
        vec![2, 3]
    );
    assert_eq!(
        solve(&[vec![MODULUS + 1]], &[u64::MAX], 1)
            .unwrap()
            .unwrap()
            .particular,
        vec![u64::MAX % MODULUS]
    );
}
