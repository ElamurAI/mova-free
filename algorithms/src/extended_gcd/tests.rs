use super::{Bezout, extended_gcd};

fn assert_identity(a: u64, b: u64) {
    let result = extended_gcd(a, b);
    let left = i128::from(a).checked_mul(result.x).unwrap();
    let right = i128::from(b).checked_mul(result.y).unwrap();
    assert_eq!(left.checked_add(right), Some(i128::from(result.gcd)));
    assert!(result.x.abs() <= i128::from(a.max(b)));
    assert!(result.y.abs() <= i128::from(a.max(b)));
    if result.gcd != 0 {
        assert_eq!(a % result.gcd, 0);
        assert_eq!(b % result.gcd, 0);
    }
}

#[test]
fn bounded_divisor_oracle() {
    for a in 0..=96 {
        for b in 0..=96 {
            let expected = (1..=a.max(b))
                .filter(|d| a % d == 0 && b % d == 0)
                .max()
                .unwrap_or(0);
            assert_eq!(extended_gcd(a, b).gcd, expected);
            assert_identity(a, b);
        }
    }
}

#[test]
fn zero_and_full_width_coefficients() {
    assert_eq!(extended_gcd(0, 0), Bezout { gcd: 0, x: 0, y: 0 });
    for value in [1, 2, 17, u64::MAX] {
        assert_eq!(
            extended_gcd(value, 0),
            Bezout {
                gcd: value,
                x: 1,
                y: 0
            }
        );
        assert_eq!(
            extended_gcd(0, value),
            Bezout {
                gcd: value,
                x: 0,
                y: 1
            }
        );
    }
    for a in [0, 1, 2, 3, 1 << 63, u64::MAX - 1, u64::MAX] {
        for b in [0, 1, 2, 3, 1 << 63, u64::MAX - 1, u64::MAX] {
            assert_identity(a, b);
        }
    }
    assert_eq!(extended_gcd(u64::MAX, u64::MAX - 1).gcd, 1);
}

#[test]
fn fibonacci_and_deterministic_full_width() {
    let (mut a, mut b) = (1_u64, 1_u64);
    while let Some(next) = a.checked_add(b) {
        assert_identity(a, b);
        assert_eq!(extended_gcd(a, b).gcd, 1);
        (a, b) = (b, next);
    }
    let mut state = 0x2f0a_16c9_03d1_8b65_u64;
    for _ in 0..4096 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a = state;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let b = state;
        assert_identity(a, b);
        assert_eq!(extended_gcd(a, b).gcd, extended_gcd(b, a).gcd);
    }
}

#[test]
fn affine_family_is_not_the_coefficient_policy() {
    for (a, b) in [(30_u64, 18_u64), (17, 31), (0, 9), (9, 0), (7, 7)] {
        let result = extended_gcd(a, b);
        let da = i128::from(a / result.gcd);
        let db = i128::from(b / result.gcd);
        for k in [-3_i128, -1, 1, 4] {
            let candidate = Bezout {
                gcd: result.gcd,
                x: result.x + k * db,
                y: result.y - k * da,
            };
            assert_eq!(
                i128::from(a) * candidate.x + i128::from(b) * candidate.y,
                i128::from(result.gcd)
            );
            assert_ne!(candidate, result);
        }
    }
    // Equal inputs pin the recurrence's selected representative.
    assert_eq!(extended_gcd(7, 7), Bezout { gcd: 7, x: 0, y: 1 });
}

#[test]
fn common_divisors_and_linear_solvability() {
    for (a, b) in [(30_u64, 18_u64), (84, 126), (35, 64), (0, 12)] {
        let result = extended_gcd(a, b);
        let divisors = |n: u64| {
            (1..=a.max(b))
                .filter(move |d| n.is_multiple_of(*d))
                .collect::<Vec<_>>()
        };
        let common = divisors(a)
            .into_iter()
            .filter(|d| b.is_multiple_of(*d))
            .collect::<Vec<_>>();
        assert_eq!(common, divisors(result.gcd));
        for c in [0_u64, 1, 6, 12, 42] {
            let found = (-256_i128..=256).any(|x| {
                (-256_i128..=256).any(|y| i128::from(a) * x + i128::from(b) * y == i128::from(c))
            });
            assert_eq!(found, c.is_multiple_of(result.gcd));
        }
    }
}

#[test]
fn modular_inverse_projection_with_domain_premises() {
    for (a, modulus) in [(3_u64, 11_u64), (6, 15), (17, 31), (0, 7), (1, 2)] {
        let result = extended_gcd(a, modulus);
        let oracle =
            (0..modulus).find(|&r| (u128::from(a) * u128::from(r)) % u128::from(modulus) == 1);
        if result.gcd == 1 {
            let residue = result.x.rem_euclid(i128::from(modulus)) as u64;
            assert_eq!(Some(residue), oracle);
        } else {
            assert_eq!(oracle, None);
        }
    }
    let modulus = u64::MAX;
    let a = modulus - 1;
    let result = extended_gcd(a, modulus);
    let residue = result.x.rem_euclid(i128::from(modulus)) as u64;
    assert_eq!(residue, modulus - 1);
    assert_eq!(
        (u128::from(a) * u128::from(residue)) % u128::from(modulus),
        1
    );
    assert!(i128::from(a).checked_mul(i128::from(residue)).is_none());
}
