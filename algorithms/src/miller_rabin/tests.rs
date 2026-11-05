use super::is_prime;

fn trial(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

#[test]
fn exhaustive_trial_division() {
    for n in 0..100_000 {
        assert_eq!(is_prime(n), trial(n), "n={n}");
    }
}

#[test]
fn strong_pseudoprime_boundaries() {
    for n in [
        0,
        1,
        4,
        9,
        25,
        341,
        561,
        1105,
        1729,
        2047,
        1_373_653,
        25_326_001,
        3_215_031_751,
        2_152_302_898_747,
        341_550_071_728_321,
        3_825_123_056_546_413_051,
        u64::MAX,
        4_294_967_291u64 * 4_294_967_291,
    ] {
        assert!(!is_prime(n), "composite {n}");
    }
}

#[test]
fn primes_and_large_products() {
    for n in [
        2,
        3,
        5,
        73,
        193,
        407521,
        4_294_967_291,
        18_446_744_073_709_551_557,
    ] {
        assert!(is_prime(n), "prime {n}");
    }
    for a in [3u64, 17, 65537, 4_294_967_291] {
        for b in [3u64, 19, 65539, 4_294_967_291] {
            assert!(!is_prime(a * b));
        }
    }
}

#[test]
fn boolean_fibers_and_complement() {
    let primes = (0..50).filter(|&n| is_prime(n)).collect::<Vec<_>>();
    assert_eq!(
        primes,
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    );
    for n in 0..50 {
        assert_ne!(is_prime(n), !trial(n));
    }
}
