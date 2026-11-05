use super::*;

fn assert_root(n: u64) {
    let r = integer_sqrt(n);
    assert_eq!(r, n.isqrt(), "n={n}");
    let wide = u128::from(r);
    assert!(wide * wide <= u128::from(n), "lower bound n={n}, r={r}");
    assert!(
        u128::from(n) < (wide + 1) * (wide + 1),
        "upper bound n={n}, r={r}"
    );
}

#[test]
fn bounded_standard_oracle() {
    for n in 0..=65536 {
        assert_root(n);
    }
}

#[test]
fn square_neighbors_and_maximum() {
    for r in [0u64, 1, 2, 3, 255, 65535, 65536, 1 << 31, u32::MAX as u64] {
        let square = r * r;
        assert_root(square);
        assert_eq!(integer_sqrt(square), r, "r={r}");
        if square > 0 {
            assert_eq!(integer_sqrt(square - 1), r - 1, "r={r}");
        }
        if let Some(next) = square.checked_add(1) {
            assert_root(next);
        }
    }
    assert_eq!(integer_sqrt(u64::MAX), u32::MAX as u64);
}

#[test]
fn fixed_seed_full_width_and_monotonicity() {
    let mut state = 0x3c6ef372fe94f82bu64;
    for case in 0..4096 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        assert_root(state);
        if let Some(next) = state.checked_add(1) {
            let a = integer_sqrt(state);
            let b = integer_sqrt(next);
            assert!(a <= b && b <= a + 1, "case={case}, n={state}");
        }
    }
}
