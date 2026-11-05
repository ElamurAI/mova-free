use super::z_algorithm;

fn oracle(input: &[u8]) -> Vec<usize> {
    (0..input.len())
        .map(|i| {
            input
                .iter()
                .zip(&input[i..])
                .take_while(|(a, b)| a == b)
                .count()
        })
        .collect()
}

#[test]
fn exhaustive_ternary_prefixes() {
    let mut count = 0;
    for n in 0..=8 {
        for mut code in 0..3_usize.pow(n) {
            let mut input = vec![0; n as usize];
            for byte in &mut input {
                *byte = (code % 3) as u8;
                code /= 3;
            }
            assert_eq!(z_algorithm(&input), oracle(&input), "{input:?}");
            count += 1;
        }
    }
    assert_eq!(count, 9841);
}

#[test]
fn boundaries_and_utf8_bytes() {
    assert_eq!(z_algorithm(b""), Vec::<usize>::new());
    assert_eq!(z_algorithm(b"a"), vec![1]);
    assert_eq!(z_algorithm(b"aaaaa"), vec![5, 4, 3, 2, 1]);
    assert_eq!(z_algorithm(b"ababa"), vec![5, 0, 3, 0, 1]);
    assert_eq!(z_algorithm("éé".as_bytes()), vec![4, 0, 2, 0]);
    assert_eq!(z_algorithm(&[255, 0, 255, 0]), vec![4, 0, 2, 0]);
}

#[test]
fn relabeling_and_long_border() {
    let input = b"abcababcababc";
    let renamed: Vec<_> = input.iter().map(|x| 255 - x).collect();
    assert_eq!(z_algorithm(input), z_algorithm(&renamed));
    let input = vec![7; 100_000];
    let z = z_algorithm(&input);
    assert!(z.iter().enumerate().all(|(i, &r)| r == input.len() - i));
}

// Distinct equality classes give a byte witness for these bounded candidates.
fn reconstruct(z: &[usize]) -> Option<Vec<u8>> {
    let n = z.len();
    if n > 256 || (n > 0 && z[0] != n) {
        return None;
    }
    let mut classes: Vec<usize> = (0..n).collect();
    for (i, &radius) in z.iter().enumerate() {
        if radius > n - i {
            return None;
        }
        for j in 0..radius {
            let (from, to) = (classes[i + j], classes[j]);
            for class in &mut classes {
                if *class == from {
                    *class = to;
                }
            }
        }
    }
    for (i, &radius) in z.iter().enumerate() {
        if i + radius < n && classes[radius] == classes[i + radius] {
            return None;
        }
    }
    Some(classes.into_iter().map(|c| c as u8).collect())
}

#[test]
fn equality_constraints_reconstruct_and_reject() {
    for input in [b"".as_slice(), b"abacaba", b"aaaa", b"abcab", b"ababa"] {
        let z = z_algorithm(input);
        let witness = reconstruct(&z).unwrap();
        assert_eq!(z_algorithm(&witness), z);
    }
    // Individually legal lengths, but z[1]=2 forces all bytes equal.
    assert!(reconstruct(&[3, 2, 0]).is_none());
    assert!(reconstruct(&[3, 3, 0]).is_none());
    assert!(reconstruct(&[0]).is_none());
    assert_eq!(z_algorithm(b"abb"), z_algorithm(b"abc"));
    assert_ne!(b"abb", b"abc"); // Equality partitions are not recoverable.
}

#[test]
fn borders_periods_and_prefix_occurrences() {
    for input in [b"a".as_slice(), b"ababa", b"abcabc", b"aaaaa", b"abcde"] {
        let z = z_algorithm(input);
        let n = input.len();
        for p in 1..=n {
            let period = input[p..] == input[..n - p];
            assert_eq!(period, p == n || z[p] == n - p);
            if p < n {
                let border = n - p;
                assert_eq!(period, input[..border] == input[n - border..]);
            }
        }
        for k in 1..=n {
            let positions: Vec<_> = (0..n).filter(|&i| z[i] >= k).collect();
            let direct: Vec<_> = input
                .windows(k)
                .enumerate()
                .filter_map(|(i, w)| (w == &input[..k]).then_some(i))
                .collect();
            assert_eq!(positions, direct);
        }
    }
    // A period need not divide length (ababa has period two).
    assert_eq!(z_algorithm(b"ababa")[2], 3);
}

#[test]
fn compressed_observation_and_collapsing_symbols() {
    let a = z_algorithm(b"abca");
    let b = z_algorithm(b"aabc");
    assert_ne!(a, b);
    assert_eq!(a.iter().sum::<usize>(), b.iter().sum::<usize>());
    assert_ne!(z_algorithm(b"ab"), z_algorithm(b"aa"));
    assert_ne!(z_algorithm(b"aab"), z_algorithm(b"baa"));
}
