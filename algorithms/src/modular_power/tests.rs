use super::*;

fn repeated(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1u128 % u128::from(modulus);
    for _ in 0..exponent {
        result = result * u128::from(base) % u128::from(modulus);
    }
    result as u64
}

#[test]
fn bounded_repeated_multiply_oracle() {
    for base in 0..=24 {
        for exponent in 0..=16 {
            for modulus in 1..=24 {
                assert_eq!(
                    modular_power(base, exponent, modulus),
                    Ok(repeated(base, exponent, modulus)),
                    "b={base}, e={exponent}, m={modulus}"
                );
            }
        }
    }
}

#[test]
fn zero_identity_and_full_width_products() {
    for base in [0, 1, 2, 1 << 63, u64::MAX] {
        for exponent in [0, 1, 2, u64::MAX] {
            assert_eq!(modular_power(base, exponent, 0), Err(ZeroModulus));
            assert_eq!(modular_power(base, exponent, 1), Ok(0));
        }
        assert_eq!(modular_power(base, 0, u64::MAX), Ok(1));
    }
    for modulus in [2, 3, 1 << 63, u64::MAX - 1, u64::MAX] {
        for exponent in [0, 1, 2, 3, u64::MAX - 1, u64::MAX] {
            let expected = if exponent % 2 == 0 { 1 } else { modulus - 1 };
            assert_eq!(
                modular_power(modulus - 1, exponent, modulus),
                Ok(expected),
                "e={exponent}, m={modulus}"
            );
        }
    }
    assert_eq!(modular_power(u64::MAX, 2, u64::MAX - 1), Ok(1));
    assert_eq!(modular_power(u64::MAX - 2, 3, u64::MAX), Ok(u64::MAX - 8));
}

#[test]
fn fixed_seed_reduction_and_exponent_addition() {
    let mut state = 0xa54ff53a5f1d36f1u64;
    for case in 0..1024 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let base = state;
        let modulus = state.rotate_left(29) | 1;
        let a = (state >> 58) & 31;
        let b = (state >> 52) & 31;
        assert_eq!(
            modular_power(base, a, modulus),
            Ok(repeated(base, a, modulus)),
            "case={case}"
        );
        assert_eq!(
            modular_power(base, a, modulus),
            modular_power(base % modulus, a, modulus),
            "reduction case={case}"
        );
        let lhs = modular_power(base, a + b, modulus).unwrap();
        let x = modular_power(base, a, modulus).unwrap();
        let y = modular_power(base, b, modulus).unwrap();
        let rhs = (u128::from(x) * u128::from(y) % u128::from(modulus)) as u64;
        assert_eq!(lhs, rhs, "addition case={case}");
    }
}
