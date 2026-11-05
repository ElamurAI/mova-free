use super::*;

fn schoolbook(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = ((u128::from(out[i + j])
                + u128::from(x % MODULUS) * u128::from(y % MODULUS))
                % u128::from(MODULUS)) as u64;
        }
    }
    out
}

fn vectors() -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    for n in 0..=4 {
        for mut code in 0..3usize.pow(n) {
            let mut row = Vec::new();
            for _ in 0..n {
                row.push((code % 3) as u64);
                code /= 3;
            }
            result.push(row);
        }
    }
    result
}

#[test]
fn finite_schoolbook_oracle() {
    let inputs = vectors();
    for a in &inputs {
        for b in &inputs {
            assert_eq!(convolution(a, b).unwrap(), schoolbook(a, b));
        }
    }
}

#[test]
fn prime_root_order_and_transform_roundtrip() {
    // Complete trial division proves primality for this one fixed constant.
    assert!((2..=31_595u64).all(|d| !MODULUS.is_multiple_of(d)));
    // Prime factors of p-1 are 2, 7, 17; these establish root 3's full order.
    assert_eq!(pow(ROOT, MODULUS - 1), 1);
    for q in [2, 7, 17] {
        assert_ne!(pow(ROOT, (MODULUS - 1) / q), 1);
    }
    for exponent in 0..=23 {
        let n = 1usize << exponent;
        let root = pow(ROOT, (MODULUS - 1) / n as u64);
        assert_eq!(pow(root, n as u64), 1);
        if n > 1 {
            assert_ne!(pow(root, n as u64 / 2), 1);
        }
    }
    for exponent in 0..=10 {
        let mut input: Vec<u64> = (0..1usize << exponent)
            .map(|i| (i * i + 17) as u64 % MODULUS)
            .collect();
        let original = input.clone();
        transform(&mut input, false);
        transform(&mut input, true);
        assert_eq!(input, original);
    }
}

#[test]
fn boundaries_normalization_and_checked_lengths() {
    assert_eq!(convolution(&[], &[u64::MAX]), Ok(vec![]));
    assert_eq!(
        convolution(&[u64::MAX], &[u64::MAX]).unwrap(),
        schoolbook(&[u64::MAX], &[u64::MAX])
    );
    assert_eq!(convolution(&[1, 2, 3], &[4, 5]), Ok(vec![4, 13, 22, 15]));
    assert_eq!(
        lengths(MAX_TRANSFORM_LEN, 1),
        Ok((MAX_TRANSFORM_LEN, MAX_TRANSFORM_LEN))
    );
    assert_eq!(lengths(MAX_TRANSFORM_LEN, 2), Err(Error::TransformTooLarge));
    assert_eq!(lengths(usize::MAX, 2), Err(Error::LengthOverflow));
    assert_eq!(lengths(usize::MAX, 1), Err(Error::LengthOverflow));
    for n in [7, 8, 9, 31, 32, 33, 127] {
        let a: Vec<u64> = (0..n).map(|i| u64::MAX - i).collect();
        assert_eq!(convolution(&a, &a).unwrap(), schoolbook(&a, &a));
    }
}

#[test]
fn evaluation_mass_identity_and_ambiguous_factors() {
    let a = [4, 9, 3, 2];
    let b = [7, 0, 1];
    let product = convolution(&a, &b).unwrap();
    for x in 0..10 {
        let eval = |v: &[u64]| v.iter().rev().fold(0, |acc, &c| (acc * x + c) % MODULUS);
        assert_eq!(eval(&product), eval(&a) * eval(&b) % MODULUS);
    }
    assert_eq!(convolution(&a, &[1]), Ok(a.to_vec()));
    assert_eq!(convolution(&a, &b), convolution(&b, &a));
    assert_eq!(convolution(&[2], &[3]), convolution(&[1], &[6]));
    assert_eq!(convolution(&[0, 0], &a), Ok(vec![0; 5]));
}
