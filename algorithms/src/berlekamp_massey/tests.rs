use super::*;

// Independent row-reduction consistency test for the order-k Hankel system.
#[allow(clippy::needless_range_loop)] // Matrix coordinates keep elimination explicit.
fn feasible(s: &[u64], k: usize) -> bool {
    let mut rows: Vec<Vec<u64>> = (k..s.len())
        .map(|t| {
            let mut row: Vec<u64> = (0..k).map(|j| s[t - 1 - j] % MODULUS).collect();
            row.push(s[t] % MODULUS);
            row
        })
        .collect();
    let mut rank = 0;
    for col in 0..k {
        let Some(pivot) = (rank..rows.len()).find(|&r| rows[r][col] != 0) else {
            continue;
        };
        rows.swap(rank, pivot);
        // Independent extended Euclid inversion, not the production exponentiation.
        let (mut old_r, mut r, mut old_t, mut t) =
            (MODULUS as i128, rows[rank][col] as i128, 0_i128, 1_i128);
        while r != 0 {
            let q = old_r / r;
            (old_r, r) = (r, old_r - q * r);
            (old_t, t) = (t, old_t - q * t);
        }
        let inv = old_t.rem_euclid(MODULUS as i128) as u64;
        for j in col..=k {
            rows[rank][j] = rows[rank][j] * inv % MODULUS;
        }
        for i in 0..rows.len() {
            if i == rank {
                continue;
            }
            let factor = rows[i][col];
            for j in col..=k {
                rows[i][j] = (rows[i][j] + MODULUS - factor * rows[rank][j] % MODULUS) % MODULUS;
            }
        }
        rank += 1;
    }
    !rows
        .iter()
        .any(|r| r[..k].iter().all(|&v| v == 0) && r[k] != 0)
}
fn compatible(s: &[u64], c: &[u64]) -> bool {
    (c.len()..s.len()).all(|t| {
        let sum = c.iter().enumerate().fold(0_u128, |v, (j, &x)| {
            v + u128::from(x) * u128::from(s[t - 1 - j] % MODULUS)
        });
        (sum % u128::from(MODULUS)) as u64 == s[t] % MODULUS
    })
}
#[test]
fn exhaustive_gaussian_minimality() {
    for n in 0..=7 {
        for mut code in 0..3_usize.pow(n) {
            let s: Vec<u64> = (0..n)
                .map(|_| {
                    let v = code % 3;
                    code /= 3;
                    v as u64
                })
                .collect();
            let c = minimal_recurrence(&s).unwrap();
            assert!(compatible(&s, &c), "{s:?} {c:?}");
            let optimum = (0..=s.len()).find(|&k| feasible(&s, k)).unwrap();
            assert_eq!(c.len(), optimum, "{s:?}");
        }
    }
}
#[test]
fn lag_order_and_continuation_join() {
    let mut s = vec![2_u64, 7];
    for t in 2..24 {
        s.push((3 * s[t - 1] + 5 * s[t - 2]) % MODULUS);
    }
    let c = minimal_recurrence(&s[..12]).unwrap();
    assert_eq!(c, vec![3, 5]);
    for (n, &value) in s.iter().enumerate() {
        assert_eq!(
            crate::linear_recurrence::nth_term(&s[..2], &c, n as u64, MODULUS),
            Ok(value)
        );
    }
}
#[test]
fn zero_delayed_and_finite_ambiguity() {
    assert_eq!(minimal_recurrence(&[]), Ok(vec![]));
    assert_eq!(minimal_recurrence(&[0, 0, 0]), Ok(vec![]));
    assert_eq!(minimal_recurrence(&[0, 0, 1]).unwrap().len(), 3);
    // Both distinct order-two vectors explain this prefix; prediction is not identification.
    assert!(compatible(&[1, 2, 3], &[1, 1]));
    assert!(compatible(&[1, 2, 3], &[0, 3]));
    assert_ne!((3 + 2) % MODULUS, 6);
    assert_eq!(minimal_recurrence(&[1, 2, 3]).unwrap().len(), 2);
}
#[test]
fn normalization_and_limits() {
    let s = [u64::MAX, MODULUS, MODULUS - 1, 0, 7];
    let normalized: Vec<_> = s.iter().map(|v| v % MODULUS).collect();
    assert_eq!(minimal_recurrence(&s), minimal_recurrence(&normalized));
    assert!(compatible(&s, &minimal_recurrence(&s).unwrap()));
    assert_eq!(minimal_recurrence(&vec![0; MAX_TERMS]), Ok(vec![]));
    assert_eq!(
        minimal_recurrence(&vec![0; MAX_TERMS + 1]),
        Err(Error::TooManyTerms)
    );
}
