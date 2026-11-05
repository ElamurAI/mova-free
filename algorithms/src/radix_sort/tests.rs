use super::{byte_pass, radix_sort};

#[test]
fn exhaustive_small_arrays() {
    let alphabet = [0, 255, 256, u64::MAX];
    for len in 0..=7 {
        for mut code in 0..4usize.pow(len) {
            let mut actual = vec![0; len as usize];
            for value in &mut actual {
                *value = alphabet[code % 4];
                code /= 4;
            }
            let mut expected = actual.clone();
            expected.sort();
            radix_sort(&mut actual).unwrap();
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn stable_byte_pass_matches_independent_oracle() {
    let source = [0x301, 0x101, 0x202, 0x001, u64::MAX, 0x102, 0x100];
    for byte in 0..8 {
        let mut expected = source;
        expected.sort_by_key(|value| value.to_le_bytes()[byte]);
        let mut actual = [0; 7];
        byte_pass(&source, &mut actual, byte);
        assert_eq!(actual, expected);
    }
}

#[test]
fn full_width_boundaries_and_idempotence() {
    let mut values = vec![u64::MAX, 0, 1, 1 << 63, u64::MAX - 1];
    for byte in 0..8 {
        values.extend([1 << (8 * byte), 255 << (8 * byte)]);
    }
    let mut expected = values.clone();
    expected.sort();
    radix_sort(&mut values).unwrap();
    assert_eq!(values, expected);
    radix_sort(&mut values).unwrap();
    assert_eq!(values, expected);
    radix_sort(&mut []).unwrap();
    radix_sort(&mut [u64::MAX]).unwrap();
}

#[test]
fn deterministic_large_oracle() {
    let mut seed = 17u64;
    let mut values: Vec<_> = (0..8192)
        .map(|_| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            seed
        })
        .collect();
    let mut expected = values.clone();
    expected.sort();
    radix_sort(&mut values).unwrap();
    assert_eq!(values, expected);
}

#[test]
fn byte_pass_preimages_are_subsequence_preserving_shuffles() {
    let target = [0x300, 0x100, 0x201, 0x001];
    let alphabet = target;
    let subsequences = |xs: &[u64], digit: u8| {
        xs.iter()
            .copied()
            .filter(|x| x.to_le_bytes()[0] == digit)
            .collect::<Vec<_>>()
    };
    let mut fiber_size = 0;
    for mut code in 0..4usize.pow(4) {
        let mut input = [0; 4];
        for x in &mut input {
            *x = alphabet[code % 4];
            code /= 4;
        }
        let compatible = (0..=1).all(|d| subsequences(&input, d) == subsequences(&target, d));
        let mut actual = [0; 4];
        byte_pass(&input, &mut actual, 0);
        assert_eq!(actual == target, compatible);
        fiber_size += usize::from(compatible);
    }
    assert_eq!(fiber_size, 6); // Binomial(4,2) shuffles.
    let reordered = [0x100, 0x300, 0x201, 0x001];
    let mut actual = [0; 4];
    byte_pass(&reordered, &mut actual, 0);
    assert_ne!(actual, target); // Equal histograms do not identify a pass fiber.
}

#[test]
fn stable_pass_composition_observes_low_byte_order() {
    let source = [0x0201, 0x0102, 0x0101, 0x0200, 0x10001, u64::MAX];
    let mut actual = source;
    for byte in 0..8 {
        let mut next = [0; 6];
        byte_pass(&actual, &mut next, byte);
        actual = next;
        let mask = u64::MAX >> (8 * (7 - byte));
        let mut expected = source;
        expected.sort_by_key(|value| value & mask);
        assert_eq!(actual, expected);
    }
    let mut wrong = source;
    for byte in (0..8).rev() {
        let mut next = [0; 6];
        byte_pass(&wrong, &mut next, byte);
        wrong = next;
    }
    assert_ne!(wrong, actual); // Reversing pass significance is not equivalent.
    let mut full = source;
    radix_sort(&mut full).unwrap();
    assert_eq!(full, actual);
}

#[test]
fn full_sort_fibers_and_bounded_monotone_maps() {
    let source = [257, 0, 255, 257];
    let mut canonical = source;
    radix_sort(&mut canonical).unwrap();
    for shift in 0..source.len() {
        let mut permuted = source;
        permuted.rotate_left(shift);
        radix_sort(&mut permuted).unwrap();
        assert_eq!(permuted, canonical);
    }
    let map = |x: u64| x.checked_mul(257).unwrap().checked_add(19).unwrap();
    let mut transformed = source.map(map);
    radix_sort(&mut transformed).unwrap();
    assert_eq!(transformed, canonical.map(map));
    let mut wrong = [0, 255, 255, 257];
    radix_sort(&mut wrong).unwrap();
    assert_ne!(wrong, canonical); // Same support, wrong multiplicities.
    let mut wrapped = [0, u64::MAX].map(|x| x.wrapping_add(1));
    let mapped_sorted = wrapped;
    radix_sort(&mut wrapped).unwrap();
    assert_ne!(wrapped, mapped_sorted); // Overflow breaks the order-preserving premise.
}
