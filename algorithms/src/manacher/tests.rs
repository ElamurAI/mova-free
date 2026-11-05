use super::{PalindromeRadii, manacher};

fn oracle(input: &[u8]) -> PalindromeRadii {
    let n = input.len();
    let mut result = PalindromeRadii {
        odd: vec![0; n],
        even: vec![0; n],
    };
    // Enumerate intervals and compare both ends; no mirrored-radius recurrence.
    for start in 0..n {
        for end in start + 1..=n {
            let span = &input[start..end];
            if span.iter().eq(span.iter().rev()) {
                let length = end - start;
                let center = start + length / 2;
                if length % 2 == 0 {
                    result.even[center] = result.even[center].max(length / 2);
                } else {
                    result.odd[center] = result.odd[center].max(length / 2 + 1);
                }
            }
        }
    }
    result
}

#[test]
fn exhaustive_ternary_palindromes() {
    let mut count = 0;
    for n in 0..=8 {
        for mut code in 0..3_usize.pow(n) {
            let mut input = vec![0; n as usize];
            for byte in &mut input {
                *byte = (code % 3) as u8;
                code /= 3;
            }
            assert_eq!(manacher(&input), oracle(&input), "{input:?}");
            count += 1;
        }
    }
    assert_eq!(count, 9841);
}

#[test]
fn boundaries_and_utf8_bytes() {
    assert_eq!(
        manacher(b""),
        PalindromeRadii {
            odd: vec![],
            even: vec![]
        }
    );
    assert_eq!(
        manacher(b"a"),
        PalindromeRadii {
            odd: vec![1],
            even: vec![0]
        }
    );
    assert_eq!(
        manacher(b"abba"),
        PalindromeRadii {
            odd: vec![1; 4],
            even: vec![0, 0, 2, 0]
        }
    );
    assert_eq!(
        manacher(b"ababa"),
        PalindromeRadii {
            odd: vec![1, 2, 3, 2, 1],
            even: vec![0; 5]
        }
    );
    assert_eq!(
        manacher("éé".as_bytes()),
        PalindromeRadii {
            odd: vec![1, 2, 2, 1],
            even: vec![0; 4]
        }
    );
    assert_eq!(manacher(&[255, 0, 255]), oracle(&[255, 0, 255]));
}

#[test]
fn relabeling_and_long_palindrome() {
    let input = b"abacabbacaba";
    let renamed: Vec<_> = input.iter().map(|x| 255 - x).collect();
    assert_eq!(manacher(input), manacher(&renamed));
    let n = 100_000;
    let result = manacher(&vec![7; n]);
    for i in 0..n {
        assert_eq!(result.odd[i], (i + 1).min(n - i));
        assert_eq!(result.even[i], i.min(n - i));
    }
}

fn reconstruct(radii: &PalindromeRadii) -> Option<Vec<u8>> {
    let n = radii.odd.len();
    if n > 256 || radii.even.len() != n {
        return None;
    }
    let mut classes: Vec<usize> = (0..n).collect();
    let mut unequal = Vec::new();
    for i in 0..n {
        let (odd, even) = (radii.odd[i], radii.even[i]);
        if odd == 0 || odd > (i + 1).min(n - i) || even > i.min(n - i) {
            return None;
        }
        let pairs = (1..odd)
            .map(|k| (i - k, i + k))
            .chain((0..even).map(|k| (i - 1 - k, i + k)));
        for (a, b) in pairs {
            let (from, to) = (classes[b], classes[a]);
            for class in &mut classes {
                if *class == from {
                    *class = to;
                }
            }
        }
        if odd <= i && odd < n - i {
            unequal.push((i - odd, i + odd));
        }
        if even < i && even < n - i {
            unequal.push((i - 1 - even, i + even));
        }
    }
    if unequal.iter().any(|&(a, b)| classes[a] == classes[b]) {
        return None;
    }
    Some(classes.into_iter().map(|c| c as u8).collect())
}

#[test]
fn joint_parity_constraints_reconstruct_and_reject() {
    for input in [b"".as_slice(), b"abba", b"abacaba", b"aabb", b"abc"] {
        let r = manacher(input);
        assert_eq!(manacher(&reconstruct(&r).unwrap()), r);
    }
    // Even centers force a=b=c while the middle odd radius forbids a=c.
    assert!(
        reconstruct(&PalindromeRadii {
            odd: vec![1; 3],
            even: vec![0, 1, 1]
        })
        .is_none()
    );
    assert!(
        reconstruct(&PalindromeRadii {
            odd: vec![0],
            even: vec![0]
        })
        .is_none()
    );
    assert!(
        reconstruct(&PalindromeRadii {
            odd: vec![1],
            even: vec![]
        })
        .is_none()
    );
}

#[test]
fn reversal_centers_and_injective_symbols() {
    for input in [b"".as_slice(), b"a", b"aab", b"abba", b"abacabb"] {
        let n = input.len();
        let original = manacher(input);
        let reversed: Vec<_> = input.iter().copied().rev().collect();
        let r = manacher(&reversed);
        for i in 0..n {
            assert_eq!(r.odd[i], original.odd[n - 1 - i]);
            assert_eq!(r.even[i], if i == 0 { 0 } else { original.even[n - i] });
        }
        let renamed: Vec<_> = input.iter().map(|&c| c ^ 0xa5).collect();
        assert_eq!(manacher(&renamed), original);
    }
    assert_ne!(manacher(b"ab"), manacher(b"aa"));
}

#[test]
fn interval_membership_count_and_lossy_projection() {
    for input in [b"".as_slice(), b"abacaba", b"abba", b"aaaa", b"abc"] {
        let r = manacher(input);
        let mut count = 0;
        for start in 0..input.len() {
            for end in start + 1..=input.len() {
                let length = end - start;
                let center = start + length / 2;
                let predicted = if length % 2 == 0 {
                    r.even[center] >= length / 2
                } else {
                    r.odd[center] > length / 2
                };
                let actual = input[start..end].iter().eq(input[start..end].iter().rev());
                assert_eq!(predicted, actual);
                count += usize::from(actual);
            }
        }
        assert_eq!(count, r.odd.iter().chain(&r.even).sum::<usize>());
    }
    let (a, b) = (manacher(b"aab"), manacher(b"abb"));
    assert_ne!(a, b);
    assert_eq!(
        a.odd.iter().chain(&a.even).sum::<usize>(),
        b.odd.iter().chain(&b.even).sum::<usize>()
    );
}
