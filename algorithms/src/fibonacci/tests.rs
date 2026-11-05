use super::*;

fn add(a: u64, b: u64, m: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(m)) as u64
}
fn matrix(n: u64, m: u64) -> u64 {
    fn mul(a: [u64; 4], b: [u64; 4], m: u64) -> [u64; 4] {
        let mut c = [0; 4];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let term = (u128::from(a[i * 2 + k]) * u128::from(b[k * 2 + j]) % u128::from(m))
                        as u64;
                    c[i * 2 + j] = add(c[i * 2 + j], term, m);
                }
            }
        }
        c
    }
    let mut exponent = n;
    let mut base = [1 % m, 1 % m, 1 % m, 0];
    let mut result = [1 % m, 0, 0, 1 % m];
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul(result, base, m);
        }
        base = mul(base, base, m);
        exponent >>= 1;
    }
    result[1]
}

#[test]
fn exhaustive_iterative_oracle() {
    for m in (1..=128).chain([u64::MAX, u64::MAX - 1]) {
        let (mut a, mut b) = (0, 1 % m);
        for n in 0..=1024 {
            assert_eq!(fibonacci_mod(n, m), Ok(a), "n={n},m={m}");
            (a, b) = (b, add(a, b, m));
        }
    }
}

#[test]
fn full_width_matrix_oracle() {
    for n in [0, 1, 2, 93, 94, 1 << 32, 1 << 63, u64::MAX - 1, u64::MAX] {
        for m in [1, 2, 3, 1_000_000_007, 1 << 63, u64::MAX - 1, u64::MAX] {
            assert_eq!(fibonacci_mod(n, m), Ok(matrix(n, m)), "n={n},m={m}");
        }
    }
}

#[test]
fn errors_and_large_recurrence() {
    for n in [0, 1, u64::MAX] {
        assert_eq!(fibonacci_mod(n, 0), Err(FibonacciError::ZeroModulus));
        assert_eq!(fibonacci_mod(n, 1), Ok(0));
    }
    for n in [1 << 32, 1 << 63, u64::MAX - 2] {
        for m in [2, 97, u64::MAX] {
            assert_eq!(
                fibonacci_mod(n + 2, m).unwrap(),
                add(
                    fibonacci_mod(n, m).unwrap(),
                    fibonacci_mod(n + 1, m).unwrap(),
                    m
                )
            );
        }
    }
}
