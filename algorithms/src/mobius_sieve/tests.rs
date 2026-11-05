use super::*;

fn trial_mu(mut n: usize) -> i8 {
    if n == 0 {
        return 0;
    }
    let mut sign = 1;
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            n /= d;
            sign = -sign;
            if n.is_multiple_of(d) {
                return 0;
            }
        }
        d += 1;
    }
    if n > 1 { -sign } else { sign }
}

#[test]
fn exhaustive_factor_oracle() {
    let mu = mobius_sieve(10_000).unwrap();
    for (n, &value) in mu.iter().enumerate() {
        assert_eq!(value, trial_mu(n), "n={n}");
    }
}

#[test]
fn prefixes_and_square_factors() {
    assert_eq!(mobius_sieve(0), Ok(vec![0]));
    assert_eq!(mobius_sieve(1), Ok(vec![0, 1]));
    let full = mobius_sieve(128).unwrap();
    for n in 0..=128 {
        assert_eq!(mobius_sieve(n).unwrap(), full[..=n]);
    }
    assert_eq!(&full[..11], &[0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1]);
    for square in [4, 9, 25, 49] {
        for n in (square..=128).step_by(square) {
            assert_eq!(full[n], 0);
        }
    }
}

#[test]
fn limit_and_divisor_identity() {
    for n in [MAX_N + 1, usize::MAX] {
        assert_eq!(
            mobius_sieve(n),
            Err(MobiusError::LimitExceeded {
                requested: n,
                maximum: MAX_N
            })
        );
    }
    let mu = mobius_sieve(MAX_N).unwrap();
    assert_eq!(mu[MAX_N], 0);
    for n in 1..=512 {
        assert_eq!(
            (1..=n)
                .filter(|d| n % d == 0)
                .map(|d| i32::from(mu[d]))
                .sum::<i32>(),
            i32::from(n == 1)
        );
    }
}
