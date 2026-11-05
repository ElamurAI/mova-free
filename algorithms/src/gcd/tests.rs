use super::*;

#[test]
fn bounded_divisor_oracle() {
    for a in 0..=128 {
        for b in 0..=128 {
            let expected = if a == 0 && b == 0 {
                0
            } else {
                (1..=a.max(b))
                    .rev()
                    .find(|d| a % d == 0 && b % d == 0)
                    .unwrap()
            };
            assert_eq!(gcd(a, b), expected, "a={a}, b={b}");
        }
    }
}

#[test]
fn zero_extremes_and_long_chain() {
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(0, u64::MAX), u64::MAX);
    assert_eq!(gcd(u64::MAX, 0), u64::MAX);
    assert_eq!(gcd(u64::MAX, u64::MAX), u64::MAX);
    assert_eq!(gcd(u64::MAX, u64::MAX - 1), 1);
    assert_eq!(gcd(u64::MAX, 3), 3);
    assert_eq!(gcd(1 << 63, 1 << 62), 1 << 62);
    assert_eq!(gcd(12200160415121876738, 7540113804746346429), 1);
}
fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shared = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            return a << shared;
        }
    }
}

#[test]
fn full_width_binary_oracle() {
    let mut seed = 0xbb67ae8584caa73bu64;
    for case in 0..4096 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let a = seed;
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let b = seed.rotate_left(23);
        assert_eq!(gcd(a, b), binary_gcd(a, b), "case={case}, a={a}, b={b}");
        assert_eq!(gcd(b, a), binary_gcd(a, b), "swapped case={case}");
    }
    for a in [0, 1, 1 << 63, u64::MAX] {
        for b in [0, 1, 1 << 63, u64::MAX] {
            assert_eq!(gcd(a, b), binary_gcd(a, b), "a={a}, b={b}");
        }
    }
}

#[test]
fn constructed_coprime_scaled_pairs() {
    // Consecutive integers are coprime; scaling both gives exactly the factor.
    for factor in [1, 2, 3, 255, 65537, 1 << 32, u64::MAX / 3] {
        for x in [1, 2, 3, 31, 65535, u32::MAX as u64] {
            if let (Some(a), Some(b)) = (factor.checked_mul(x), factor.checked_mul(x + 1)) {
                assert_eq!(gcd(a, b), factor, "factor={factor}, x={x}");
                assert_eq!(gcd(b, a), factor, "swapped factor={factor}, x={x}");
            }
        }
    }
}
