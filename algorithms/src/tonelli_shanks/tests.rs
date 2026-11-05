use super::{ModulusError, sqrt_mod};

fn trial(n: u64) -> bool {
    n >= 2 && !(2..n).any(|d| n.is_multiple_of(d))
}

#[test]
fn exhaustive_root_enumeration() {
    for p in (2..200).filter(|&p| trial(p)) {
        for a in 0..3 * p {
            let expected = (0..p).find(|&x| x * x % p == a % p);
            assert_eq!(sqrt_mod(a, p), Ok(expected), "a={a}, p={p}");
        }
    }
}

#[test]
fn validation_precedes_zero_shortcut() {
    for a in [0, 1, u64::MAX] {
        for p in [0, 1] {
            assert_eq!(sqrt_mod(a, p), Err(ModulusError::InvalidModulus));
        }
        for p in [4, 9, 15, 561, 341_550_071_728_321, u64::MAX] {
            assert_eq!(sqrt_mod(a, p), Err(ModulusError::CompositeModulus));
        }
    }
}

#[test]
fn large_prime_algebraic_roots() {
    for p in [
        257u64,
        65537,
        998244353,
        4_294_967_291,
        18_446_744_073_709_551_557,
    ] {
        for x in [0, 1, 2, 3, p / 2, p - 2, p - 1] {
            let a = (u128::from(x) * u128::from(x) % u128::from(p)) as u64;
            assert_eq!(sqrt_mod(a, p), Ok(Some(x.min(p - x))));
        }
        let a = u64::MAX;
        if let Some(r) = sqrt_mod(a, p).unwrap() {
            assert!(r <= p - r);
            assert_eq!(
                u128::from(r) * u128::from(r) % u128::from(p),
                u128::from(a % p)
            );
        }
    }
}

#[test]
fn normalization_fibers_and_nonresidue() {
    assert_eq!(sqrt_mod(u64::MAX, 2), Ok(Some(1)));
    assert_eq!(sqrt_mod(3, 7), Ok(None));
    assert_eq!(sqrt_mod(4, 7), Ok(Some(2)));
    for a in [4, 11, 18, 25] {
        assert_eq!(sqrt_mod(a, 7), Ok(Some(2)));
    }
}
