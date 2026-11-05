use super::{SieveError, reserve, sieve};

fn trial_prime(value: usize) -> bool {
    value >= 2 && (2..value).all(|divisor| !value.is_multiple_of(divisor))
}

#[test]
fn inclusive_trial_divisor_oracle() {
    for limit in 0..=384 {
        let expected: Vec<_> = (0..=limit).filter(|&n| trial_prime(n)).collect();
        assert_eq!(sieve(limit).unwrap(), expected, "limit={limit}");
    }
}

#[test]
fn squares_boundaries_and_prefixes() {
    assert_eq!(sieve(2).unwrap(), [2]);
    assert_eq!(sieve(3).unwrap(), [2, 3]);
    assert_eq!(
        sieve(49).unwrap(),
        [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    );
    let primes = sieve(4096).unwrap();
    assert_eq!(primes.len(), 564);
    assert!(primes.windows(2).all(|pair| pair[0] < pair[1]));
    for limit in [0, 1, 2, 3, 4, 48, 49, 50, 997, 1024, 4095, 4096] {
        assert_eq!(
            sieve(limit).unwrap(),
            primes
                .iter()
                .copied()
                .take_while(|&p| p <= limit)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn impossible_sizes_rejected_without_allocating() {
    assert_eq!(sieve(usize::MAX), Err(SieveError::SizeOverflow));
    assert_eq!(sieve(isize::MAX as usize), Err(SieveError::SizeOverflow));
    let mut values = Vec::<usize>::new();
    assert_eq!(
        reserve(&mut values, usize::MAX),
        Err(SieveError::SizeOverflow)
    );
    assert_eq!(
        reserve(
            &mut values,
            isize::MAX as usize / std::mem::size_of::<usize>() + 1
        ),
        Err(SieveError::SizeOverflow)
    );
    assert_eq!(values.capacity(), 0);
    assert_eq!(
        SieveError::AllocationFailed.to_string(),
        "sieve storage allocation failed"
    );
}

#[test]
fn composite_witnesses_partition_the_prefix() {
    let limit = 1024;
    let primes = sieve(limit).unwrap();
    for value in [4_usize, 49, 77, 121, 143, 221, 361, 529, 899, 961, 1024] {
        let factor = (2..value).find(|d| value.is_multiple_of(*d)).unwrap();
        assert!(trial_prime(factor));
        assert!(factor <= value / factor);
        assert!(primes.contains(&factor));
        assert!(!primes.contains(&value));
        assert!(!sieve(value).unwrap().contains(&value));
    }
    for value in primes {
        assert!(trial_prime(value));
    }
}

#[test]
fn output_fibers_are_prime_gap_intervals() {
    for (last, next) in [(2, 3), (3, 5), (23, 29), (89, 97), (113, 127)] {
        assert!(trial_prime(last) && trial_prime(next));
        assert!((last + 1..next).all(|n| !trial_prime(n)));
        let prefix = sieve(last).unwrap();
        for limit in last..next {
            assert_eq!(sieve(limit).unwrap(), prefix);
        }
        assert_ne!(sieve(last - 1).unwrap(), prefix);
        assert_ne!(sieve(next).unwrap(), prefix);
        let mut false_prefix = prefix.clone();
        false_prefix.pop();
        assert_ne!(sieve(last).unwrap(), false_prefix);
    }
    assert_eq!(sieve(0), sieve(1));
    assert_ne!(sieve(1), sieve(2));
}

#[test]
fn typed_capacity_rejection_preserves_existing_values() {
    let mut values = vec![2_usize, 3, 5];
    let before = values.clone();
    let capacity = values.capacity();
    let count = isize::MAX as usize / std::mem::size_of::<usize>() + 1;
    assert_eq!(reserve(&mut values, count), Err(SieveError::SizeOverflow));
    assert_eq!(values, before);
    assert_eq!(values.capacity(), capacity);
    let mut flags = vec![1_u8];
    assert_eq!(
        reserve(&mut flags, isize::MAX as usize + 1),
        Err(SieveError::SizeOverflow)
    );
    assert_eq!(flags, [1]);
    assert_ne!(SieveError::SizeOverflow, SieveError::AllocationFailed);
}
