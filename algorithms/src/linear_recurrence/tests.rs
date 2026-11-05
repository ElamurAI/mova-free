use super::*;

fn iterative(initial: &[u64], c: &[u64], n: usize, m: u64) -> u64 {
    let mut terms: Vec<u64> = initial.iter().map(|x| x % m).collect();
    for t in terms.len()..=n {
        let mut value = 0u128;
        for j in 0..c.len() {
            value = (value + u128::from(c[j] % m) * u128::from(terms[t - 1 - j])) % u128::from(m);
        }
        terms.push(value as u64);
    }
    terms[n]
}

#[test]
fn finite_iterative_oracle() {
    for k in 1..=3 {
        for code in 0..3usize.pow((2 * k) as u32) {
            let mut digits = code;
            let mut input = vec![0; 2 * k];
            for x in &mut input {
                *x = (digits % 3) as u64;
                digits /= 3;
            }
            let (initial, c) = input.split_at(k);
            for m in [1, 2, 7, 16, u64::MAX] {
                for n in 0..=16 {
                    assert_eq!(
                        nth_term(initial, c, n as u64, m),
                        Ok(iterative(initial, c, n, m))
                    );
                }
            }
        }
    }
}

#[test]
fn validation_precedence_and_zero_index() {
    assert_eq!(nth_term(&[], &[], 0, 0), Err(Error::ZeroModulus));
    assert_eq!(nth_term(&[1], &[], 0, 1), Err(Error::EmptyOrder));
    assert_eq!(nth_term(&[], &[1], 0, 1), Err(Error::DimensionMismatch));
    let over = vec![0; MAX_ORDER + 1];
    assert_eq!(nth_term(&over, &over, 0, 1), Err(Error::OrderTooLarge));
    let max = vec![u64::MAX; MAX_ORDER];
    assert_eq!(nth_term(&max, &max, 0, 17), Ok(u64::MAX % 17));
    assert_eq!(nth_term(&[13], &[99], 0, 5), Ok(3));
}

#[test]
fn full_width_products_and_huge_index() {
    let m = u64::MAX;
    let initial = [m - 1, m - 2, m - 3];
    let c = [m - 1, m - 7, m - 11];
    for n in 0..80 {
        assert_eq!(
            nth_term(&initial, &c, n as u64, m),
            Ok(iterative(&initial, &c, n, m))
        );
    }
    assert_eq!(nth_term(&[1], &[m - 1], u64::MAX, m), Ok(m - 1));
    assert_eq!(nth_term(&[1], &[m - 1], u64::MAX - 1, m), Ok(1));
    assert_eq!(nth_term(&[9], &[7], u64::MAX, 1), Ok(0));
    assert_eq!(nth_term(&[0, 1], &[1, 1], 10, 1000), Ok(55));
}

#[test]
fn linearity_normalization_and_ambiguous_fibers() {
    for n in 0..40 {
        let a = nth_term(&[2, 3], &[2, 1], n, 97).unwrap();
        let b = nth_term(&[5, 7], &[2, 1], n, 97).unwrap();
        assert_eq!(nth_term(&[7, 10], &[2, 1], n, 97), Ok((a + b) % 97));
        assert_eq!(nth_term(&[99, 100], &[99, 98], n, 97), Ok(a));
    }
    // Same result does not identify seeds or coefficients.
    assert_eq!(nth_term(&[0, 0], &[1, 1], 80, 97), Ok(0));
    assert_eq!(nth_term(&[0, 0], &[9, 4], 80, 97), Ok(0));
    assert_eq!(nth_term(&[4, 8], &[0, 0], 2, 97), Ok(0));
}
