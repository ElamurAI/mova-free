use super::{Error, factorize};

fn is_prime(n: u64) -> bool {
    n >= 2 && (2..n).all(|d| !n.is_multiple_of(d))
}

#[test]
fn bounded_independent_divisor_oracle() {
    for n in 1_u64..=2048 {
        let expected: Vec<_> = (2..=n)
            .filter(|&p| n.is_multiple_of(p) && is_prime(p))
            .map(|p| {
                let mut power = p;
                let mut e = 1;
                while let Some(next) = power.checked_mul(p) {
                    if !n.is_multiple_of(next) {
                        break;
                    }
                    power = next;
                    e += 1;
                }
                (p, e)
            })
            .collect();
        assert_eq!(factorize(n), Ok(expected), "n={n}");
    }
}

#[test]
fn zero_one_and_prime_squares() {
    assert_eq!(factorize(0), Err(Error::ZeroInput));
    assert_eq!(factorize(1), Ok(vec![]));
    for p in [2, 3, 5, 7, 31, 97, 257, 65537] {
        assert_eq!(factorize(p * p), Ok(vec![(p, 2)]));
    }
}

#[test]
fn full_width_and_exponents() {
    assert_eq!(
        factorize(u64::MAX),
        Ok(vec![
            (3, 1),
            (5, 1),
            (17, 1),
            (257, 1),
            (641, 1),
            (65537, 1),
            (6700417, 1)
        ])
    );
    for e in 1..=63 {
        assert_eq!(factorize(1_u64 << e), Ok(vec![(2, e)]));
    }
    assert_eq!(factorize(3_u64.pow(40)), Ok(vec![(3, 40)]));
}

#[test]
fn reconstruction_and_unique_normal_form() {
    for n in 1..=4096 {
        let factors = factorize(n).unwrap();
        let product = factors
            .iter()
            .fold(1_u128, |a, &(p, e)| a * u128::from(p).pow(e));
        assert_eq!(product, u128::from(n));
        assert!(factors.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(factors.iter().all(|&(p, e)| e > 0 && is_prime(p)));
    }
    // A product alone does not certify primality or ascending normalization.
    assert_eq!(2_u64.pow(2) * 3, 4 * 3);
    assert_ne!(factorize(12).unwrap(), vec![(4, 1), (3, 1)]);
}

#[test]
fn multiplication_adds_prime_valuations() {
    for a in 1..=64 {
        for b in 1..=64 {
            let mut joined = std::collections::BTreeMap::new();
            for (p, e) in factorize(a)
                .unwrap()
                .into_iter()
                .chain(factorize(b).unwrap())
            {
                *joined.entry(p).or_insert(0) += e;
            }
            assert_eq!(
                factorize(a * b).unwrap(),
                joined.into_iter().collect::<Vec<_>>()
            );
        }
    }
}
