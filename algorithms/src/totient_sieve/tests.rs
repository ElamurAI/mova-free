use super::*;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[test]
fn exhaustive_gcd_oracle() {
    let phi = totient_sieve(2048).unwrap();
    for (n, &actual) in phi.iter().enumerate().skip(1) {
        let expected = (1..=n).filter(|&k| gcd(k, n) == 1).count();
        assert_eq!(actual, expected, "n={n}");
    }
}

#[test]
fn prefixes_and_conventions() {
    assert_eq!(totient_sieve(0), Ok(vec![0]));
    assert_eq!(totient_sieve(1), Ok(vec![0, 1]));
    let full = totient_sieve(128).unwrap();
    for n in 0..=128 {
        assert_eq!(totient_sieve(n).unwrap(), full[..=n]);
    }
    assert_eq!(&full[..11], &[0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
}

#[test]
fn limit_and_divisor_identity() {
    for n in [MAX_N + 1, usize::MAX] {
        assert_eq!(
            totient_sieve(n),
            Err(TotientError::LimitExceeded {
                requested: n,
                maximum: MAX_N
            })
        );
    }
    let phi = totient_sieve(MAX_N).unwrap();
    assert_eq!(phi[MAX_N], 400_000);
    for n in 1..=512 {
        assert_eq!(
            (1..=n)
                .filter(|d| n % d == 0)
                .map(|d| phi[d])
                .sum::<usize>(),
            n
        );
    }
}
