use super::*;
fn naive(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![0; a.len().max(b.len()).next_power_of_two()];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i ^ j] = ((u128::from(out[i ^ j])
                + u128::from(x % MODULUS) * u128::from(y % MODULUS))
                % u128::from(MODULUS)) as u64;
        }
    }
    out
}
#[test]
fn exhaustive_naive_xor() {
    let vectors: Vec<Vec<u64>> = (0..=4)
        .flat_map(|n| {
            (0..3_usize.pow(n)).map(move |mut code| {
                (0..n)
                    .map(|_| {
                        let x = code % 3;
                        code /= 3;
                        x as u64
                    })
                    .collect()
            })
        })
        .collect();
    for a in &vectors {
        for b in &vectors {
            assert_eq!(xor_convolution(a, b).unwrap(), naive(a, b));
        }
    }
}
#[test]
fn matrix_and_inverse() {
    for n in [1_usize, 2, 4, 8, 32, 128] {
        let a: Vec<u64> = (0..n).map(|i| u64::MAX - i as u64 * 19).collect();
        let expected: Vec<u64> = (0..n)
            .map(|k| {
                a.iter().enumerate().fold(0, |acc, (i, &v)| {
                    if (i & k).count_ones() % 2 == 0 {
                        (acc + v % MODULUS) % MODULUS
                    } else {
                        (acc + MODULUS - v % MODULUS) % MODULUS
                    }
                })
            })
            .collect();
        let mut transformed = a.clone();
        transform(&mut transformed, false).unwrap();
        assert_eq!(transformed, expected);
        transform(&mut transformed, true).unwrap();
        assert_eq!(
            transformed,
            a.iter().map(|v| v % MODULUS).collect::<Vec<_>>()
        );
    }
}
#[test]
fn padding_identity_and_boundaries() {
    assert_eq!(xor_convolution(&[1, 2, 3], &[1]), Ok(vec![1, 2, 3, 0]));
    assert_eq!(xor_convolution(&[0, 1], &[0, 1]), Ok(vec![1, 0]));
    assert_eq!(
        xor_convolution(&[u64::MAX], &[u64::MAX]).unwrap(),
        naive(&[u64::MAX], &[u64::MAX])
    );
    let mut invalid = vec![u64::MAX; 3];
    let saved = invalid.clone();
    assert_eq!(transform(&mut invalid, false), Err(Error::InvalidLength));
    assert_eq!(invalid, saved);
    assert_eq!(transform(&mut [], true), Err(Error::InvalidLength));
    assert_eq!(
        xor_convolution(&[], &vec![0; MAX_LEN + 1]),
        Err(Error::LengthTooLarge)
    );
    let mut too_large = vec![0; MAX_LEN * 2];
    assert_eq!(transform(&mut too_large, false), Err(Error::LengthTooLarge));
}
#[test]
fn bilinearity_and_mass() {
    let (a, b, c) = ([1, 4, 9], [2, 8, 3], [5, 7, 1]);
    let sum: Vec<_> = a.iter().zip(b).map(|(x, y)| x + y).collect();
    let ac = xor_convolution(&a, &c).unwrap();
    let bc = xor_convolution(&b, &c).unwrap();
    assert_eq!(
        xor_convolution(&sum, &c).unwrap(),
        ac.iter()
            .zip(bc)
            .map(|(x, y)| (x + y) % MODULUS)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        ac.iter().sum::<u64>() % MODULUS,
        a.iter().sum::<u64>() * c.iter().sum::<u64>() % MODULUS
    );
}
