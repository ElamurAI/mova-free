use super::*;

#[test]
fn independent_checked_pascal_triangle() {
    let mut row = vec![Some(1u128)];
    for n in 0..=300 {
        for k in 0..=n {
            assert_eq!(
                binomial_coefficient(n, k),
                row[k as usize].ok_or(Error::Overflow),
                "n={n} k={k}"
            );
        }
        let mut next = vec![Some(1); row.len() + 1];
        for k in 1..row.len() {
            next[k] = row[k - 1].zip(row[k]).and_then(|(a, b)| a.checked_add(b));
        }
        row = next;
    }
}

#[test]
fn boundaries_and_invalid_domain() {
    assert_eq!(binomial_coefficient(0, 0), Ok(1));
    assert_eq!(binomial_coefficient(0, 1), Err(Error::InvalidK));
    assert_eq!(binomial_coefficient(u64::MAX, u64::MAX), Ok(1));
    assert_eq!(binomial_coefficient(u64::MAX, 1), Ok(u128::from(u64::MAX)));
    assert_eq!(
        binomial_coefficient(u64::MAX, 2),
        Ok(u128::from(u64::MAX) * u128::from(u64::MAX - 1) / 2)
    );
    assert_eq!(binomial_coefficient(u64::MAX, 3), Err(Error::Overflow));
    assert_eq!(
        binomial_coefficient(u64::MAX, u64::MAX / 2),
        Err(Error::Overflow)
    );
}

#[test]
fn cancellation_prevents_false_intermediate_overflow() {
    assert_eq!(
        binomial_coefficient(131, 65),
        Ok(188694833082770476622296176145946360850)
    );
    assert_eq!(binomial_coefficient(132, 66), Err(Error::Overflow));
}

#[test]
fn symmetry_and_pascal_identity() {
    for n in 1..=100 {
        for k in 1..n {
            let c = binomial_coefficient(n, k).unwrap();
            assert_eq!(c, binomial_coefficient(n, n - k).unwrap());
            assert_eq!(
                c,
                binomial_coefficient(n - 1, k - 1).unwrap()
                    + binomial_coefficient(n - 1, k).unwrap()
            );
        }
    }
}

#[test]
fn row_sum_and_known_fibers() {
    for n in 0..=100 {
        assert_eq!(
            (0..=n)
                .map(|k| binomial_coefficient(n, k).unwrap())
                .sum::<u128>(),
            1u128 << n
        );
    }
    assert_eq!(binomial_coefficient(5, 2), Ok(10));
    assert_eq!(binomial_coefficient(10, 1), Ok(10));
}
